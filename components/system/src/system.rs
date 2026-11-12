// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Stufe 4 und 5: Ausfuehrung unter Freigabe, danach Blockproduktion.
//!
//! Der Ablauf einer Ausfuehrung ist bindend und nicht umstellbar:
//!  1. Freigabe pruefen (ATC-SEC-EXEC-001, Signatur gegen den Schluessel)
//!  2. Artefakt aus den **Bytes** lesen (nicht aus dem Speicherobjekt)
//!  3. Zustand der Kette in die VM geben, Generation aus dem Artefakt
//!  4. Ausfuehren mit dem Gasbudget der Kette (transaktional)
//!  5. Zustandswurzel bilden und als Block an die Kette haengen
//!
//! Faellt Schritt 1 bis 4 aus, entsteht **kein** Block und **keine**
//! Zustandsaenderung. Es gibt keinen Pfad, der ohne Freigabe bucht.
//!
//! **Eingangsvertrag (D-0):** von aussen wird ausschliesslich die
//! Artefakt-Huelle `ATCE`/`0x02` angenommen ([`System::deploy_container`]). Die
//! Huelle wird **vor** jeder Uebersetzung, Signatur und Ausfuehrung geprueft;
//! `ATCB`/`0x01` bleibt das innere Ausfuehrungsformat und ist kein externer
//! Eingang. Es gibt keinen Rueckfall: ein `ATCB`-Strom an der Huelle ergibt
//! `E-007`, eine leere Huelle `E-001`. Die Ausfuehrung liest den Strom immer
//! ueber die Huelle und vergleicht den Bytecode-Hash mit der Freigabe.

use atc_exec_admission::{
    AdmCode, Authority, Component, ExecutionPolicy, IsolationState, Registry, Request, Revocation,
    Scope,
};
use atc_node::bootstrap::Genesis;
use atc_node::chain::Chain;
use atc_vm::artifact::{Artifact, ArtifactError, GENERATION_V1, HASH_LEN};
use atc_vm::context::ChainContext;
use atc_vm::envelope::{Envelope, EnvelopeError};
use atc_vm::vm::{Op, Vm, VmError};

use crate::backend::{self, BackendError};
use crate::compile::{self, CompileError};
use crate::seal::{AuthorizedKeyVerifier, SealError, SealKey};
use crate::state::ContractState;

/// Schluessel der Devnet-Freigabeinstanz. Bewusst deterministisch: ein
/// reproduzierbarer Durchlauf ist mehr wert als ein zufaelliger.
pub const DEVNET_SEAL_SEED: [u8; 32] = [0x42; 32];

/// Rechte, die eine Vertragsausfuehrung anfordert: lesen und schreiben.
pub const CONTRACT_RIGHTS: u16 = 0b11;

/// Standard-Gasbudget einer Ausfuehrung.
pub const DEFAULT_GAS: u64 = 50_000;

/// Gueltigkeit einer Freigabe: 10 Jahre ab Freigabezeitpunkt.
pub const POLICY_TTL_US: i64 = 10 * 365 * 24 * 60 * 60 * 1_000_000;

/// Version der Freigabe.
pub const POLICY_VERSION: u64 = 1;

/// Warum eine Stufe des Systems abgelehnt hat.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemError {
    Compile(CompileError),
    Backend(BackendError),
    Artifact(ArtifactError),
    /// Die aeussere Artefakt-Huelle `ATCE` wurde abgelehnt; die Identitaet ist
    /// das Paar `(Code, Offset)` aus der geschlossenen Liste `E-001` bis `E-012`.
    Container(EnvelopeError),
    /// Die Block-Payload wurde abgelehnt; Identitaet `P-001` bis `P-003`.
    BlockPayload(crate::block_payload::BlockPayloadError),
    Seal(SealError),
    Admission {
        code: Option<&'static str>,
        detail: String,
    },
    Vm(VmError),
    Block(String),
    Genesis(String),
}

impl SystemError {
    pub fn stage(&self) -> &'static str {
        match self {
            SystemError::Compile(_) => "compile",
            SystemError::Backend(_) => "backend",
            SystemError::Artifact(_) => "artifact",
            SystemError::Container(_) => "container",
            SystemError::BlockPayload(_) => "block_payload",
            SystemError::Seal(_) => "seal",
            SystemError::Admission { .. } => "admission",
            SystemError::Vm(_) => "vm",
            SystemError::Block(_) => "block",
            SystemError::Genesis(_) => "genesis",
        }
    }
}

impl std::fmt::Display for SystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SystemError::Compile(e) => write!(f, "Kompilierung: {}", e.message()),
            SystemError::Backend(e) => write!(f, "Uebersetzung: {}", e.message()),
            SystemError::Artifact(e) => write!(f, "Artefakt: {e:?}"),
            SystemError::Container(e) => {
                write!(f, "Artefakthuelle: {} bei {}", e.code(), e.offset())
            }
            SystemError::BlockPayload(e) => write!(f, "Block-Payload: {e}"),
            SystemError::Seal(e) => write!(f, "Freigabeschluessel: {e:?}"),
            SystemError::Admission { code, detail } => match code {
                Some(code) => write!(f, "Freigabe abgelehnt ({code}): {detail}"),
                None => write!(f, "Freigabe abgelehnt: {detail}"),
            },
            SystemError::Vm(e) => write!(f, "VM: {e:?}"),
            SystemError::Block(e) => write!(f, "Block: {e}"),
            SystemError::Genesis(e) => write!(f, "Genesis: {e}"),
        }
    }
}

/// Ein freigegebenes Artefakt, wie es die Kette kennt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deployment {
    pub code_hash: [u8; 32],
    /// Inneres Ausfuehrungsformat `ATCB`/`0x01` (Bytecode plus Bytecode-Hash).
    pub artifact_bytes: Vec<u8>,
    /// Aeusseres Artefaktformat `ATCE`/`0x02`, wie es von aussen angenommen und
    /// von der Kette gefuehrt wird. Traegt denselben Strom wie `artifact_bytes`;
    /// beide werden bei der Freigabe gegeneinander geprueft.
    pub container_bytes: Vec<u8>,
    pub policy: ExecutionPolicy,
    pub signature: [u8; 64],
    /// Anzahl der ATCB-Operationen im Artefakt.
    pub ops: usize,
    /// Anzahl der ATCLang-Instruktionen in der Quelle.
    pub source_instructions: usize,
    /// Offene Semantik-Unterschiede, die fuer dieses Artefakt gelten.
    pub divergences: &'static [&'static str],
}

impl Deployment {
    pub fn code_hash_hex(&self) -> String {
        hex(&self.code_hash)
    }

    /// `artifact_hash` der Huelle (eigener Digest, getrennt vom Bytecode-Hash).
    pub fn container_hash_hex(&self) -> String {
        match Envelope::parse(&self.container_bytes) {
            Ok(env) => hex(&env.artifact_hash()),
            Err(_) => "ungueltig".to_string(),
        }
    }

    /// Hash genau der Bytes, die ausgefuehrt werden (aus dem Container gelesen).
    pub fn artifact_hash_hex(&self) -> String {
        match Artifact::parse(&self.artifact_bytes) {
            Ok(artifact) => hex(&artifact.hash()),
            Err(_) => "ungueltig".to_string(),
        }
    }
}

/// Beleg einer Ausfuehrung.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Receipt {
    pub code_hash: [u8; 32],
    pub state_root_before: [u8; 32],
    pub state_root_after: [u8; 32],
    /// Stapel nach der Ausfuehrung; der letzte Wert ist das Ergebnis.
    pub returned: Vec<u64>,
    pub gas_limit: u64,
    pub block_height: u64,
    pub block_hash: [u8; 32],
    pub audit_entries: usize,
}

impl Receipt {
    pub fn result(&self) -> Option<u64> {
        self.returned.last().copied()
    }

    pub fn changed_state(&self) -> bool {
        self.state_root_before != self.state_root_after
    }
}

/// Das zusammengesetzte System: Genesis, Kette, Freigaberegister, Authority und
/// Signaturpruefer in einem Objekt.
pub struct System {
    genesis: Genesis,
    chain: Chain,
    registry: Registry,
    authority: Authority,
    verifier: AuthorizedKeyVerifier,
    seal: SealKey,
}

impl System {
    /// Devnet-System mit deterministischem Freigabeschluessel.
    pub fn devnet() -> Result<Self, SystemError> {
        let genesis = Genesis::devnet();
        genesis.validate().map_err(SystemError::Genesis)?;
        let seal = SealKey::from_seed(DEVNET_SEAL_SEED).map_err(SystemError::Seal)?;
        let verifier = AuthorizedKeyVerifier::new(seal.public_key()).map_err(SystemError::Seal)?;

        let mut authority = Authority::new(IsolationState::Available);
        authority.authorize_signer(seal.address());
        authority.set_ceiling(Component::AtcVm, CONTRACT_RIGHTS);

        Ok(System {
            chain: Chain::from_genesis(&genesis),
            genesis,
            registry: Registry::new(),
            authority,
            verifier,
            seal,
        })
    }

    pub fn genesis(&self) -> &Genesis {
        &self.genesis
    }

    pub fn chain(&self) -> &Chain {
        &self.chain
    }

    pub fn seal_address(&self) -> String {
        self.seal.address()
    }

    pub fn audit_count(&self) -> usize {
        self.authority.log().len()
    }

    /// Betreiberpfad: Widerruf, Obergrenzen, Freigaben. Die Authority ist ein
    /// verwaltetes Objekt, kein Konstantenblock.
    pub fn authority_mut(&mut self) -> &mut Authority {
        &mut self.authority
    }

    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    pub fn verifier(&self) -> &AuthorizedKeyVerifier {
        &self.verifier
    }

    pub fn seal(&self) -> &SealKey {
        &self.seal
    }

    /// Stufe 1 bis 3: Quelle kompilieren, in ein Artefakt uebersetzen, Freigabe
    /// signieren und registrieren.
    pub fn deploy(&mut self, source: &str, now_us: i64) -> Result<Deployment, SystemError> {
        let compiled = compile::compile(source).map_err(SystemError::Compile)?;
        let translation = backend::translate(&compiled).map_err(SystemError::Backend)?;

        let artifact =
            Artifact::new(GENERATION_V1, translation.ops).map_err(SystemError::Artifact)?;
        let artifact_bytes = artifact.to_bytes();
        let code_hash = artifact.hash();

        // Selbstpruefung: die Bytes muessen dasselbe Artefakt ergeben. Ein
        // Deployment, dessen eigener Umlauf nicht stimmt, geht nicht weiter.
        let reparsed = Artifact::parse(&artifact_bytes).map_err(SystemError::Artifact)?;
        if reparsed.hash() != code_hash {
            return Err(SystemError::Artifact(ArtifactError::BadHash {
                offset: artifact_bytes.len() - HASH_LEN,
            }));
        }

        // Aeussere Huelle bauen und denselben Umlauf verlangen: Huelle -> ATCB
        // muss wieder genau diesen Bytecode-Hash ergeben. Sonst gaebe es ein
        // Deployment, dessen Huelle eine andere Ausfuehrung benennt.
        let container_bytes = Envelope::from_artifact(&artifact).to_bytes();
        let aus_huelle = Envelope::parse(&container_bytes)
            .map_err(SystemError::Container)?
            .into_artifact()
            .map_err(SystemError::Container)?;
        if aus_huelle.hash() != code_hash {
            return Err(SystemError::Admission {
                code: None,
                detail: "Huelle und Bytecode nennen verschiedene Hashes".to_string(),
            });
        }

        let policy = ExecutionPolicy {
            code_hash,
            signer_identity: self.seal.address(),
            source_provenance: Some(format!("atclang:{}ops", translation.source_len)),
            generation_proof: None,
            component_identity: Component::AtcVm,
            version: POLICY_VERSION,
            requested_capabilities: CONTRACT_RIGHTS,
            execution_scope: Scope::Contract,
            expiration_us: now_us.saturating_add(POLICY_TTL_US),
            revocation_status: Revocation::Active,
        };
        let signature = self.seal.sign_policy(&policy);
        if let Err(konflikt) = self.registry.register(policy.clone()) {
            return Err(SystemError::Admission {
                code: None,
                detail: format!(
                    "Policy-Konflikt, vorhandene Freigabe bleibt: {konflikt} (Code ADM-012 vorgesehen, bis zur normativen Registrierung ohne Fehleridentitaet)"
                ),
            });
        }

        Ok(Deployment {
            code_hash,
            artifact_bytes,
            container_bytes,
            policy,
            signature,
            ops: artifact.program().len(),
            source_instructions: translation.source_len,
            divergences: backend::DIVERGENZEN,
        })
    }

    /// Externer Eingang: ein Artefakt als `ATCE`/`0x02`-Huelle annehmen.
    ///
    /// Reihenfolge ist bindend und nicht umstellbar:
    ///  1. Huelle pruefen (Laenge, Magic, Fassung, Generation, Strom, Tail)
    ///  2. Strom an das bestehende `ATCB`-Artefakt uebergeben (`into_artifact`)
    ///  3. Freigabe bilden, signieren, registrieren
    ///
    /// Zwischen 1 und 2 liegt **keine** Uebersetzung: die Huelle wird geprueft,
    /// bevor Hash-, Dekodier- oder Ausfuehrungsarbeit der VM anfaellt. Ein
    /// `ATCB`-Strom wird hier mit `E-007` abgewiesen; es gibt keinen Rueckfall
    /// auf das innere Format.
    ///
    /// Die Quelle ist bei diesem Weg unbekannt: `source_instructions` bleibt 0
    /// und es gelten keine Uebersetzungs-Unterschiede.
    pub fn deploy_container(
        &mut self,
        container: &[u8],
        now_us: i64,
    ) -> Result<Deployment, SystemError> {
        // 1 — Huelle. Fehleridentitaet ist (Code, Offset).
        let envelope = Envelope::parse(container).map_err(SystemError::Container)?;
        let container_hash = envelope.artifact_hash();

        // 2 — Uebergabe an das bestehende Artefaktmodell, kein zweiter Parser.
        let artifact = envelope.into_artifact().map_err(SystemError::Container)?;
        let artifact_bytes = artifact.to_bytes();
        let code_hash = artifact.hash();

        let policy = ExecutionPolicy {
            code_hash,
            signer_identity: self.seal.address(),
            source_provenance: Some(format!("atce:{}ops", artifact.program().len())),
            generation_proof: None,
            component_identity: Component::AtcVm,
            version: POLICY_VERSION,
            requested_capabilities: CONTRACT_RIGHTS,
            execution_scope: Scope::Contract,
            expiration_us: now_us.saturating_add(POLICY_TTL_US),
            revocation_status: Revocation::Active,
        };
        let signature = self.seal.sign_policy(&policy);
        if let Err(konflikt) = self.registry.register(policy.clone()) {
            return Err(SystemError::Admission {
                code: None,
                detail: format!(
                    "Policy-Konflikt, vorhandene Freigabe bleibt: {konflikt} (Code ADM-012 vorgesehen, bis zur normativen Registrierung ohne Fehleridentitaet)"
                ),
            });
        }

        let _ = container_hash;
        Ok(Deployment {
            code_hash,
            artifact_bytes,
            container_bytes: container.to_vec(),
            policy,
            signature,
            ops: artifact.program().len(),
            source_instructions: 0,
            divergences: &[],
        })
    }

    /// Nur pruefen, ohne Deployment: gibt die Fehleridentitaet der Huelle zurueck.
    pub fn check_container(&self, container: &[u8]) -> Result<(), SystemError> {
        Envelope::parse(container)
            .map(|_| ())
            .map_err(SystemError::Container)
    }

    /// Stufe 4 und 5: Freigabe pruefen, ausfuehren, Block buchen.
    /// `state` wird nur bei Erfolg fortgeschrieben.
    pub fn execute(
        &mut self,
        deployment: &Deployment,
        state: &mut ContractState,
        gas_limit: u64,
        now_us: i64,
        caller: u64,
    ) -> Result<Receipt, SystemError> {
        let state_root_before = state.state_root(&deployment.code_hash);

        // 0 — Die aeussere Huelle lesen, dann den Strom an das Artefakt geben.
        // Die Huelle prueft Struktur vor Digest und meldet (Code, Offset); eine
        // leere oder fehlende Huelle ergibt E-001 und damit keinen Pfad an der
        // Huelle vorbei.
        let artifact = Envelope::parse(&deployment.container_bytes)
            .map_err(SystemError::Container)?
            .into_artifact()
            .map_err(SystemError::Container)?;
        let observed_hash = artifact.hash();

        // 0b — Huelle und Freigabe muessen dasselbe Programm benennen.
        if observed_hash != deployment.code_hash {
            return Err(SystemError::Admission {
                code: None,
                detail: "Huelle und Freigabe nennen verschiedene Bytecode-Hashes".to_string(),
            });
        }

        // 0c — Es gibt genau eine Wahrheit: die Huelle. Die inneren Bytes sind
        // eine Ableitung davon und duerfen nicht auseinanderlaufen; sonst haette
        // das System zwei Aussagen ueber dasselbe Artefakt.
        if artifact.to_bytes() != deployment.artifact_bytes {
            return Err(SystemError::Admission {
                code: None,
                detail: "innere Artefaktbytes weichen von der Huelle ab".to_string(),
            });
        }

        // 1 — Freigabe. Die Signatur wird gegen den Schluessel geprueft, nicht
        // gegen "vorhanden oder nicht".
        let policy_bytes = deployment.policy.canonical_bytes();
        let request = Request {
            code_hash: deployment.code_hash,
            observed_hash,
            policy_bytes,
            component: Component::AtcVm,
            requested_capabilities: CONTRACT_RIGHTS,
            signature: Some(&deployment.signature),
            now_us,
        };
        let decision = self
            .authority
            .admit(&self.registry, &request, &self.verifier);
        if !decision.is_authorized() {
            return Err(SystemError::Admission {
                code: decision.code.map(AdmCode::code),
                detail: decision.detail,
            });
        }

        // 3 — Zustand kommt von der Kette, Generation nur aus dem Artefakt.
        let mut vm = Vm::from_artifact_with_context(&artifact, caller, state.words().to_vec());

        // 4 — Ausfuehrung mit dem Budget der Kette, transaktional.
        let context = ChainContext {
            chain_id: self.genesis.chain_id.clone(),
            network_id: self.genesis.network_id.clone(),
            genesis_id: self.genesis.genesis_id.clone(),
            protocol_version: self.genesis.protocol_version.clone(),
            vm_version: self.genesis.vm_version.clone(),
        };
        let returned = vm
            .execute_state_transition_with_gas(
                &context,
                &self.genesis.genesis_id,
                &self.genesis.protocol_version,
                &self.genesis.vm_version,
                gas_limit,
            )
            .map_err(SystemError::Vm)?;

        let after = ContractState::from_words(vm.state().to_vec());
        let state_root_after = after.state_root(&deployment.code_hash);

        // 5 — Block. Payload ohne '|' und ';' (Wire-Format der Kette).
        let payload = format!(
            "exec:{}:{}",
            hex(&deployment.code_hash[..8]),
            hex(&state_root_after)
        );
        let block_hash = self.chain.produce(&payload).map_err(SystemError::Block)?;

        *state = after;

        Ok(Receipt {
            code_hash: deployment.code_hash,
            state_root_before,
            state_root_after,
            returned,
            gas_limit,
            block_height: self.chain.height(),
            block_hash,
            audit_entries: self.authority.log().len(),
        })
    }
}

/// Kleine Hex-Hilfe, damit Belege lesbar bleiben.
pub fn hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}

/// Nur fuer die CLI: eine feste Demo-Quelle, damit `atc-system demo` ohne Datei
/// laeuft.
pub const DEMO_SOURCE: &str = "fn main() -> i64 { let i = 0; let s = 0; while i < 5 { let s = s + i; let i = i + 1; } return s; }";

/// Bequemer Durchlauf: deployen und einmal ausfuehren.
pub fn deploy_and_run(
    system: &mut System,
    source: &str,
    state: &mut ContractState,
    gas_limit: u64,
    now_us: i64,
) -> Result<(Deployment, Receipt), SystemError> {
    let deployment = system.deploy(source, now_us)?;
    let receipt = system.execute(&deployment, state, gas_limit, now_us, 1)?;
    Ok((deployment, receipt))
}

/// Operationen eines Artefakts, nur fuer Diagnose.
pub fn ops_of(deployment: &Deployment) -> Result<Vec<Op>, SystemError> {
    let artifact = Artifact::parse(&deployment.artifact_bytes).map_err(SystemError::Artifact)?;
    Ok(artifact.program().to_vec())
}
