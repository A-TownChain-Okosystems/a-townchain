// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// K-Sprint 18 — Block-Proposal-Pipeline
use alloc::vec;
use alloc::collections::BTreeMap; use alloc::vec::Vec; use alloc::string::{String,ToString}; use alloc::sync::Arc; use spin::Mutex;
use crate::mempool::{Transaction,TxStatus,TxType};
use crate::consensus::ConsensusEngine;
use crate::security::simple_hash;
#[derive(Clone,Debug,PartialEq)]
pub struct Block{pub id:[u8;32],pub height:u64,pub parent_hash:[u8;32],pub proposer_did:String,pub timestamp:u64,pub poh_hash:[u8;32],pub transactions:Vec<Transaction>,pub tx_root:[u8;32],pub state_root:[u8;32],pub gas_used:u64,pub total_fees:u64,pub signature:[u8;64]}
impl Block{pub fn new(height:u64,parent_hash:[u8;32],proposer_did:String,timestamp:u64,poh_hash:[u8;32],transactions:Vec<Transaction>,signature:[u8;64])->Self{let gas_used:u64=transactions.iter().map(|tx|tx.gas_cost()).sum();let total_fees:u64=transactions.iter().map(|tx|tx.max_fee()).sum();let mut ti=Vec::new();for tx in&transactions{ti.extend_from_slice(&tx.id);}let tx_root=simple_hash(&ti);let mut bi=Vec::new();bi.extend_from_slice(&height.to_be_bytes());bi.extend_from_slice(&parent_hash);bi.extend_from_slice(proposer_did.as_bytes());bi.extend_from_slice(&timestamp.to_be_bytes());bi.extend_from_slice(&poh_hash);bi.extend_from_slice(&tx_root);bi.extend_from_slice(&signature);let id=simple_hash(&bi);Block{id,height,parent_hash,proposer_did,timestamp,poh_hash,transactions,tx_root,state_root:[0u8;32],gas_used,total_fees,signature}}
pub fn tx_count(&self)->usize{self.transactions.len()} pub fn is_empty(&self)->bool{self.transactions.is_empty()}}
pub struct BlockChain{blocks:Mutex<BTreeMap<u64,Block>>,by_hash:Mutex<BTreeMap<[u8;32],u64>>,current_height:Mutex<u64>,genesis_hash:Mutex<Option<[u8;32]>>}
impl BlockChain{pub fn new()->Self{BlockChain{blocks:Mutex::new(BTreeMap::new()),by_hash:Mutex::new(BTreeMap::new()),current_height:Mutex::new(0),genesis_hash:Mutex::new(None)}}
pub fn add_genesis(&self,block:Block)->Result<(),PipelineError>{if block.height!=0{return Err(PipelineError::InvalidHeight);}let mut b=self.blocks.lock();if b.contains_key(&0){return Err(PipelineError::GenesisExists);}*self.genesis_hash.lock()=Some(block.id);self.by_hash.lock().insert(block.id,0);b.insert(0,block);*self.current_height.lock()=0;Ok(())}
pub fn add_block(&self,block:Block)->Result<(),PipelineError>{let mut b=self.blocks.lock();let mut bh=self.by_hash.lock();let mut h=self.current_height.lock();if block.height!=*h+1{return Err(PipelineError::InvalidHeight);}if b.contains_key(&block.height){return Err(PipelineError::BlockExists);}if bh.contains_key(&block.id){return Err(PipelineError::DuplicateBlock);}let bid=block.id;let bh2=block.height;bh.insert(bid,bh2);b.insert(bh2,block);*h=bh2;Ok(())}
pub fn get_block(&self,height:u64)->Option<Block>{self.blocks.lock().get(&height).cloned()} pub fn get_by_hash(&self,hash:&[u8;32])->Option<Block>{let bh=self.by_hash.lock();bh.get(hash).and_then(|&h|self.blocks.lock().get(&h).cloned())} pub fn current_height(&self)->u64{*self.current_height.lock()} pub fn block_count(&self)->usize{self.blocks.lock().len()} pub fn last_block(&self)->Option<Block>{let h=*self.current_height.lock();self.blocks.lock().get(&h).cloned()}}
pub struct ProposalPipeline{mempool:Arc<crate::mempool::MemoryPool>,validator:Arc<crate::mempool::TxValidator>,consensus:Arc<ConsensusEngine>,chain:Arc<BlockChain>,state:Arc<crate::mempool::StateDb>,our_did:String,block_height:Mutex<u64>}
impl ProposalPipeline{pub fn new(mempool:Arc<crate::mempool::MemoryPool>,validator:Arc<crate::mempool::TxValidator>,consensus:Arc<ConsensusEngine>,chain:Arc<BlockChain>,state:Arc<crate::mempool::StateDb>,our_did:String)->Self{ProposalPipeline{mempool,validator,consensus,chain,state,our_did,block_height:Mutex::new(0)}}
pub fn create_genesis(&self,timestamp:u64)->Result<Block,PipelineError>{let pe=self.consensus.poh().tick(timestamp);let g=Block::new(0,[0u8;32],self.our_did.clone(),timestamp,pe.hash,vec![],[0u8;64]);self.chain.add_genesis(g.clone())?;*self.block_height.lock()=0;let _=self.consensus.init_genesis(timestamp).map_err(|_|PipelineError::DagInsertFailed)?;Ok(g)}
pub fn propose_block(&self,max_txs:usize,timestamp:u64)->Result<Block,PipelineError>{let pending=self.mempool.get_pending_batch(max_txs);if pending.is_empty(){return Err(PipelineError::NoPendingTxs);}let mut valid=Vec::new();for tx in pending{if self.validator.validate(&tx).is_ok(){self.validator.apply(&tx).unwrap_or(());self.mempool.mark_in_dag(&tx.id);valid.push(tx);}}if valid.is_empty(){return Err(PipelineError::AllTxsInvalid);}let parent=self.chain.last_block().ok_or(PipelineError::NoGenesis)?;let pe=self.consensus.poh().tick(timestamp);let h=parent.height+1;let block=Block::new(h,parent.id,self.our_did.clone(),timestamp,pe.hash,valid,[0u8;64]);self.chain.add_block(block.clone())?;*self.block_height.lock()=h;let _=self.consensus.propose_vertex(block.id,timestamp,[0u8;64]).map_err(|_|PipelineError::DagInsertFailed)?;Ok(block)}
pub fn vote_on_block(&self,block_hash:[u8;32],timestamp:u64,approve:bool,signature:[u8;64]){self.consensus.vote(block_hash,timestamp,approve,signature);}
pub fn cleanup_mempool(&self,now:u64)->usize{self.mempool.cleanup(now)} pub fn chain(&self)->&Arc<BlockChain>{&self.chain} pub fn mempool(&self)->&Arc<crate::mempool::MemoryPool>{&self.mempool} pub fn consensus(&self)->&Arc<ConsensusEngine>{&self.consensus} pub fn state(&self)->&Arc<crate::mempool::StateDb>{&self.state} pub fn current_height(&self)->u64{*self.block_height.lock()}}
#[derive(Debug,Clone,PartialEq,Eq)] pub enum PipelineError{NoPendingTxs,AllTxsInvalid,NoGenesis,InvalidHeight,ParentNotFound,BlockExists,DuplicateBlock,GenesisExists,DagInsertFailed}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::mempool::{NonceTracker, MemoryPool, TxType};
    use crate::consensus::ConsensusEngine;
    fn setup() -> ProposalPipeline {
        let state = Arc::new(crate::mempool::StateDb::new());
        let nonces = Arc::new(NonceTracker::new());
        let mp = Arc::new(MemoryPool::new(1000, 300));
        let val = Arc::new(crate::mempool::TxValidator::new(state.clone(), nonces, 1));
        let con = Arc::new(ConsensusEngine::new("did:p".into(), [0x42; 32]));
        let ch = Arc::new(BlockChain::new());
        state.deposit("did:alice", 100_000_000); state.deposit("did:bob", 100_000_000);
        ProposalPipeline::new(mp, val, con, ch, state, "did:p".into())
    }
    fn mk_tx(s: &str, r: &str, a: u64, n: u64) -> Transaction {
        Transaction::new(TxType::Transfer, s.into(), Some(r.into()), a, 10, 50000, n, 1000, vec![], [0u8;64], [0x42;32])
    }
    #[test] fn test_block_create() { let b = Block::new(1, [0xAA;32], "p".into(), 1000, [0xBB;32], vec![], [0;64]); assert_eq!(b.height, 1); assert!(b.is_empty()); assert_eq!(b.gas_used, 0); }
    #[test] fn test_block_with_txs() { let t = vec![mk_tx("a","b",100,0)]; let b = Block::new(1, [0;32], "p".into(), 1000, [0;32], t, [0;64]); assert_eq!(b.tx_count(), 1); assert_eq!(b.gas_used, 1000); }
    #[test] fn test_block_det_id() { let t = vec![mk_tx("a","b",100,0)]; let b1 = Block::new(1,[0xAA;32],"p".into(),1000,[0xBB;32],t.clone(),[0;64]); let b2 = Block::new(1,[0xAA;32],"p".into(),1000,[0xBB;32],t,[0;64]); assert_eq!(b1.id, b2.id); }
    #[test] fn test_chain_genesis() { let c = BlockChain::new(); c.add_genesis(Block::new(0,[0;32],"p".into(),1000,[0;32],vec![],[0;64])).unwrap(); assert_eq!(c.block_count(), 1); assert_eq!(c.current_height(), 0); }
    #[test] fn test_chain_genesis_dup() { let c = BlockChain::new(); c.add_genesis(Block::new(0,[0;32],"p".into(),1000,[0;32],vec![],[0;64])).unwrap(); assert_eq!(c.add_genesis(Block::new(0,[0;32],"p".into(),2000,[0;32],vec![],[0;64])), Err(PipelineError::GenesisExists)); }
    #[test] fn test_chain_add_blocks() { let c = BlockChain::new(); let g = Block::new(0,[0;32],"p".into(),1000,[0;32],vec![],[0;64]); c.add_genesis(g.clone()).unwrap(); let b1 = Block::new(1,g.id,"p".into(),2000,[0;32],vec![],[0;64]); c.add_block(b1).unwrap(); assert_eq!(c.current_height(), 1); }
    #[test] fn test_chain_bad_height() { let c = BlockChain::new(); c.add_genesis(Block::new(0,[0;32],"p".into(),1000,[0;32],vec![],[0;64])).unwrap(); assert_eq!(c.add_block(Block::new(5,[0;32],"p".into(),2000,[0;32],vec![],[0;64])), Err(PipelineError::InvalidHeight)); }
    #[test] fn test_chain_get_by_hash() { let c = BlockChain::new(); let g = Block::new(0,[0;32],"p".into(),1000,[0;32],vec![],[0;64]); let h = g.id; c.add_genesis(g).unwrap(); assert!(c.get_by_hash(&h).is_some()); }
    #[test] fn test_pipeline_genesis() { let p = setup(); let g = p.create_genesis(1000).unwrap(); assert_eq!(g.height, 0); assert_eq!(p.current_height(), 0); }
    #[test] fn test_pipeline_no_pending() { let p = setup(); p.create_genesis(1000).unwrap(); assert_eq!(p.propose_block(10, 2000), Err(PipelineError::NoPendingTxs)); }
    #[test] fn test_pipeline_propose() { let p = setup(); p.create_genesis(1000).unwrap(); let t = mk_tx("did:alice","did:bob",1000,0); let tid = t.id; p.mempool().add(t, 1000).unwrap(); p.mempool().validate_tx(&tid, 1000).unwrap(); let b = p.propose_block(10, 2000).unwrap(); assert_eq!(b.height, 1); assert_eq!(b.tx_count(), 1); }
    #[test] fn test_pipeline_multi_blocks() { let p = setup(); p.create_genesis(1000).unwrap(); for i in 0..2 { let t = mk_tx("did:alice","did:bob",100*(i+1),i); let tid = t.id; p.mempool().add(t, 1000).unwrap(); p.mempool().validate_tx(&tid, 1000).unwrap(); p.propose_block(10, 2000+i as u64).unwrap(); } assert_eq!(p.current_height(), 2); assert_eq!(p.chain().block_count(), 3); }
    #[test] fn test_pipeline_state() { let p = setup(); p.create_genesis(1000).unwrap(); let before = p.state().get_balance("did:alice"); let t = mk_tx("did:alice","did:bob",5000,0); let tid = t.id; p.mempool().add(t, 1000).unwrap(); p.mempool().validate_tx(&tid, 1000).unwrap(); p.propose_block(10, 2000).unwrap(); assert!(p.state().get_balance("did:alice") < before); assert!(p.state().get_balance("did:bob") > 100_000_000); }
    #[test] fn test_pipeline_dag() { let p = setup(); p.create_genesis(1000).unwrap(); assert!(p.consensus().dag().vertex_count() >= 1); let t = mk_tx("did:alice","did:bob",1000,0); let tid = t.id; p.mempool().add(t, 1000).unwrap(); p.mempool().validate_tx(&tid, 1000).unwrap(); p.propose_block(10, 2000).unwrap(); assert!(p.consensus().dag().vertex_count() >= 2); }
    #[test] fn test_block_merkle_diff() { let b1 = Block::new(1,[0;32],"p".into(),1000,[0;32],vec![mk_tx("a","b",100,0)],[0;64]); let b2 = Block::new(1,[0;32],"p".into(),1000,[0;32],vec![mk_tx("a","b",200,0)],[0;64]); assert_ne!(b1.tx_root, b2.tx_root); assert_ne!(b1.id, b2.id); }
}
