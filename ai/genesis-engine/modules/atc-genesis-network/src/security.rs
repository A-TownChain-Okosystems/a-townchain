use crate::{NetworkEntity, ReplicatedState, ReplicationHeader, Tick};

pub const MAX_REPLICATION_PACKET_BYTES: usize = ReplicationHeader::BYTES + ReplicatedState::BYTES;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PeerId(pub u64);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SessionId(pub u64);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PeerSequence {
    pub peer: PeerId,
    pub last_sequence: u64,
    pub last_tick: Tick,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PacketRejectReason {
    Malformed,
    Oversized,
    SessionMismatch,
    Replay,
    StaleTick,
    EntityMismatch,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidatedPacket {
    pub peer: PeerId,
    pub session: SessionId,
    pub header: ReplicationHeader,
    pub state: ReplicatedState,
}

pub struct PacketGuard {
    peer: PeerId,
    session: SessionId,
    sequence: PeerSequence,
}

impl PacketGuard {
    pub fn new(peer: PeerId, session: SessionId) -> Self {
        Self {
            peer,
            session,
            sequence: PeerSequence {
                peer,
                last_sequence: 0,
                last_tick: Tick(0),
            },
        }
    }
    pub fn peer(&self) -> PeerId {
        self.peer
    }
    pub fn session(&self) -> SessionId {
        self.session
    }
    pub fn last_sequence(&self) -> u64 {
        self.sequence.last_sequence
    }
    pub fn last_tick(&self) -> Tick {
        self.sequence.last_tick
    }

    pub fn validate(
        &mut self,
        session: SessionId,
        bytes: &[u8],
    ) -> Result<ValidatedPacket, PacketRejectReason> {
        self.validate_inner(session, None, bytes)
    }

    /// Validate and authorize a packet for a specific entity before committing sequence state.
    /// This prevents a packet for another entity from consuming a valid sequence number.
    pub fn validate_for_entity(
        &mut self,
        session: SessionId,
        expected: NetworkEntity,
        bytes: &[u8],
    ) -> Result<ValidatedPacket, PacketRejectReason> {
        self.validate_inner(session, Some(expected), bytes)
    }

    fn validate_inner(
        &mut self,
        session: SessionId,
        expected: Option<NetworkEntity>,
        bytes: &[u8],
    ) -> Result<ValidatedPacket, PacketRejectReason> {
        if bytes.len() > MAX_REPLICATION_PACKET_BYTES {
            return Err(PacketRejectReason::Oversized);
        }
        if session != self.session {
            return Err(PacketRejectReason::SessionMismatch);
        }
        if bytes.len() != MAX_REPLICATION_PACKET_BYTES {
            return Err(PacketRejectReason::Malformed);
        }
        let header = ReplicationHeader::decode(&bytes[..ReplicationHeader::BYTES])
            .ok_or(PacketRejectReason::Malformed)?;
        let state = ReplicatedState::decode(&bytes[ReplicationHeader::BYTES..])
            .ok_or(PacketRejectReason::Malformed)?;
        if header.entity != state.entity {
            return Err(PacketRejectReason::EntityMismatch);
        }
        if let Some(expected_entity) = expected {
            if state.entity != expected_entity {
                return Err(PacketRejectReason::EntityMismatch);
            }
        }
        if header.tick != state.tick || !state.has_valid_rotation() {
            return Err(PacketRejectReason::Malformed);
        }
        if header.sequence <= self.sequence.last_sequence {
            return Err(PacketRejectReason::Replay);
        }
        if header.tick.0 < self.sequence.last_tick.0 {
            return Err(PacketRejectReason::StaleTick);
        }
        self.sequence.last_sequence = header.sequence;
        self.sequence.last_tick = header.tick;
        Ok(ValidatedPacket {
            peer: self.peer,
            session: self.session,
            header,
            state,
        })
    }
}

pub fn validate_entity_owner(expected: NetworkEntity, state: &ReplicatedState) -> bool {
    expected == state.entity
}

#[cfg(test)]
mod tests {
    use super::*;
    fn packet_for(entity: u64, tick: u64, sequence: u64) -> Vec<u8> {
        let header = ReplicationHeader {
            tick: Tick(tick),
            sequence,
            entity: NetworkEntity(entity),
        };
        let state = ReplicatedState {
            entity: NetworkEntity(entity),
            tick: Tick(tick),
            position_mm: [1, 2, 3],
            rotation_xyz_microunits: [4, 5, 6],
        };
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&header.encode());
        bytes.extend_from_slice(&state.encode());
        bytes
    }
    fn packet() -> Vec<u8> {
        packet_for(9, 4, 1)
    }
    #[test]
    fn accepts_monotonic_packet() {
        let mut g = PacketGuard::new(PeerId(7), SessionId(11));
        let p = g.validate(SessionId(11), &packet()).unwrap();
        assert_eq!(p.peer, PeerId(7));
        assert_eq!(g.last_sequence(), 1);
        assert_eq!(g.last_tick(), Tick(4));
    }
    #[test]
    fn rejects_wrong_session() {
        let mut g = PacketGuard::new(PeerId(7), SessionId(11));
        assert_eq!(
            g.validate(SessionId(12), &packet()),
            Err(PacketRejectReason::SessionMismatch)
        );
    }
    #[test]
    fn rejects_replay() {
        let mut g = PacketGuard::new(PeerId(7), SessionId(11));
        assert!(g.validate(SessionId(11), &packet()).is_ok());
        assert_eq!(
            g.validate(SessionId(11), &packet()),
            Err(PacketRejectReason::Replay)
        );
    }
    #[test]
    fn rejects_stale_tick() {
        let mut g = PacketGuard::new(PeerId(7), SessionId(11));
        assert!(g.validate(SessionId(11), &packet()).is_ok());
        let b = packet_for(9, 3, 2);
        assert_eq!(
            g.validate(SessionId(11), &b),
            Err(PacketRejectReason::StaleTick)
        );
    }
    #[test]
    fn rejects_oversized_packet() {
        let mut g = PacketGuard::new(PeerId(7), SessionId(11));
        let mut b = packet();
        b.push(0);
        assert_eq!(
            g.validate(SessionId(11), &b),
            Err(PacketRejectReason::Oversized)
        );
    }
    #[test]
    fn rejects_invalid_rotation() {
        let mut g = PacketGuard::new(PeerId(7), SessionId(11));
        let mut b = packet();
        let offset = ReplicationHeader::BYTES + 28;
        b[offset..offset + 4].copy_from_slice(&1_000_001i32.to_le_bytes());
        assert_eq!(
            g.validate(SessionId(11), &b),
            Err(PacketRejectReason::Malformed)
        );
        assert_eq!(g.last_sequence(), 0);
    }
    #[test]
    fn unauthorized_entity_does_not_advance_sequence() {
        let mut g = PacketGuard::new(PeerId(7), SessionId(11));
        let wrong = packet_for(99, 4, 1);
        assert_eq!(
            g.validate_for_entity(SessionId(11), NetworkEntity(9), &wrong),
            Err(PacketRejectReason::EntityMismatch)
        );
        assert_eq!(g.last_sequence(), 0);
        assert_eq!(g.last_tick(), Tick(0));
        let allowed = packet_for(9, 4, 1);
        assert!(g
            .validate_for_entity(SessionId(11), NetworkEntity(9), &allowed)
            .is_ok());
        assert_eq!(g.last_sequence(), 1);
    }
}
