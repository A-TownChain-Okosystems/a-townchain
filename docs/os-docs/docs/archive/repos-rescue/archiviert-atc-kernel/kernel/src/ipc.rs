//! ShivaCore Kernel — Inter-Process Communication (Rust).
//!
//! Channel-basierte IPC mit Capability-Durchsetzung.
//! Prozesse kommunizieren ueber Channels, aber nur wenn sie die
//! entsprechenden Capabilities (READ zum Empfangen, WRITE zum Senden)
//! besitzen. Der Kernel prueft jede Operation.
//!
//! Eigenschaften:
//! - Capability-gegated: send/recv nur mit gueltiger Cap
//! - Synchron: send() blockiert bis Receiver bereit (hier: Fehler wenn leer/voll)
//! - Isoliert: ein Prozess kann nicht auf Channels anderer zugreifen
//! - Automatisch: kill() schliesst alle Channels eines Prozesses

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

use crate::capability::{CapabilityTable, CapId, Pid, ResourceType, Rights, CapabilityError};

/// Eindeutige Channel-ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ChannelId(pub u64);

/// Message-Typ
#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub sender: Pid,
    pub data: Vec<u8>,
    pub timestamp: u64,
}

/// IPC-Channel — unidirektional (Sender → Empfaenger)
pub struct Channel {
    pub id: ChannelId,
    pub owner: Pid,           // Empfaenger-Prozess
    pub sender_cap: Option<CapId>,  // Cap die Senden erlaubt
    pub recv_cap: Option<CapId>,    // Cap die Empfangen erlaubt
    pub buffer: Vec<Message>,
    pub capacity: usize,
    pub closed: bool,
}

/// IPC-Subsystem — verwaltet alle Channels
pub struct IpcSubsystem {
    channels: BTreeMap<ChannelId, Channel>,
    next_channel_id: AtomicU64,
}

impl IpcSubsystem {
    pub fn new() -> Self {
        Self {
            channels: BTreeMap::new(),
            next_channel_id: AtomicU64::new(1),
        }
    }

    /// Erzeugt einen neuen IPC-Channel fuer einen Prozess.
    /// Der Owner (Empfaenger) bekommt automatisch READ+WRITE+DELEGATE Capabilities.
    pub fn create_channel(
        &mut self,
        caps: &mut CapabilityTable,
        owner: Pid,
        capacity: usize,
    ) -> ChannelId {
        let id = ChannelId(self.next_channel_id.fetch_add(1, Ordering::SeqCst));
        let resource_id = id.0;

        // Owner bekommt WRITE (senden) und READ (empfangen) + DELEGATE
        let send_cap = caps.create(owner, ResourceType::IpcChannel, resource_id,
            Rights::WRITE | Rights::DELEGATE);
        let recv_cap = caps.create(owner, ResourceType::IpcChannel, resource_id,
            Rights::READ | Rights::DELEGATE);

        let channel = Channel {
            id,
            owner,
            sender_cap: Some(send_cap),
            recv_cap: Some(recv_cap),
            buffer: Vec::new(),
            capacity,
            closed: false,
        };
        self.channels.insert(id, channel);
        id
    }

    /// Sendet eine Message ueber einen Channel.
    /// Prueft: Channel existiert, nicht geschlossen, nicht voll,
    /// Sender hat WRITE-Capability fuer diesen Channel.
    pub fn send(
        &mut self,
        caps: &CapabilityTable,
        sender: Pid,
        channel_id: ChannelId,
        data: Vec<u8>,
    ) -> Result<(), IpcError> {
        let channel = self.channels.get_mut(&channel_id)
            .ok_or(IpcError::ChannelNotFound)?;

        if channel.closed {
            return Err(IpcError::ChannelClosed);
        }
        if channel.buffer.len() >= channel.capacity {
            return Err(IpcError::ChannelFull);
        }

        // Capability-Check: Sender muss WRITE haben
        if !caps.check(sender, ResourceType::IpcChannel, channel_id.0, Rights::WRITE) {
            return Err(IpcError::NoWriteCapability);
        }

        let msg = Message {
            sender,
            data,
            timestamp: 0, // Im echten Kernel: Kernel-Timer
        };
        channel.buffer.push(msg);
        Ok(())
    }

    /// Empfaengt eine Message von einem Channel (FIFO).
    /// Prueft: Channel existiert, nicht leer,
    /// Empfaenger hat READ-Capability fuer diesen Channel.
    pub fn recv(
        &mut self,
        caps: &CapabilityTable,
        receiver: Pid,
        channel_id: ChannelId,
    ) -> Result<Message, IpcError> {
        let channel = self.channels.get_mut(&channel_id)
            .ok_or(IpcError::ChannelNotFound)?;

        if channel.closed {
            return Err(IpcError::ChannelClosed);
        }

        // Capability-Check VOR Buffer-Inspektion (Security: keine Info-Lecks an Unbefugte)
        if !caps.check(receiver, ResourceType::IpcChannel, channel_id.0, Rights::READ) {
            return Err(IpcError::NoReadCapability);
        }

        if channel.buffer.is_empty() {
            return Err(IpcError::ChannelEmpty);
        }

        Ok(channel.buffer.remove(0)) // FIFO
    }

    /// Schliesst einen Channel und widerruft alle Capabilities.
    pub fn close_channel(&mut self, caps: &mut CapabilityTable, owner: Pid, channel_id: ChannelId) -> bool {
        let channel = match self.channels.get_mut(&channel_id) {
            Some(c) => c,
            None => return false,
        };
        if channel.owner != owner { return false; }

        channel.closed = true;

        // Alle Caps fuer diese Resource widerrufen (kaskadierend)
        let cap_ids: Vec<CapId> = caps.list_for(owner).iter()
            .filter(|c| c.resource_type == ResourceType::IpcChannel && c.resource_id == channel_id.0)
            .map(|c| c.id)
            .collect();
        for cap_id in cap_ids {
            caps.revoke(cap_id);
        }
        true
    }

    /// Schliesst alle Channels eines Prozesses (wird von kill() aufgerufen)
    pub fn close_all_for(&mut self, caps: &mut CapabilityTable, pid: Pid) -> usize {
        let to_close: Vec<ChannelId> = self.channels.values()
            .filter(|c| c.owner == pid && !c.closed)
            .map(|c| c.id)
            .collect();
        let count = to_close.len();
        for ch_id in to_close {
            self.close_channel(caps, pid, ch_id);
        }
        count
    }

    /// Erlaubt einem anderen Prozess, auf den Channel zuzugreifen,
    /// durch Delegation der entsprechenden Capability.
    pub fn grant_access(
        &mut self,
        caps: &mut CapabilityTable,
        owner: Pid,
        channel_id: ChannelId,
        target: Pid,
        rights: Rights,
    ) -> Result<CapId, CapabilityError> {
        let channel = self.channels.get(&channel_id)
            .ok_or(CapabilityError::NotFound)?;
        if channel.owner != owner {
            return Err(CapabilityError::NotOwner);
        }

        // Finde die entsprechende Cap des Owners
        let owner_caps: Vec<CapId> = caps.list_for(owner).iter()
            .filter(|c| c.resource_type == ResourceType::IpcChannel
                && c.resource_id == channel_id.0
                && c.rights.has(rights))
            .map(|c| c.id)
            .collect();

        let source_cap = owner_caps.first()
            .ok_or(CapabilityError::NoDelegateRight)?;

        caps.delegate(owner, *source_cap, target, rights)
    }

    pub fn channel_count(&self) -> usize { self.channels.len() }
    pub fn pending_messages(&self, channel_id: ChannelId) -> Option<usize> {
        self.channels.get(&channel_id).map(|c| c.buffer.len())
    }
}

/// IPC-Fehler
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpcError {
    ChannelNotFound,
    ChannelClosed,
    ChannelFull,
    ChannelEmpty,
    NoWriteCapability,
    NoReadCapability,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pid(n: u32) -> Pid { Pid(n) }

    #[test]
    fn test_create_channel_and_send_recv() {
        let mut caps = CapabilityTable::new();
        let mut ipc = IpcSubsystem::new();

        let ch = ipc.create_channel(&mut caps, pid(1), 16);
        assert!(ch.0 > 0);

        // Owner kann senden und empfangen
        ipc.send(&caps, pid(1), ch, b"hello".to_vec()).unwrap();
        let msg = ipc.recv(&caps, pid(1), ch).unwrap();
        assert_eq!(msg.data, b"hello");
        assert_eq!(msg.sender, pid(1));
    }

    #[test]
    fn test_send_without_write_cap_rejected() {
        let mut caps = CapabilityTable::new();
        let mut ipc = IpcSubsystem::new();

        let ch = ipc.create_channel(&mut caps, pid(1), 16);
        // pid(2) hat keine Cap fuer diesen Channel
        let result = ipc.send(&caps, pid(2), ch, b"hack".to_vec());
        assert_eq!(result, Err(IpcError::NoWriteCapability));
    }

    #[test]
    fn test_recv_without_read_cap_rejected() {
        let mut caps = CapabilityTable::new();
        let mut ipc = IpcSubsystem::new();

        let ch = ipc.create_channel(&mut caps, pid(1), 16);
        ipc.send(&caps, pid(1), ch, b"data".to_vec()).unwrap();

        // pid(2) hat keine READ-Cap
        let result = ipc.recv(&caps, pid(2), ch);
        assert_eq!(result, Err(IpcError::NoReadCapability));
    }

    #[test]
    fn test_grant_access_and_cross_process_comm() {
        let mut caps = CapabilityTable::new();
        let mut ipc = IpcSubsystem::new();

        let ch = ipc.create_channel(&mut caps, pid(1), 16);

        // pid(1) delegiert WRITE an pid(2), pid(2) kann senden
        ipc.grant_access(&mut caps, pid(1), ch, pid(2), Rights::WRITE | Rights::DELEGATE).unwrap();
        ipc.send(&caps, pid(2), ch, b"from p2".to_vec()).unwrap();

        // pid(1) delegiert READ an pid(3), pid(3) kann empfangen
        ipc.grant_access(&mut caps, pid(1), ch, pid(3), Rights::READ).unwrap();
        let msg = ipc.recv(&caps, pid(3), ch).unwrap();
        assert_eq!(msg.data, b"from p2");
        assert_eq!(msg.sender, pid(2));
    }

    #[test]
    fn test_channel_full() {
        let mut caps = CapabilityTable::new();
        let mut ipc = IpcSubsystem::new();

        let ch = ipc.create_channel(&mut caps, pid(1), 2);
        ipc.send(&caps, pid(1), ch, b"a".to_vec()).unwrap();
        ipc.send(&caps, pid(1), ch, b"b".to_vec()).unwrap();
        // Dritte Message: Channel voll
        let result = ipc.send(&caps, pid(1), ch, b"c".to_vec());
        assert_eq!(result, Err(IpcError::ChannelFull));
    }

    #[test]
    fn test_recv_empty_channel() {
        let mut caps = CapabilityTable::new();
        let mut ipc = IpcSubsystem::new();

        let ch = ipc.create_channel(&mut caps, pid(1), 16);
        let result = ipc.recv(&caps, pid(1), ch);
        assert_eq!(result, Err(IpcError::ChannelEmpty));
    }

    #[test]
    fn test_close_channel() {
        let mut caps = CapabilityTable::new();
        let mut ipc = IpcSubsystem::new();

        let ch = ipc.create_channel(&mut caps, pid(1), 16);
        assert!(ipc.close_channel(&mut caps, pid(1), ch));

        // Channel ist geschlossen
        let result = ipc.send(&caps, pid(1), ch, b"x".to_vec());
        assert_eq!(result, Err(IpcError::ChannelClosed));
    }

    #[test]
    fn test_close_rejects_non_owner() {
        let mut caps = CapabilityTable::new();
        let mut ipc = IpcSubsystem::new();

        let ch = ipc.create_channel(&mut caps, pid(1), 16);
        // pid(2) versucht, pid(1)s Channel zu schliessen
        assert!(!ipc.close_channel(&mut caps, pid(2), ch));
    }

    #[test]
    fn test_close_all_for_process() {
        let mut caps = CapabilityTable::new();
        let mut ipc = IpcSubsystem::new();

        let ch1 = ipc.create_channel(&mut caps, pid(1), 16);
        let ch2 = ipc.create_channel(&mut caps, pid(1), 16);
        let ch3 = ipc.create_channel(&mut caps, pid(2), 16);

        let closed = ipc.close_all_for(&mut caps, pid(1));
        assert_eq!(closed, 2);

        // pid(1)s Channels sind zu
        assert_eq!(ipc.send(&caps, pid(1), ch1, b"x".to_vec()), Err(IpcError::ChannelClosed));
        assert_eq!(ipc.send(&caps, pid(1), ch2, b"x".to_vec()), Err(IpcError::ChannelClosed));
        // pid(2)s Channel ist noch offen
        assert!(ipc.send(&caps, pid(2), ch3, b"x".to_vec()).is_ok());
    }

    #[test]
    fn test_grant_access_wrong_owner_rejected() {
        let mut caps = CapabilityTable::new();
        let mut ipc = IpcSubsystem::new();

        let ch = ipc.create_channel(&mut caps, pid(1), 16);
        // pid(2) versucht, Zugang zu gewaehren, ist aber nicht Owner
        let result = ipc.grant_access(&mut caps, pid(2), ch, pid(3), Rights::WRITE);
        assert_eq!(result, Err(CapabilityError::NotOwner));
    }

    #[test]
    fn test_fifo_order() {
        let mut caps = CapabilityTable::new();
        let mut ipc = IpcSubsystem::new();

        let ch = ipc.create_channel(&mut caps, pid(1), 16);
        ipc.send(&caps, pid(1), ch, b"first".to_vec()).unwrap();
        ipc.send(&caps, pid(1), ch, b"second".to_vec()).unwrap();
        ipc.send(&caps, pid(1), ch, b"third".to_vec()).unwrap();

        let m1 = ipc.recv(&caps, pid(1), ch).unwrap();
        let m2 = ipc.recv(&caps, pid(1), ch).unwrap();
        let m3 = ipc.recv(&caps, pid(1), ch).unwrap();
        assert_eq!(m1.data, b"first");
        assert_eq!(m2.data, b"second");
        assert_eq!(m3.data, b"third");
    }

    #[test]
    fn test_pending_messages_count() {
        let mut caps = CapabilityTable::new();
        let mut ipc = IpcSubsystem::new();

        let ch = ipc.create_channel(&mut caps, pid(1), 16);
        assert_eq!(ipc.pending_messages(ch), Some(0));
        ipc.send(&caps, pid(1), ch, b"a".to_vec()).unwrap();
        assert_eq!(ipc.pending_messages(ch), Some(1));
        ipc.recv(&caps, pid(1), ch).unwrap();
        assert_eq!(ipc.pending_messages(ch), Some(0));
    }

    // --- Capability-Gating Edge Cases ---

    #[test]
    fn test_revoked_cap_blocks_send() {
        // Wenn eine Capability widerrufen wird, kann der Prozess nicht mehr senden
        let mut caps = CapabilityTable::new();
        let mut ipc = IpcSubsystem::new();

        let ch = ipc.create_channel(&mut caps, pid(1), 16);
        ipc.grant_access(&mut caps, pid(1), ch, pid(2), Rights::WRITE | Rights::DELEGATE).unwrap();

        // pid(2) kann zunaechst senden
        assert!(ipc.send(&caps, pid(2), ch, b"first".to_vec()).is_ok());

        // Widerrufe pid(2)s WRITE-Cap
        let p2_caps: Vec<CapId> = caps.list_for(pid(2)).into_iter()
            .filter(|c| c.resource_type == ResourceType::IpcChannel && c.resource_id == ch.0)
            .map(|c| c.id)
            .collect();
        for cap_id in p2_caps {
            caps.revoke(cap_id);
        }

        // pid(2) kann nicht mehr senden
        let result = ipc.send(&caps, pid(2), ch, b"blocked".to_vec());
        assert_eq!(result, Err(IpcError::NoWriteCapability));
    }

    #[test]
    fn test_revoked_cap_blocks_recv() {
        // Wenn READ-Cap widerrufen wird, kann der Prozess nicht mehr empfangen
        let mut caps = CapabilityTable::new();
        let mut ipc = IpcSubsystem::new();

        let ch = ipc.create_channel(&mut caps, pid(1), 16);
        ipc.grant_access(&mut caps, pid(1), ch, pid(3), Rights::READ).unwrap();

        // Message ablegen
        ipc.send(&caps, pid(1), ch, b"msg".to_vec()).unwrap();

        // pid(3) kann empfangen
        assert!(ipc.recv(&caps, pid(3), ch).is_ok());

        // Weitere Message + READ-Cap widerrufen
        ipc.send(&caps, pid(1), ch, b"msg2".to_vec()).unwrap();
        let p3_caps: Vec<CapId> = caps.list_for(pid(3)).into_iter()
            .filter(|c| c.resource_type == ResourceType::IpcChannel && c.resource_id == ch.0)
            .map(|c| c.id)
            .collect();
        for cap_id in p3_caps {
            caps.revoke(cap_id);
        }

        // pid(3) kann nicht mehr empfangen
        let result = ipc.recv(&caps, pid(3), ch);
        assert_eq!(result, Err(IpcError::NoReadCapability));
    }

    #[test]
    fn test_attenuated_cap_cannot_exceed_original() {
        // Owner delegiert nur READ — Target kann nicht WRITE erlangen
        let mut caps = CapabilityTable::new();
        let mut ipc = IpcSubsystem::new();

        let ch = ipc.create_channel(&mut caps, pid(1), 16);

        // pid(1) delegiert nur READ an pid(2)
        ipc.grant_access(&mut caps, pid(1), ch, pid(2), Rights::READ).unwrap();

        // pid(2) kann empfangen aber nicht senden
        ipc.send(&caps, pid(1), ch, b"data".to_vec()).unwrap();
        assert!(ipc.recv(&caps, pid(2), ch).is_ok());
        assert_eq!(ipc.send(&caps, pid(2), ch, b"hack".to_vec()), Err(IpcError::NoWriteCapability));
    }

    #[test]
    fn test_delegation_chain_capability_gating() {
        // Alice -> Bob (WRITE+DELEGATE) -> Charlie (WRITE only)
        // Charlie kann senden, Bob kann senden, aber Charlie kann nicht weiter delegieren
        let mut caps = CapabilityTable::new();
        let mut ipc = IpcSubsystem::new();

        let ch = ipc.create_channel(&mut caps, pid(1), 16); // Alice

        // Alice delegiert WRITE+DELEGATE an Bob
        let bob_cap = ipc.grant_access(&mut caps, pid(1), ch, pid(2),
            Rights::WRITE | Rights::DELEGATE).unwrap();

        // Bob delegiert nur WRITE an Charlie (Attenuation)
        let charlie_cap = caps.delegate(pid(2), bob_cap, pid(3), Rights::WRITE).unwrap();

        // Charlie kann senden
        assert!(ipc.send(&caps, pid(3), ch, b"from charlie".to_vec()).is_ok());

        // Charlie kann aber nicht weiter delegieren (kein DELEGATE-Recht)
        let result = caps.delegate(pid(3), charlie_cap, pid(4), Rights::WRITE);
        assert_eq!(result, Err(CapabilityError::NoDelegateRight));
    }

    #[test]
    fn test_close_channel_revokes_all_delegated_caps() {
        // Channel schliessen widerruft alle Caps — auch delegierte an andere Prozesse
        let mut caps = CapabilityTable::new();
        let mut ipc = IpcSubsystem::new();

        let ch = ipc.create_channel(&mut caps, pid(1), 16);
        ipc.grant_access(&mut caps, pid(1), ch, pid(2), Rights::WRITE | Rights::DELEGATE).unwrap();
        ipc.grant_access(&mut caps, pid(1), ch, pid(3), Rights::READ).unwrap();

        // Bevor schliessen: alle koennen zugreifen
        assert!(caps.check(pid(1), ResourceType::IpcChannel, ch.0, Rights::WRITE));
        assert!(caps.check(pid(2), ResourceType::IpcChannel, ch.0, Rights::WRITE));
        assert!(caps.check(pid(3), ResourceType::IpcChannel, ch.0, Rights::READ));

        // Channel schliessen
        ipc.close_channel(&mut caps, pid(1), ch);

        // Nach schliessen: niemand hat noch Caps fuer diesen Channel
        assert!(!caps.check(pid(1), ResourceType::IpcChannel, ch.0, Rights::WRITE));
        assert!(!caps.check(pid(2), ResourceType::IpcChannel, ch.0, Rights::WRITE));
        assert!(!caps.check(pid(3), ResourceType::IpcChannel, ch.0, Rights::READ));
    }

    #[test]
    fn test_isolated_channels_capability_gating() {
        // Prozesse koennen nur auf Channels zugreifen fuer die sie Caps haben
        let mut caps = CapabilityTable::new();
        let mut ipc = IpcSubsystem::new();

        let ch1 = ipc.create_channel(&mut caps, pid(1), 16);
        let ch2 = ipc.create_channel(&mut caps, pid(2), 16);

        // pid(1) kann auf ch1 senden aber nicht auf ch2
        assert!(ipc.send(&caps, pid(1), ch1, b"ok".to_vec()).is_ok());
        assert_eq!(ipc.send(&caps, pid(1), ch2, b"no".to_vec()), Err(IpcError::NoWriteCapability));

        // pid(2) kann auf ch2 senden aber nicht auf ch1
        assert!(ipc.send(&caps, pid(2), ch2, b"ok".to_vec()).is_ok());
        assert_eq!(ipc.send(&caps, pid(2), ch1, b"no".to_vec()), Err(IpcError::NoWriteCapability));
    }

    #[test]
    fn test_grant_then_revoke_blocks_access() {
        // Grant access, dann widerruf, Zugriff sollte blockiert sein
        let mut caps = CapabilityTable::new();
        let mut ipc = IpcSubsystem::new();

        let ch = ipc.create_channel(&mut caps, pid(1), 16);

        // Grant WRITE an pid(2)
        let cap = ipc.grant_access(&mut caps, pid(1), ch, pid(2), Rights::WRITE).unwrap();
        assert!(ipc.send(&caps, pid(2), ch, b"ok".to_vec()).is_ok());

        // Widerrufe die spezifische Cap
        caps.revoke(cap);
        assert_eq!(ipc.send(&caps, pid(2), ch, b"blocked".to_vec()), Err(IpcError::NoWriteCapability));
    }

    #[test]
    fn test_cross_channel_capability_isolation() {
        // Caps fuer Channel A geben keinen Zugriff auf Channel B
        let mut caps = CapabilityTable::new();
        let mut ipc = IpcSubsystem::new();

        let ch_a = ipc.create_channel(&mut caps, pid(1), 16);
        let ch_b = ipc.create_channel(&mut caps, pid(1), 16);

        // Grant WRITE auf ch_a an pid(2)
        ipc.grant_access(&mut caps, pid(1), ch_a, pid(2), Rights::WRITE).unwrap();

        // pid(2) kann auf ch_a senden
        assert!(ipc.send(&caps, pid(2), ch_a, b"a".to_vec()).is_ok());

        // pid(2) kann NICHT auf ch_b senden (keine Cap)
        assert_eq!(ipc.send(&caps, pid(2), ch_b, b"b".to_vec()), Err(IpcError::NoWriteCapability));

        // pid(2) kann auch nicht von ch_b empfangen
        assert_eq!(ipc.recv(&caps, pid(2), ch_b), Err(IpcError::NoReadCapability));
    }

    #[test]
    fn test_send_to_nonexistent_channel() {
        let mut caps = CapabilityTable::new();
        let mut ipc = IpcSubsystem::new();

        let result = ipc.send(&caps, pid(1), ChannelId(999), b"x".to_vec());
        assert_eq!(result, Err(IpcError::ChannelNotFound));
    }

    #[test]
    fn test_recv_from_nonexistent_channel() {
        let mut caps = CapabilityTable::new();
        let mut ipc = IpcSubsystem::new();

        let result = ipc.recv(&caps, pid(1), ChannelId(999));
        assert_eq!(result, Err(IpcError::ChannelNotFound));
    }

}
