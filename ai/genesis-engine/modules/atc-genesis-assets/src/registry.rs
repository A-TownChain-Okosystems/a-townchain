use atc_genesis_platform::{AssetId, AssetKind};
use std::collections::{HashMap,HashSet};
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct AssetMetadata{pub id:AssetId,pub kind:AssetKind,pub source:String,pub dependencies:Vec<AssetId>}
#[derive(Default)]
pub struct AssetRegistry{assets:HashMap<AssetId,AssetMetadata>}
impl AssetRegistry{
 pub fn register(&mut self,mut asset:AssetMetadata){asset.dependencies.sort_by_key(|id|id.0);asset.dependencies.dedup();self.assets.insert(asset.id,asset);}
 pub fn get(&self,id:AssetId)->Option<&AssetMetadata>{self.assets.get(&id)}
 pub fn dependencies(&self,id:AssetId)->Option<&[AssetId]>{self.assets.get(&id).map(|a|a.dependencies.as_slice())}
 pub fn dependents(&self,id:AssetId)->Vec<AssetId>{let mut out:Vec<_>=self.assets.values().filter(|a|a.dependencies.contains(&id)).map(|a|a.id).collect();out.sort_by_key(|x|x.0);out}
 pub fn missing_dependencies(&self,id:AssetId)->Option<Vec<AssetId>>{self.assets.get(&id).map(|a|{let mut m:Vec<_>=a.dependencies.iter().copied().filter(|d|!self.assets.contains_key(d)).collect();m.sort_by_key(|x|x.0);m})}
 pub fn has_dependency_cycle(&self,start:AssetId)->bool{fn visit(reg:&AssetRegistry,id:AssetId,stack:&mut HashSet<AssetId>,done:&mut HashSet<AssetId>)->bool{if stack.contains(&id){return true}if done.contains(&id){return false}let Some(a)=reg.assets.get(&id)else{return false};stack.insert(id);for d in &a.dependencies{if visit(reg,*d,stack,done){return true}}stack.remove(&id);done.insert(id);false}visit(self,start,&mut HashSet::new(),&mut HashSet::new())}
 pub fn len(&self)->usize{self.assets.len()}
 pub fn is_empty(&self)->bool{self.assets.is_empty()}
}
#[cfg(test)]mod tests{use super::*;#[test]fn dependency_graph_detects_cycle(){let mut r=AssetRegistry::default();r.register(AssetMetadata{id:AssetId(1),kind:AssetKind::Mesh,source:"a".into(),dependencies:vec![AssetId(2)]});r.register(AssetMetadata{id:AssetId(2),kind:AssetKind::Material,source:"b".into(),dependencies:vec![AssetId(1)]});assert!(r.has_dependency_cycle(AssetId(1)));}}
