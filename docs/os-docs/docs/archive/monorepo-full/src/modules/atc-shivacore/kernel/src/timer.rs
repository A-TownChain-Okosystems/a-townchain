// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ─────────────────────────────────────────────────────────────────────────
// K-Sprint 10 — Timer/Clock-Subsystem
// Kernel Layer | Chain-ID 658467
// Monotone Uhr, Deadline-Tracking, Sleep-Queue für den Scheduler.
// Trait-basiert: HPET/PIT in Hardware, SimulatedTimerSource für Tests.
// ─────────────────────────────────────────────────────────────────────────

use alloc::format;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use alloc::string::String;
use alloc::string::ToString;
use spin::Mutex;

// ─── Timer-Source Trait ────────────────────────────────────────────────────

/// Abstraktion für eine Zeitquelle (HPET, PIT, TSC, oder simuliert).
pub trait TimerSource: Send + Sync {
    /// Liefert die aktuelle Zeit in Nanosekunden (monoton steigend).
    fn now_ns(&self) -> u64;

    /// Liefert die Timer-Frequenz in Hz (Ticks pro Sekunde).
    fn frequency(&self) -> u64;

    /// Liefert die Aufösung in Nanosekunden pro Tick.
    fn resolution_ns(&self) -> u64 {
        1_000_000_000 / self.frequency().max(1)
    }
}

// ─── Simulierte Zeitquelle (für Tests und Software-Validierung) ─────────────

pub struct SimulatedTimerSource {
    current_ns: Mutex<u64>,
    freq: u64,
}

impl SimulatedTimerSource {
    pub fn new(freq: u64) -> Self {
        SimulatedTimerSource {
            current_ns: Mutex::new(0),
            freq,
        }
    }

    /// Erhoeht die simulierte Zeit um n Nanosekunden.
    pub fn advance(&self, ns: u64) {
        let mut t = self.current_ns.lock();
        *t += ns;
    }

    /// Setzt die simulierte Zeit auf einen festen Wert.
    pub fn set(&self, ns: u64) {
        let mut t = self.current_ns.lock();
        *t = ns;
    }
}

impl TimerSource for SimulatedTimerSource {
    fn now_ns(&self) -> u64 {
        *self.current_ns.lock()
    }

    fn frequency(&self) -> u64 {
        self.freq
    }
}

// ─── MonotonicClock ─────────────────────────────────────────────────────────

/// Monotone Uhr — kapselt eine TimerSource und bietet Convenience-Methoden.
pub struct MonotonicClock {
    source: &'static dyn TimerSource,
    boot_ns: u64,
}

impl MonotonicClock {
    pub fn new(source: &'static dyn TimerSource) -> Self {
        let boot_ns = source.now_ns();
        MonotonicClock { source, boot_ns }
    }

    /// Uptime in Nanosekunden seit Boot.
    pub fn uptime_ns(&self) -> u64 {
        self.source.now_ns().saturating_sub(self.boot_ns)
    }

    /// Uptime in Millisekunden.
    pub fn uptime_ms(&self) -> u64 {
        self.uptime_ns() / 1_000_000
    }

    /// Uptime in Sekunden.
    pub fn uptime_secs(&self) -> u64 {
        self.uptime_ns() / 1_000_000_000
    }

    /// Aktuelle Zeit in Nanosekunden (absolut, nicht relativ zum Boot).
    pub fn now_ns(&self) -> u64 {
        self.source.now_ns()
    }

    /// Formatierte Uptime als String (z.B. "12.345s").
    pub fn uptime_string(&self) -> String {
        let secs = self.uptime_secs();
        let ms = (self.uptime_ms() % 1000) as u32;
        format!("{}.{:03}s", secs, ms)
    }
}

// ─── Sleep-Queue / Deadline-Tracking ────────────────────────────────────────

/// Ein registrierter Timer-Callback mit Deadline.
#[derive(Clone, Debug)]
pub struct TimerEvent {
    pub id: u64,
    pub deadline_ns: u64,
    pub pid: u64,
    pub callback_type: TimerCallback,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TimerCallback {
    /// Weckt einen Prozess auf (für sleep()).
    Wakeup(u64),
    /// Periodischer Timer (Intervall in ns).
    Periodic(u64),
    /// Einmaliger Alarm.
    Alarm,
}

/// Verwaltet alle Timer-Events, sortiert nach Deadline.
pub struct TimerManager {
    events: BTreeMap<u64, TimerEvent>,  // deadline_ns -> event
    next_id: u64,
    source: &'static dyn TimerSource,
}

impl TimerManager {
    pub fn new(source: &'static dyn TimerSource) -> Self {
        TimerManager {
            events: BTreeMap::new(),
            next_id: 1,
            source,
        }
    }

    /// Registriert einen Sleep-Timer für einen Prozess.
    pub fn sleep(&mut self, pid: u64, duration_ns: u64) -> u64 {
        let deadline = self.source.now_ns() + duration_ns;
        let id = self.next_id;
        self.next_id += 1;
        let event = TimerEvent {
            id,
            deadline_ns: deadline,
            pid,
            callback_type: TimerCallback::Wakeup(pid),
        };
        self.events.insert(deadline, event);
        id
    }

    /// Registriert einen periodischen Timer.
    pub fn schedule_periodic(&mut self, pid: u64, interval_ns: u64) -> u64 {
        let deadline = self.source.now_ns() + interval_ns;
        let id = self.next_id;
        self.next_id += 1;
        let event = TimerEvent {
            id,
            deadline_ns: deadline,
            pid,
            callback_type: TimerCallback::Periodic(interval_ns),
        };
        self.events.insert(deadline, event);
        id
    }

    /// Registriert einen einmaligen Alarm.
    pub fn schedule_alarm(&mut self, pid: u64, delay_ns: u64) -> u64 {
        let deadline = self.source.now_ns() + delay_ns;
        let id = self.next_id;
        self.next_id += 1;
        let event = TimerEvent {
            id,
            deadline_ns: deadline,
            pid,
            callback_type: TimerCallback::Alarm,
        };
        self.events.insert(deadline, event);
        id
    }

    /// Bricht einen Timer ab. (Sucht nach der event-ID.)
    pub fn cancel(&mut self, event_id: u64) -> bool {
        let key = self.events.iter()
            .find(|(_, e)| e.id == event_id)
            .map(|(&k, _)| k);
        match key {
            Some(k) => self.events.remove(&k).is_some(),
            None => false,
        }
    }

    /// Prüft alle Timer und liefert die fired events (Deadlines die erreicht wurden).
    /// Bei periodischen Timern wird der nächste Tick automatisch eingetragen.
    pub fn tick(&mut self) -> Vec<TimerEvent> {
        let now = self.source.now_ns();
        let mut fired = Vec::new();

        loop {
            let expired_keys: Vec<u64> = self.events.keys()
                .filter(|&&deadline| deadline <= now)
                .copied()
                .collect();

            if expired_keys.is_empty() { break; }

            for key in expired_keys {
                if let Some(event) = self.events.remove(&key) {
                    if let TimerCallback::Periodic(interval) = event.callback_type {
                        let next_deadline = event.deadline_ns + interval;
                        let mut next_event = event.clone();
                        next_event.deadline_ns = next_deadline;
                        self.events.insert(next_deadline, next_event);
                    }
                    fired.push(event);
                }
            }
        }

        fired.sort_by_key(|e| e.deadline_ns);
        fired
    }

    /// Anzahl der aktiven Timer.
    pub fn active_count(&self) -> usize {
        self.events.len()
    }

    /// Nächste Deadline in Nanosekunden (oder None wenn keine Timer aktiv).
    pub fn next_deadline(&self) -> Option<u64> {
        self.events.keys().next().copied()
    }

    /// Zeit bis zur nächsten Deadline in Nanosekunden.
    pub fn time_to_next_deadline(&self) -> Option<u64> {
        self.next_deadline().map(|d| d.saturating_sub(self.source.now_ns()))
    }
}

// ─── Convenience: Duration-Hilfsfunktionen ──────────────────────────────────

pub mod duration {
    pub const NS_PER_US: u64 = 1_000;
    pub const NS_PER_MS: u64 = 1_000_000;
    pub const NS_PER_SEC: u64 = 1_000_000_000;
    pub const NS_PER_MIN: u64 = 60 * NS_PER_SEC;
    pub const NS_PER_HOUR: u64 = 60 * NS_PER_MIN;

    pub fn from_ms(ms: u64) -> u64 { ms * NS_PER_MS }
    pub fn from_secs(secs: u64) -> u64 { secs * NS_PER_SEC }
    pub fn from_us(us: u64) -> u64 { us * NS_PER_US }
    pub fn from_mins(mins: u64) -> u64 { mins * NS_PER_MIN }

    pub fn to_ms(ns: u64) -> u64 { ns / NS_PER_MS }
    pub fn to_secs(ns: u64) -> u64 { ns / NS_PER_SEC }
    pub fn to_us(ns: u64) -> u64 { ns / NS_PER_US }
}

// ─── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_source() -> &'static SimulatedTimerSource {
        // Leaked für &'static — nur in Tests akzeptabel
        Box::leak(Box::new(SimulatedTimerSource::new(1_000_000_000))) // 1 GHz
    }

    // ── SimulatedTimerSource ────────────────────────────────────────────────

    #[test]
    fn test_simulated_source_starts_at_zero() {
        let src = make_source();
        assert_eq!(src.now_ns(), 0);
        assert_eq!(src.frequency(), 1_000_000_000);
        assert_eq!(src.resolution_ns(), 1);
    }

    #[test]
    fn test_simulated_source_advance() {
        let src = make_source();
        src.advance(500_000_000); // 500ms
        assert_eq!(src.now_ns(), 500_000_000);
        src.advance(500_000_000); // +500ms
        assert_eq!(src.now_ns(), 1_000_000_000); // 1s
    }

    #[test]
    fn test_simulated_source_set() {
        let src = make_source();
        src.set(42_000_000_000);
        assert_eq!(src.now_ns(), 42_000_000_000);
    }

    // ── MonotonicClock ──────────────────────────────────────────────────────

    #[test]
    fn test_clock_uptime_from_zero() {
        let src = make_source();
        let clock = MonotonicClock::new(src);
        assert_eq!(clock.uptime_ns(), 0);
        src.advance(1_500_000_000); // 1.5s
        assert_eq!(clock.uptime_ns(), 1_500_000_000);
        assert_eq!(clock.uptime_ms(), 1500);
        assert_eq!(clock.uptime_secs(), 1);
    }

    #[test]
    fn test_clock_uptime_string() {
        let src = make_source();
        let clock = MonotonicClock::new(src);
        src.advance(12_345_000_000); // 12.345s
        assert_eq!(clock.uptime_string(), "12.345s");
    }

    #[test]
    fn test_clock_now_vs_uptime() {
        let src = make_source();
        src.set(100_000_000_000); // Start at 100s
        let clock = MonotonicClock::new(src);
        src.advance(5_000_000_000); // +5s
        assert_eq!(clock.now_ns(), 105_000_000_000);
        assert_eq!(clock.uptime_ns(), 5_000_000_000);
    }

    // ── TimerManager — Sleep ─────────────────────────────────────────────────

    #[test]
    fn test_sleep_registers_deadline() {
        let src = make_source();
        let mut tm = TimerManager::new(src);
        let id = tm.sleep(1, 1_000_000_000); // pid=1, 1s sleep
        assert_eq!(tm.active_count(), 1);
        assert!(tm.next_deadline().is_some());
    }

    #[test]
    fn test_sleep_fires_after_duration() {
        let src = make_source();
        let mut tm = TimerManager::new(src);
        tm.sleep(1, 1_000_000_000); // 1s

        // Vor Ablauf: nichts fired
        src.advance(999_000_000); // 999ms
        let fired = tm.tick();
        assert_eq!(fired.len(), 0);

        // Nach Ablauf: fired
        src.advance(2_000_000); // +2ms → 1001ms total
        let fired = tm.tick();
        assert_eq!(fired.len(), 1);
        assert_eq!(fired[0].pid, 1);
        assert!(matches!(fired[0].callback_type, TimerCallback::Wakeup(1)));
    }

    #[test]
    fn test_multiple_timers_ordered_by_deadline() {
        let src = make_source();
        let mut tm = TimerManager::new(src);
        tm.sleep(1, 3_000_000_000); // 3s
        tm.sleep(2, 1_000_000_000); // 1s
        tm.sleep(3, 2_000_000_000); // 2s

        src.advance(2_500_000_000); // 2.5s
        let fired = tm.tick();
        assert_eq!(fired.len(), 2);
        assert_eq!(fired[0].pid, 2); // 1s deadline first
        assert_eq!(fired[1].pid, 3); // 2s deadline second
    }

    // ── TimerManager — Periodic ─────────────────────────────────────────────

    #[test]
    fn test_periodic_timer_refires() {
        let src = make_source();
        let mut tm = TimerManager::new(src);
        let _id = tm.schedule_periodic(1, 1_000_000_000); // 1s interval

        // Erster Tick
        src.advance(1_000_000_000); // 1s
        let fired = tm.tick();
        assert_eq!(fired.len(), 1);
        assert!(matches!(fired[0].callback_type, TimerCallback::Periodic(_)));
        assert_eq!(tm.active_count(), 1); // re-registered

        // Zweiter Tick
        src.advance(1_000_000_000); // +1s
        let fired = tm.tick();
        assert_eq!(fired.len(), 1);
        assert_eq!(tm.active_count(), 1); // still active
    }

    #[test]
    fn test_periodic_timer_multiple_intervals() {
        let src = make_source();
        let mut tm = TimerManager::new(src);
        tm.schedule_periodic(1, 100_000_000); // 100ms interval

        src.advance(350_000_000); // 350ms
        let fired = tm.tick();
        assert_eq!(fired.len(), 3); // fires at 100, 200, 300ms
        assert_eq!(tm.active_count(), 1); // next at 400ms
    }

    // ── TimerManager — Alarm ────────────────────────────────────────────────

    #[test]
    fn test_alarm_one_shot() {
        let src = make_source();
        let mut tm = TimerManager::new(src);
        tm.schedule_alarm(1, 500_000_000); // 500ms

        src.advance(500_000_000);
        let fired = tm.tick();
        assert_eq!(fired.len(), 1);
        assert!(matches!(fired[0].callback_type, TimerCallback::Alarm));

        // Alarm ist einmalig — nicht re-registriert
        src.advance(1_000_000_000);
        let fired2 = tm.tick();
        assert_eq!(fired2.len(), 0);
        assert_eq!(tm.active_count(), 0);
    }

    // ── TimerManager — Cancel ────────────────────────────────────────────────

    #[test]
    fn test_cancel_timer() {
        let src = make_source();
        let mut tm = TimerManager::new(src);
        let id = tm.sleep(1, 1_000_000_000);

        assert_eq!(tm.active_count(), 1);
        assert!(tm.cancel(id));
        assert_eq!(tm.active_count(), 0);

        // Cancel nochmal → false
        assert!(!tm.cancel(id));
    }

    #[test]
    fn test_cancel_nonexistent() {
        let src = make_source();
        let mut tm = TimerManager::new(src);
        assert!(!tm.cancel(999));
    }

    // ── TimerManager — next_deadline ────────────────────────────────────────

    #[test]
    fn test_next_deadline() {
        let src = make_source();
        let mut tm = TimerManager::new(src);
        assert!(tm.next_deadline().is_none());
        assert!(tm.time_to_next_deadline().is_none());

        tm.sleep(1, 5_000_000_000); // 5s
        assert_eq!(tm.next_deadline(), Some(5_000_000_000));

        src.advance(2_000_000_000); // 2s
        assert_eq!(tm.time_to_next_deadline(), Some(3_000_000_000));
    }

    #[test]
    fn test_empty_timer_manager() {
        let src = make_source();
        let mut tm = TimerManager::new(src);
        assert_eq!(tm.active_count(), 0);
        assert_eq!(tm.tick().len(), 0);
    }

    // ── Duration-Hilfsfunktionen ─────────────────────────────────────────────

    #[test]
    fn test_duration_conversions() {
        assert_eq!(duration::from_ms(1), 1_000_000);
        assert_eq!(duration::from_secs(1), 1_000_000_000);
        assert_eq!(duration::from_us(1), 1_000);
        assert_eq!(duration::from_mins(1), 60_000_000_000);
        assert_eq!(duration::to_ms(1_000_000_000), 1000);
        assert_eq!(duration::to_secs(1_000_000_000), 1);
        assert_eq!(duration::to_us(1_000_000), 1000);
    }

    #[test]
    fn test_duration_constants() {
        assert_eq!(duration::NS_PER_US, 1_000);
        assert_eq!(duration::NS_PER_MS, 1_000_000);
        assert_eq!(duration::NS_PER_SEC, 1_000_000_000);
        assert_eq!(duration::NS_PER_MIN, 60 * 1_000_000_000);
        assert_eq!(duration::NS_PER_HOUR, 3600 * 1_000_000_000);
    }

    // ── Integration: Clock + Timer ──────────────────────────────────────────

    #[test]
    fn test_clock_and_timer_integration() {
        let src = make_source();
        let clock = MonotonicClock::new(src);
        let mut tm = TimerManager::new(src);

        tm.sleep(1, 2_000_000_000); // 2s sleep

        src.advance(1_000_000_000); // 1s
        assert_eq!(clock.uptime_secs(), 1);
        assert_eq!(tm.tick().len(), 0); // not yet

        src.advance(1_000_000_000); // +1s → 2s
        assert_eq!(clock.uptime_secs(), 2);
        let fired = tm.tick();
        assert_eq!(fired.len(), 1);
        assert_eq!(fired[0].pid, 1);
    }
}
