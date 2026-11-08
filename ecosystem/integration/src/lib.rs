//! End-to-end integration boundary for the A-TownChain ecosystem.
//! This crate owns no consensus logic; it composes the canonical node runtime and SDK.

use atc_node::runtime::{Runtime, NUMERIC_CHAIN_ID};
use atc_sdk::TransactionBuilder;
use ed25519_dalek::SigningKey;

pub const SYSTEM_CHAIN_ID: u64 = NUMERIC_CHAIN_ID;

pub fn build_signed_transfer(
    sender: &str,
    recipient: &str,
    amount: u128,
    nonce: u64,
) -> atc_blockchain::mempool::Transaction {
    let key = SigningKey::from_bytes(&[7u8; 32]);
    TransactionBuilder::transfer(
        SYSTEM_CHAIN_ID,
        sender,
        recipient,
        amount,
        1,
        2_000,
        nonce,
        1,
    )
    .sign(&key)
}

pub fn register_dev_validator(node: &atc_blockchain::Node, proposer: &str) {
    node.register_validator(proposer.to_string(), 1)
        .expect("dev validator registration must succeed");
    node.register_validator_key(
        proposer,
        SigningKey::from_bytes(&[1u8; 32])
            .verifying_key()
            .to_bytes(),
    )
    .expect("dev validator key registration must succeed");
    node.set_vote_signer(proposer, [1u8; 32]);
}

pub fn boot_and_build_transaction(
) -> Result<([u8; 32], atc_blockchain::mempool::Transaction), String> {
    let runtime = Runtime::devnet("ecosystem-integration")?;
    let tx = build_signed_transfer("alice", "bob", atc_blockchain::economics::ATC_BASE_UNITS, 0);
    let tx_id = runtime
        .submit(tx.clone(), 1)
        .map_err(|e| format!("transaction rejected: {e:?}"))?;
    Ok((tx_id, tx))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn node_and_sdk_share_the_same_chain_contract() {
        let (tx_id, tx) = boot_and_build_transaction()
            .expect("node must accept a canonical SDK transaction into its mempool");
        assert_eq!(tx.chain_id, SYSTEM_CHAIN_ID);
        assert_eq!(tx_id, tx.id);
        assert_ne!(tx.signature, [0u8; 64]);
    }
    #[test]
    fn transaction_id_is_deterministic() {
        let a = build_signed_transfer("alice", "bob", atc_blockchain::economics::ATC_BASE_UNITS, 0);
        let b = build_signed_transfer("alice", "bob", atc_blockchain::economics::ATC_BASE_UNITS, 0);
        assert_eq!(a.id, b.id);
        assert_eq!(a.payload, b.payload);
    }

    #[test]
    fn sdk_dao_payload_round_trips_through_persistent_state_codec() {
        let tx = TransactionBuilder::dao_create_proposal(
            SYSTEM_CHAIN_ID,
            "alice",
            42,
            1,
            10,
            "Ecosystem integration",
            "Verify DAO state persistence",
            None,
            0,
            1,
            10_000,
            0,
            1,
        );
        let state = atc_blockchain::mempool::StateDb::new();
        state
            .apply_dao_payload(&tx.payload, 1, "alice")
            .expect("DAO proposal must enter canonical chain state");
        let snapshot = state.dao_snapshot();
        let root_before = state.root();
        state
            .restore_dao(&snapshot)
            .expect("DAO snapshot must be decodable");
        assert_eq!(root_before, state.root());
    }
}
#[test]
fn node_produces_and_finalizes_sdk_transaction() {
    let runtime = Runtime::devnet("ecosystem-integration").expect("runtime must boot");
    register_dev_validator(&runtime.node, "ecosystem-integration");
    let tx = build_signed_transfer("alice", "bob", 1, 0);
    runtime
        .submit(tx, 1)
        .expect("transaction must enter mempool");
    let block = runtime
        .produce(2, 100)
        .expect("node must produce a block from the pending transaction");
    assert_eq!(block.height, 1);
    assert_eq!(block.transactions.len(), 1);
    // The single registered dev validator auto-votes for its own block, so
    // weighted finality is reached with one vote out of one validator.
    assert!(runtime
        .finalize(&block)
        .expect("finality check must execute on the single-validator devnet"));
}
#[test]
fn durable_block_state_survives_runtime_restart() {
    let path =
        std::env::temp_dir().join(format!("atc-ecosystem-integration-{}", std::process::id()));
    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(path.with_extension("state"));

    let runtime = Runtime::open_storage("ecosystem-integration", &path)
        .expect("fresh durable runtime must open");
    register_dev_validator(&runtime.node, "ecosystem-integration");
    runtime
        .node
        .state
        .genesis_credit("alice", 1_000_000)
        .expect("genesis allocation must succeed");
    runtime
        .node
        .create_genesis(0)
        .expect("genesis must persist");

    let tx = build_signed_transfer("alice", "bob", 1, 0);
    runtime
        .submit(tx, 1)
        .expect("transaction must enter runtime");
    let block = runtime.produce(2, 100).expect("block must be produced");
    let committed_root = block.state_root;
    let committed_id = block.id;
    drop(runtime);

    let recovered = Runtime::open_storage("ecosystem-integration", &path)
        .expect("restart must reconstruct persisted chain and state");
    assert_eq!(recovered.node.chain.height(), 1);
    assert_eq!(recovered.node.storage.block(1).unwrap().id, committed_id);
    assert_eq!(
        atc_blockchain::committed_state_root(
            recovered.node.state.root(),
            1,
            recovered.node.consensus.validator_snapshot_commitment(1),
        )
        .expect("recovered validator commitment must be available"),
        committed_root
    );
    assert_eq!(recovered.node.storage.state_root(1), Some(committed_root));

    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(path.with_extension("state"));
}
#[test]
fn dao_transaction_state_survives_runtime_restart() {
    let path = std::env::temp_dir().join(format!("atc-ecosystem-dao-{}", std::process::id()));
    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(path.with_extension("state"));

    let runtime =
        Runtime::open_storage("ecosystem-integration", &path).expect("durable runtime must open");
    register_dev_validator(&runtime.node, "ecosystem-integration");
    runtime
        .node
        .state
        .genesis_credit("alice", 1_000_000)
        .expect("genesis allocation must respect supply cap");
    runtime
        .node
        .create_genesis(0)
        .expect("genesis must persist");

    let key = SigningKey::from_bytes(&[11u8; 32]);

    let stake = TransactionBuilder::stake(
        SYSTEM_CHAIN_ID,
        "alice",
        100_000 * atc_blockchain::economics::ATC_BASE_UNITS,
        1,
        2_000,
        0,
        1,
    )
    .sign(&key);
    runtime.submit(stake, 1).expect("stake must enter runtime");
    runtime
        .produce(1, 10)
        .expect("stake block must be produced");

    let create = TransactionBuilder::dao_create_proposal(
        SYSTEM_CHAIN_ID,
        "alice",
        7,
        2,
        5,
        "Treasury",
        "Fund audit",
        Some("bob"),
        125 * atc_blockchain::economics::ATC_BASE_UNITS,
        1,
        6_000,
        1,
        2,
    )
    .sign(&key);
    runtime
        .submit(create, 2)
        .expect("DAO proposal must enter runtime");
    runtime
        .produce(2, 10)
        .expect("proposal block must be produced");

    let fund = TransactionBuilder::dao_fund(
        SYSTEM_CHAIN_ID,
        "alice",
        500 * atc_blockchain::economics::ATC_BASE_UNITS,
        1,
        6_000,
        2,
        3,
    )
    .sign(&key);
    runtime
        .submit(fund, 3)
        .expect("DAO funding must enter runtime");
    runtime
        .produce(3, 10)
        .expect("funding block must be produced");

    let vote =
        TransactionBuilder::dao_vote(SYSTEM_CHAIN_ID, "alice", 7, 0, 1, 6_000, 3, 4).sign(&key);
    runtime
        .submit(vote, 4)
        .expect("DAO vote must enter runtime");
    runtime.produce(4, 10).expect("vote block must be produced");

    let finalize =
        TransactionBuilder::dao_finalize(SYSTEM_CHAIN_ID, "alice", 7, 1, 6_000, 4, 5).sign(&key);
    runtime
        .submit(finalize, 5)
        .expect("DAO finalization must enter runtime");
    runtime
        .produce(5, 10)
        .expect("finalization block must be produced");

    let execute =
        TransactionBuilder::dao_execute(SYSTEM_CHAIN_ID, "alice", 7, 1, 6_000, 5, 6).sign(&key);
    runtime
        .submit(execute, 6)
        .expect("DAO execution must enter runtime");
    runtime
        .produce(6, 10)
        .expect("execution block must be produced");

    let before = atc_blockchain::dao_state::DaoState::decode(&runtime.node.state.dao_snapshot())
        .expect("DAO state must decode");
    assert_eq!(
        before.proposals.get(&7).unwrap().status,
        atc_blockchain::dao_state::Status::Executed
    );
    assert_eq!(
        before.treasury,
        375 * atc_blockchain::economics::ATC_BASE_UNITS
    );
    assert_eq!(runtime.node.state.balance("bob"), 125);
    drop(runtime);

    let recovered = Runtime::open_storage("ecosystem-integration", &path)
        .expect("restart must reconstruct DAO state");
    let after = atc_blockchain::dao_state::DaoState::decode(&recovered.node.state.dao_snapshot())
        .expect("recovered DAO state must decode");
    assert_eq!(
        after.proposals.get(&7).unwrap().status,
        atc_blockchain::dao_state::Status::Executed
    );
    assert_eq!(
        after.treasury,
        375 * atc_blockchain::economics::ATC_BASE_UNITS
    );
    assert_eq!(recovered.node.state.balance("bob"), 125);
    assert_eq!(recovered.node.state.staked("alice"), 100_000);
    assert_eq!(recovered.node.chain.height(), 6);

    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(path.with_extension("state"));
}
