// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Deterministische Proof-Serialisierung (MVP-Format).

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proof {
    pub system_id: u8,
    pub commitment: [u8; 32],
    pub public_inputs: Vec<u64>,
}

impl Proof {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.push(self.system_id);
        out.extend_from_slice(&self.commitment);
        out.extend_from_slice(&(self.public_inputs.len() as u64).to_le_bytes());
        for v in &self.public_inputs {
            out.extend_from_slice(&v.to_le_bytes());
        }
        out
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Proof> {
        if bytes.len() < 41 {
            return None;
        }
        let system_id = bytes[0];
        let mut commitment = [0u8; 32];
        commitment.copy_from_slice(&bytes[1..33]);
        let n = u64::from_le_bytes(bytes[33..41].try_into().ok()?) as usize;
        let rest = &bytes[41..];
        if rest.len() != n * 8 {
            return None;
        }
        let mut public_inputs = Vec::with_capacity(n);
        for i in 0..n {
            let chunk: [u8; 8] = rest[i * 8..i * 8 + 8].try_into().ok()?;
            public_inputs.push(u64::from_le_bytes(chunk));
        }
        Some(Proof { system_id, commitment, public_inputs })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn roundtrip() {
        let p = Proof { system_id: 1, commitment: [7; 32], public_inputs: vec![1, 2, 3] };
        let bytes = p.to_bytes();
        assert_eq!(Proof::from_bytes(&bytes), Some(p));
        assert!(Proof::from_bytes(&bytes[..40]).is_none());
    }
}
