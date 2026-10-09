//! Canonical wallet façade over key management and transaction signing.
use crate::keys::{KeyError, KeyPair};
use crate::tx::{SignedTransaction, Transaction, TransactionDomain, TxError};

#[derive(Debug)]
pub struct Wallet {
    keys: KeyPair,
    nonce: u64,
}

impl Wallet {
    pub fn generate() -> Result<Self, KeyError> {
        Ok(Self { keys: KeyPair::generate()?, nonce: 0 })
    }

    pub fn from_seed(seed: [u8; 32]) -> Self {
        Self { keys: KeyPair::from_seed(seed), nonce: 0 }
    }

    pub fn address(&self) -> String { self.keys.address() }

    pub fn public_key(&self) -> [u8; 32] { self.keys.public_key_bytes() }

    pub fn nonce(&self) -> u64 { self.nonce }

    pub fn sign_transaction(
        &mut self,
        domain: &TransactionDomain,
        recipient: Vec<u8>,
        value: u64,
        fee: u64,
        payload: Vec<u8>,
    ) -> Result<SignedTransaction, TxError> {
        let tx = Transaction {
            nonce: self.nonce,
            sender: self.keys.public_key_bytes().to_vec(),
            recipient,
            value,
            fee,
            payload,
        };
        let signature = domain.sign(&tx, &self.keys.signing_key())?;
        self.nonce = self.nonce.checked_add(1).ok_or(TxError::NonceOverflow)?;
        Ok(SignedTransaction { transaction: tx, public_key: self.keys.verifying_key(), signature })
    }

    pub fn verify(
        domain: &TransactionDomain,
        signed: &SignedTransaction,
    ) -> Result<(), TxError> {
        domain.verify(&signed.transaction, &signed.public_key, &signed.signature)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn domain() -> TransactionDomain {
        TransactionDomain {
            chain_id: "atc".into(),
            network_id: "devnet".into(),
            protocol_version: "1.0.0".into(),
            transaction_type: "transfer".into(),
        }
    }

    #[test]
    fn wallet_signs_and_advances_nonce() {
        let mut wallet = Wallet::from_seed([3u8; 32]);
        let signed = wallet.sign_transaction(&domain(), vec![2; 32], 42, 1, Vec::new()).unwrap();
        assert_eq!(signed.transaction.nonce, 0);
        assert_eq!(wallet.nonce(), 1);
        Wallet::verify(&domain(), &signed).unwrap();
    }

    #[test]
    fn altered_transaction_is_rejected() {
        let mut wallet = Wallet::from_seed([4u8; 32]);
        let mut signed = wallet.sign_transaction(&domain(), vec![2; 32], 42, 1, Vec::new()).unwrap();
        signed.transaction.value = 43;
        assert_eq!(Wallet::verify(&domain(), &signed), Err(TxError::InvalidSignature));
    }
}
