pub mod security;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NetworkEntity(pub u64);
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Tick(pub u64);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReplicationMode {
    Authority,
    Predicted,
    Interpolated,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Snapshot<T> {
    pub tick: Tick,
    pub sequence: u64,
    pub value: T,
}
#[derive(Default)]
pub struct SnapshotBuffer<T> {
    capacity: usize,
    entries: Vec<Snapshot<T>>,
}
impl<T> SnapshotBuffer<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            capacity: capacity.max(1),
            entries: Vec::new(),
        }
    }
    pub fn push(&mut self, s: Snapshot<T>) {
        if self
            .entries
            .last()
            .is_some_and(|x| s.sequence <= x.sequence)
        {
            return;
        }
        self.entries.push(s);
        if self.entries.len() > self.capacity {
            self.entries.remove(0);
        }
    }
    pub fn latest(&self) -> Option<&Snapshot<T>> {
        self.entries.last()
    }
    pub fn sample_pair(&self, tick: Tick) -> Option<(&Snapshot<T>, &Snapshot<T>)> {
        self.entries
            .windows(2)
            .find(|w| w[0].tick <= tick && tick <= w[1].tick)
            .map(|w| (&w[0], &w[1]))
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReplicationHeader {
    pub tick: Tick,
    pub sequence: u64,
    pub entity: NetworkEntity,
}
impl ReplicationHeader {
    pub const BYTES: usize = 24;
    pub fn encode(self) -> [u8; Self::BYTES] {
        let mut b = [0; Self::BYTES];
        b[0..8].copy_from_slice(&self.tick.0.to_le_bytes());
        b[8..16].copy_from_slice(&self.sequence.to_le_bytes());
        b[16..24].copy_from_slice(&self.entity.0.to_le_bytes());
        b
    }
    pub fn decode(b: &[u8]) -> Option<Self> {
        if b.len() < Self::BYTES {
            return None;
        }
        Some(Self {
            tick: Tick(u64::from_le_bytes(b[0..8].try_into().ok()?)),
            sequence: u64::from_le_bytes(b[8..16].try_into().ok()?),
            entity: NetworkEntity(u64::from_le_bytes(b[16..24].try_into().ok()?)),
        })
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplicatedState {
    pub entity: NetworkEntity,
    pub tick: Tick,
    pub position_mm: [i32; 3],
    pub rotation_xyz_microunits: [i32; 3],
}
impl ReplicatedState {
    pub const BYTES: usize = 40;
    pub const ROTATION_SCALE: i64 = 1_000_000;
    pub fn encode(&self) -> [u8; Self::BYTES] {
        let mut b = [0; Self::BYTES];
        b[0..8].copy_from_slice(&self.entity.0.to_le_bytes());
        b[8..16].copy_from_slice(&self.tick.0.to_le_bytes());
        for (i, v) in self.position_mm.iter().enumerate() {
            b[16 + i * 4..20 + i * 4].copy_from_slice(&v.to_le_bytes())
        }
        for (i, v) in self.rotation_xyz_microunits.iter().enumerate() {
            b[28 + i * 4..32 + i * 4].copy_from_slice(&v.to_le_bytes())
        }
        b
    }
    pub fn decode(b: &[u8]) -> Option<Self> {
        if b.len() < Self::BYTES {
            return None;
        }
        let mut p = [0; 3];
        let mut r = [0; 3];
        for i in 0..3 {
            p[i] = i32::from_le_bytes(b[16 + i * 4..20 + i * 4].try_into().ok()?);
            r[i] = i32::from_le_bytes(b[28 + i * 4..32 + i * 4].try_into().ok()?)
        }
        Some(Self {
            entity: NetworkEntity(u64::from_le_bytes(b[0..8].try_into().ok()?)),
            tick: Tick(u64::from_le_bytes(b[8..16].try_into().ok()?)),
            position_mm: p,
            rotation_xyz_microunits: r,
        })
    }
    pub fn has_valid_rotation(&self) -> bool {
        let scale = Self::ROTATION_SCALE;
        let limit = scale * scale;
        let norm_squared = self
            .rotation_xyz_microunits
            .iter()
            .map(|component| {
                let value = i64::from(*component);
                value * value
            })
            .sum::<i64>();
        norm_squared <= limit
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InterpolatedTransform {
    pub position: [f32; 3],
    pub rotation: [f32; 3],
}
impl InterpolatedTransform {
    pub fn between(a: &ReplicatedState, b: &ReplicatedState, alpha: f32) -> Option<Self> {
        if a.entity != b.entity
            || b.tick.0 < a.tick.0
            || !a.has_valid_rotation()
            || !b.has_valid_rotation()
        {
            return None;
        }
        let t = alpha.clamp(0.0, 1.0);
        Some(Self {
            position: std::array::from_fn(|i| {
                (a.position_mm[i] as f32 + (b.position_mm[i] - a.position_mm[i]) as f32 * t) * 0.001
            }),
            rotation: std::array::from_fn(|i| {
                (a.rotation_xyz_microunits[i] as f32
                    + (b.rotation_xyz_microunits[i] - a.rotation_xyz_microunits[i]) as f32 * t)
                    * 0.000001
            }),
        })
    }
}
pub struct PredictionBuffer {
    inputs: Vec<(Tick, [f32; 3])>,
    capacity: usize,
}
impl PredictionBuffer {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inputs: Vec::new(),
            capacity: capacity.max(1),
        }
    }
    pub fn push(&mut self, tick: Tick, input: [f32; 3]) {
        self.inputs.push((tick, input));
        if self.inputs.len() > self.capacity {
            self.inputs.remove(0);
        }
    }
    pub fn inputs_after(&self, tick: Tick) -> impl Iterator<Item = (Tick, [f32; 3])> + '_ {
        self.inputs
            .iter()
            .copied()
            .filter(move |(t, _)| t.0 > tick.0)
    }
    pub fn len(&self) -> usize {
        self.inputs.len()
    }
    pub fn is_empty(&self) -> bool {
        self.inputs.is_empty()
    }
}
pub trait ReplicationTransport {
    fn send(&mut self, bytes: &[u8]) -> Result<(), String>;
}
#[derive(Default)]
pub struct LoopbackTransport {
    packets: Vec<Vec<u8>>,
}
impl LoopbackTransport {
    pub fn drain(&mut self) -> Vec<Vec<u8>> {
        std::mem::take(&mut self.packets)
    }
}
impl ReplicationTransport for LoopbackTransport {
    fn send(&mut self, bytes: &[u8]) -> Result<(), String> {
        self.packets.push(bytes.to_vec());
        Ok(())
    }
}
#[derive(Default)]
pub struct Replicator<T: ReplicationTransport> {
    pub transport: T,
    sequence: u64,
    last_received_sequence: u64,
    last_received_tick: Tick,
}
impl<T: ReplicationTransport> Replicator<T> {
    pub fn new(transport: T) -> Self {
        Self {
            transport,
            sequence: 0,
            last_received_sequence: 0,
            last_received_tick: Tick(0),
        }
    }
    pub fn publish(&mut self, state: &ReplicatedState) -> Result<u64, String> {
        self.sequence = self.sequence.saturating_add(1);
        let h = ReplicationHeader {
            tick: state.tick,
            sequence: self.sequence,
            entity: state.entity,
        };
        let mut p = Vec::with_capacity(ReplicationHeader::BYTES + ReplicatedState::BYTES);
        p.extend_from_slice(&h.encode());
        p.extend_from_slice(&state.encode());
        self.transport.send(&p)?;
        Ok(self.sequence)
    }
    pub fn decode_packet(bytes: &[u8]) -> Option<(ReplicationHeader, ReplicatedState)> {
        let expected = ReplicationHeader::BYTES + ReplicatedState::BYTES;
        if bytes.len() != expected {
            return None;
        }
        let header = ReplicationHeader::decode(&bytes[..ReplicationHeader::BYTES])?;
        let state = ReplicatedState::decode(&bytes[ReplicationHeader::BYTES..])?;
        if state.entity != header.entity || state.tick != header.tick || !state.has_valid_rotation()
        {
            return None;
        }
        Some((header, state))
    }
    pub fn accept_packet(&mut self, bytes: &[u8]) -> Option<ReplicatedState> {
        let (header, state) = Self::decode_packet(bytes)?;
        if header.sequence <= self.last_received_sequence
            || header.tick.0 < self.last_received_tick.0
        {
            return None;
        }
        self.last_received_sequence = header.sequence;
        self.last_received_tick = header.tick;
        Some(state)
    }
    pub fn sequence(&self) -> u64 {
        self.sequence
    }
    pub fn last_received_sequence(&self) -> u64 {
        self.last_received_sequence
    }
    pub fn last_received_tick(&self) -> Tick {
        self.last_received_tick
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn state() -> ReplicatedState {
        ReplicatedState {
            entity: NetworkEntity(2),
            tick: Tick(8),
            position_mm: [1, -2, 3],
            rotation_xyz_microunits: [400000, 500000, -600000],
        }
    }
    #[test]
    fn buffer() {
        let mut b = SnapshotBuffer::with_capacity(2);
        b.push(Snapshot {
            tick: Tick(0),
            sequence: 1,
            value: 1,
        });
        b.push(Snapshot {
            tick: Tick(1),
            sequence: 2,
            value: 2,
        });
        b.push(Snapshot {
            tick: Tick(2),
            sequence: 1,
            value: 3,
        });
        assert_eq!(b.latest().unwrap().value, 2)
    }
    #[test]
    fn header() {
        let h = ReplicationHeader {
            tick: Tick(4),
            sequence: 7,
            entity: NetworkEntity(9),
        };
        assert_eq!(ReplicationHeader::decode(&h.encode()), Some(h))
    }
    #[test]
    fn state_roundtrip() {
        let s = state();
        assert_eq!(ReplicatedState::decode(&s.encode()), Some(s))
    }
    #[test]
    fn valid_rotation_is_accepted() {
        assert!(state().has_valid_rotation())
    }
    #[test]
    fn invalid_rotation_is_rejected() {
        let mut s = state();
        s.rotation_xyz_microunits = [1000001, 0, 0];
        assert!(!s.has_valid_rotation());
        let mut r = Replicator::new(LoopbackTransport::default());
        r.publish(&s).unwrap();
        let p = r.transport.drain().pop().unwrap();
        assert!(r.accept_packet(&p).is_none())
    }
    #[test]
    fn interpolation() {
        let a = ReplicatedState {
            entity: NetworkEntity(1),
            tick: Tick(1),
            position_mm: [0; 3],
            rotation_xyz_microunits: [0; 3],
        };
        let b = ReplicatedState {
            entity: NetworkEntity(1),
            tick: Tick(2),
            position_mm: [1000, 2000, 3000],
            rotation_xyz_microunits: [100000, 200000, 300000],
        };
        let p = InterpolatedTransform::between(&a, &b, 0.5).unwrap();
        for (ist, soll) in p.position.iter().zip([0.5f32, 1.0, 1.5]) {
            assert!(
                (ist - soll).abs() < 1e-5,
                "Interpolation weicht ab: {ist} != {soll}"
            );
        }
        assert_eq!(p.rotation, [0.05, 0.1, 0.15])
    }
    #[test]
    fn invalid_rotation_blocks_interpolation() {
        let a = state();
        let mut b = state();
        b.tick = Tick(9);
        b.rotation_xyz_microunits = [1000001, 0, 0];
        assert!(InterpolatedTransform::between(&a, &b, 0.5).is_none())
    }
    #[test]
    fn prediction() {
        let mut p = PredictionBuffer::with_capacity(4);
        p.push(Tick(1), [1.0, 0.0, 0.0]);
        p.push(Tick(2), [2.0, 0.0, 0.0]);
        assert_eq!(p.inputs_after(Tick(1)).count(), 1)
    }
    #[test]
    fn packet_rejects_replay() {
        let mut r = Replicator::new(LoopbackTransport::default());
        r.publish(&state()).unwrap();
        let p = r.transport.drain().pop().unwrap();
        assert!(r.accept_packet(&p).is_some());
        assert!(r.accept_packet(&p).is_none())
    }
    #[test]
    fn packet_rejects_trailing_bytes() {
        let mut r = Replicator::new(LoopbackTransport::default());
        r.publish(&state()).unwrap();
        let mut p = r.transport.drain().pop().unwrap();
        p.push(0);
        assert!(r.accept_packet(&p).is_none())
    }
    #[test]
    fn packet_rejects_stale_tick() {
        let mut r = Replicator::new(LoopbackTransport::default());
        let mut first = state();
        first.tick = Tick(10);
        r.publish(&first).unwrap();
        let p1 = r.transport.drain().pop().unwrap();
        assert!(r.accept_packet(&p1).is_some());
        let mut stale = state();
        stale.tick = Tick(9);
        r.publish(&stale).unwrap();
        let p2 = r.transport.drain().pop().unwrap();
        assert!(r.accept_packet(&p2).is_none())
    }
}
