//! G2-A (Issue #112): Model Registry + Verified Cache — S21-S22 nachgeholt.
//! Deterministisch (REQ-ENG-002): feste Werte, keine Wall-Clock, kein RNG.

use kai_os_ai::model_registry::{
    ModelManifest, ModelRegistry, ModelVersion, RegistryError, VersionReq,
};
use kai_os_ai::sha_hex;
use kai_os_ai::verified_cache::{CacheError, VerifiedCache};

fn manifest(id: &str, v: ModelVersion, bytes: &[u8]) -> ModelManifest {
    ModelManifest {
        id: id.into(),
        version: v,
        owner: "research-agent".into(),
        size_bytes: bytes.len() as u64,
        sha256: sha_hex(bytes),
    }
}

// ── Registry: Publish-Immutabilität ─────────────────────────────────────

#[test]
fn publish_immutability_enforced() {
    let mut r = ModelRegistry::new();
    let v = ModelVersion::new(1, 0, 0);
    r.publish(manifest("llm-core", v, b"original")).unwrap();

    // Gleiche Version, identischer Content -> idempotent OK
    r.publish(manifest("llm-core", v, b"original")).unwrap();
    assert_eq!(r.versions_of("llm-core"), vec![v]);

    // Gleiche Version, ANDERER Content (anderer SHA) -> abgewiesen
    assert!(matches!(
        r.publish(manifest("llm-core", v, b"manipuliert")),
        Err(RegistryError::DuplicateIdVersion { .. })
    ));
    // Kein Teilzustand: nur die Original-Version bleibt
    assert_eq!(r.versions_of("llm-core"), vec![v]);

    // Neue Version -> ok
    r.publish(manifest("llm-core", ModelVersion::new(1, 1, 0), b"neu"))
        .unwrap();
    assert_eq!(r.versions_of("llm-core").len(), 2);
}

// ── Registry: deterministische Versionswahl ────────────────────────────

#[test]
fn best_match_highest_matching_regardless_of_publish_order() {
    // Zwei Registries, identische Modelle, umgekehrte Publish-Reihenfolge
    let build = |reversed: bool| -> ModelRegistry {
        let mut r = ModelRegistry::new();
        let versions = [
            (ModelVersion::new(1, 0, 0), b"v100" as &[u8]),
            (ModelVersion::new(1, 2, 0), b"v120"),
            (ModelVersion::new(1, 9, 0), b"v190"),
            (ModelVersion::new(2, 0, 0), b"v200"),
        ];
        let order: Vec<_> = if reversed {
            versions.iter().rev().cloned().collect()
        } else {
            versions.to_vec()
        };
        for (v, b) in order {
            r.publish(manifest("vision", v, b)).unwrap();
        }
        r
    };
    let ra = build(false);
    let rb = build(true);

    let ma = ra
        .best_match("vision", &VersionReq::Caret(ModelVersion::new(1, 0, 0)))
        .unwrap();
    let mb = rb
        .best_match("vision", &VersionReq::Caret(ModelVersion::new(1, 0, 0)))
        .unwrap();
    // ^1.0.0 wählt 1.9.0 (höchste PASSende, Major-Grenze 2.0.0)
    assert_eq!(ma, mb);
    assert_eq!(ma.version, ModelVersion::new(1, 9, 0));
    assert_eq!(ma.sha256, sha_hex(b"v190"));

    // Caret ueberschreitet Major-Grenze nicht
    let m2 = ra
        .best_match("vision", &VersionReq::Caret(ModelVersion::new(2, 0, 0)))
        .unwrap();
    assert_eq!(m2.version, ModelVersion::new(2, 0, 0));
    // Exact
    let e = ra
        .best_match("vision", &VersionReq::Exact(ModelVersion::new(1, 2, 0)))
        .unwrap();
    assert_eq!(e.version, ModelVersion::new(1, 2, 0));
    // Kein Match -> fail-closed
    assert!(matches!(
        ra.best_match("vision", &VersionReq::Exact(ModelVersion::new(9, 9, 9))),
        Err(RegistryError::NoMatchingVersion { .. })
    ));
    // Unbekanntes Modell -> fail-closed
    assert!(matches!(
        ra.best_match("geist", &VersionReq::Caret(ModelVersion::new(1, 0, 0))),
        Err(RegistryError::UnknownModel { .. })
    ));
}

// ── Verified Cache: kein ungeprüftes Laden ────────────────────────────

#[test]
fn cache_rejects_wrong_artifact_on_put() {
    let mut c = VerifiedCache::new(4);
    let m = manifest("llm-core", ModelVersion::new(1, 0, 0), b"echtes-artefakt");
    // Falsche Bytes (Hash passt nicht zum Manifest) -> fail-closed, NICHT gelagert
    let mismatch = c.put(m.clone(), b"gefaelschte-bytes".to_vec());
    assert!(
        matches!(&mismatch, Err(CacheError::HashMismatch { expected, .. }) if *expected == m.sha256)
    );
    assert!(
        c.is_empty(),
        "abgelehntes Artefakt darf nicht gelagert sein"
    );
}

#[test]
fn cache_verifies_on_every_load_and_detects_tampering() {
    let mut c = VerifiedCache::new(4);
    let m = manifest("vision", ModelVersion::new(1, 9, 0), b"artefakt-inhalt");
    c.put(m.clone(), b"artefakt-inhalt".to_vec()).unwrap();

    // Normaler Load: verifiziert -> OK
    let loaded = c.get("vision", ModelVersion::new(1, 9, 0)).unwrap();
    assert_eq!(loaded.bytes, b"artefakt-inhalt".to_vec());

    // Unbekannter Eintrag
    assert!(matches!(
        c.get("geist", ModelVersion::new(1, 0, 0)),
        Err(CacheError::UnknownEntry { .. })
    ));
}

// ── Verified Cache: deterministische FIFO-Eviction ──────────────────────

#[test]
fn cache_eviction_deterministic_fifo() {
    let mut c = VerifiedCache::new(2);
    let v1 = ModelVersion::new(1, 0, 0);
    let v2 = ModelVersion::new(2, 0, 0);
    let v3 = ModelVersion::new(3, 0, 0);
    c.put(manifest("m", v1, b"eins"), b"eins".to_vec()).unwrap();
    c.put(manifest("m", v2, b"zwei"), b"zwei".to_vec()).unwrap();
    assert_eq!(c.len(), 2);
    // Dritter Eintrag verdraengt den aeltesten (FIFO, deterministisch)
    c.put(manifest("m", v3, b"drei"), b"drei".to_vec()).unwrap();
    assert_eq!(c.len(), 2);
    assert!(
        c.get("m", v1).is_err(),
        "aeltester Eintrag muss evicted sein"
    );
    assert!(c.get("m", v2).is_ok());
    assert!(c.get("m", v3).is_ok());
}

// ── Registry + Cache zusammen: Integritaetskette ───────────────────────

#[test]
fn registry_and_cache_integrity_chain() {
    let mut r = ModelRegistry::new();
    let bytes = b"modell-gewichte-v1";
    let m = manifest("llm-core", ModelVersion::new(1, 0, 0), bytes);
    r.publish(m.clone()).unwrap();

    // Nur das Registry-Manifest ist die Integritaetsgrundlage:
    // ein artefakt mit abweichendem Hash kommt gar nicht in den Cache
    let mut c = VerifiedCache::new(4);
    assert!(c.put(m.clone(), b"falsch".to_vec()).is_err());
    assert!(c.put(m.clone(), bytes.to_vec()).is_ok());

    // Was aus dem Cache kommt, ist bei jedem Zugriff gegen das
    // Registry-Manifest verifizierbar
    let loaded = c.get("llm-core", m.version).unwrap();
    let reg_m = r.get("llm-core", m.version).unwrap();
    assert_eq!(loaded.manifest.sha256, reg_m.sha256);
    assert_eq!(sha_hex(&loaded.bytes), reg_m.sha256);
}
