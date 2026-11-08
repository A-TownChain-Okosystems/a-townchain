// Error types
#[derive(Debug)]
pub enum BridgeError { InsufficientFunds, InvalidSignature, InvalidChain, Timeout }
impl std::fmt::Display for BridgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BridgeError::InsufficientFunds => write!(f, "Insufficient funds"),
            BridgeError::InvalidSignature => write!(f, "Invalid signature"),
            BridgeError::InvalidChain => write!(f, "Invalid chain ID"),
            BridgeError::Timeout => write!(f, "Operation timed out"),
        }
    }
}
impl std::error::Error for BridgeError {}
