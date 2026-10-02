//! Checked wallet balance accounting; economic amounts are u128.
#[derive(Debug,Clone,Copy,PartialEq,Eq,Default)]
pub struct Balance { available:u128, locked:u128 }
impl Balance {
 pub const fn new(available:u128,locked:u128)->Self{Self{available,locked}}
 pub const fn available(&self)->u128{self.available}
 pub const fn locked(&self)->u128{self.locked}
 pub fn total(&self)->Option<u128>{self.available.checked_add(self.locked)}
 pub fn lock(&mut self,a:u128)->Result<(),BalanceError>{self.available=self.available.checked_sub(a).ok_or(BalanceError::InsufficientAvailable)?;self.locked=self.locked.checked_add(a).ok_or(BalanceError::Overflow)?;Ok(())}
 pub fn unlock(&mut self,a:u128)->Result<(),BalanceError>{self.locked=self.locked.checked_sub(a).ok_or(BalanceError::InsufficientLocked)?;self.available=self.available.checked_add(a).ok_or(BalanceError::Overflow)?;Ok(())}
}
#[derive(Debug,Clone,Copy,PartialEq,Eq)] pub enum BalanceError{InsufficientAvailable,InsufficientLocked,Overflow}
