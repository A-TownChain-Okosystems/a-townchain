//! Supported CLI commands and their typed dispatch targets.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum Command { ChainId, BootHash, Peers, Ping }
impl Command { pub fn parse(name:&str)->Option<Self>{match name{"chain-id"=>Some(Self::ChainId),"boot-hash"=>Some(Self::BootHash),"peers"=>Some(Self::Peers),"ping"=>Some(Self::Ping),_=>None}} pub const fn name(self)->&'static str{match self{Self::ChainId=>"chain-id",Self::BootHash=>"boot-hash",Self::Peers=>"peers",Self::Ping=>"ping"}} }
