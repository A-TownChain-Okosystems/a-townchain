// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! ProofRequest-Builder (MVP).

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofRequest {
    pub circuit_id: u64,
    pub public_inputs: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestError {
    pub reason: &'static str,
}

pub struct ProofRequestBuilder {
    circuit_id: Option<u64>,
    public_inputs: Vec<u64>,
}

impl ProofRequestBuilder {
    pub fn new() -> Self {
        ProofRequestBuilder { circuit_id: None, public_inputs: Vec::new() }
    }

    pub fn circuit(mut self, id: u64) -> Self {
        self.circuit_id = Some(id);
        self
    }

    pub fn public_input(mut self, v: u64) -> Self {
        self.public_inputs.push(v);
        self
    }

    pub fn build(self) -> Result<ProofRequest, RequestError> {
        match self.circuit_id {
            Some(c) if !self.public_inputs.is_empty() => {
                Ok(ProofRequest { circuit_id: c, public_inputs: self.public_inputs })
            }
            Some(_) => Err(RequestError { reason: "keine public inputs" }),
            None => Err(RequestError { reason: "keine circuit id" }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn builder() {
        let ok = ProofRequestBuilder::new().circuit(7).public_input(1).public_input(2).build();
        assert_eq!(ok, Ok(ProofRequest { circuit_id: 7, public_inputs: vec![1, 2] }));
        assert_eq!(ProofRequestBuilder::new().build(), Err(RequestError { reason: "keine circuit id" }));
        assert_eq!(ProofRequestBuilder::new().circuit(1).build(), Err(RequestError { reason: "keine public inputs" }));
    }
}
