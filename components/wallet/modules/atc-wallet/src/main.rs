use atc_wallet::{balance::Balance,keys::WalletKey};
fn main(){let key=WalletKey::from_seed([0u8;32]);let balance=Balance::default();println!("atc-wallet — A-TownChain-Okosystems");println!("public_key={:02x?}",key.public_key());println!("available={}",balance.available());println!("locked={}",balance.locked());}
