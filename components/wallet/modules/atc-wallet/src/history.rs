//! Deterministic wallet transaction history.
#[derive(Debug,Clone,PartialEq,Eq)] pub struct HistoryEntry{pub tx_id:[u8;32],pub height:u64,pub timestamp:u64}
#[derive(Debug,Default,Clone,PartialEq,Eq)] pub struct TransactionHistory{entries:Vec<HistoryEntry>}
impl TransactionHistory{pub fn push(&mut self,e:HistoryEntry)->Result<(),HistoryError>{if self.entries.last().is_some_and(|x|e.height<x.height){return Err(HistoryError::NonMonotonicHeight)}self.entries.push(e);Ok(())}pub fn entries(&self)->&[HistoryEntry]{&self.entries}pub fn len(&self)->usize{self.entries.len()}}
#[derive(Debug,Clone,Copy,PartialEq,Eq)] pub enum HistoryError{NonMonotonicHeight}
