// Event relay and verification
pub struct Relay {
    events: Vec<RelayEvent>,
}

#[derive(Debug, Clone)]
pub struct RelayEvent {
    pub source_chain: String,
    pub target_chain: String,
    pub event_type: String,
    pub data: Vec<u8>,
    pub signatures: Vec<Vec<u8>>,
}

impl Relay {
    pub fn new() -> Self { Self { events: Vec::new() } }
    pub fn submit(&mut self, event: RelayEvent) -> usize {
        self.events.push(event);
        self.events.len() - 1
    }
    pub fn get(&self, id: usize) -> Option<&RelayEvent> { self.events.get(id) }
    pub fn count(&self) -> usize { self.events.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_relay() {
        let mut r = Relay::new();
        let id = r.submit(RelayEvent {
            source_chain: "ethereum".into(), target_chain: "atc".into(),
            event_type: "transfer".into(), data: vec![], signatures: vec![],
        });
        assert_eq!(r.count(), 1);
        assert!(r.get(id).is_some());
    }
}
