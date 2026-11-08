// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ShivaCore — K-Sprint 40: Power Management + ACPI
// Copyright (c) Michael Wroblewski. All rights reserved.
//
// ACPI-Tabellen (RSDP/RSDT/FADT/DSDT), Power States (S0-S5),
// CPU C-States, Thermal Zones, Battery, PowerManager (suspend/resume/shutdown).

#![allow(dead_code)]

// ─── ACPI Table Signatures ────────────────────────────────
const SIG_RSDP: [u8; 8] = *b"RSD PTR ";
const SIG_RSDT: [u8; 4] = *b"RSDT";
const SIG_XSDT: [u8; 4] = *b"XSDT";
const SIG_FADT: [u8; 4] = *b"FACP";
const SIG_DSDT: [u8; 4] = *b"DSDT";
const SIG_MADT: [u8; 4] = *b"APIC";
const SIG_MCFG: [u8; 4] = *b"MCFG";
const SIG_HPET: [u8; 4] = *b"HPET";
const SIG_SSDT: [u8; 4] = *b"SSDT";

// ─── ACPI Table ──────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct AcpiTableHeader {
    pub signature: [u8; 4],
    pub length: u32,
    pub revision: u8,
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub oem_table_id: [u8; 8],
    pub oem_revision: u32,
    pub creator_id: u32,
    pub creator_revision: u32,
}

impl AcpiTableHeader {
    pub fn new(signature: [u8; 4], length: u32, revision: u8) -> Self {
        Self {
            signature, length, revision, checksum: 0,
            oem_id: *b"SHIVACO", oem_table_id: *b"SHIVA000",
            oem_revision: 1, creator_id: 0x4F53, creator_revision: 1,
        }
    }

    pub fn matches(&self, sig: &[u8; 4]) -> bool {
        &self.signature == sig
    }

    pub fn name(&self) -> &str {
        match &self.signature {
            x if x == b"RSDT" => "RSDT",
            x if x == b"XSDT" => "XSDT",
            x if x == b"FACP" => "FADT",
            x if x == b"DSDT" => "DSDT",
            x if x == b"APIC" => "MADT",
            x if x == b"MCFG" => "MCFG",
            x if x == b"HPET" => "HPET",
            x if x == b"SSDT" => "SSDT",
            _ => "UNKNOWN",
        }
    }
}

// ─── RSDP (Root System Description Pointer) ────────────────
#[derive(Clone, Debug)]
pub struct Rsdp {
    pub signature: [u8; 8],
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub revision: u8,
    pub rsdt_address: u32,
    pub length: u32,
    pub xsdt_address: u64,
    pub extended_checksum: u8,
}

impl Rsdp {
    pub fn new(rsdt_addr: u32) -> Self {
        Self {
            signature: SIG_RSDP, checksum: 0, oem_id: *b"SHIVACO",
            revision: 2, rsdt_address: rsdt_addr, length: 36,
            xsdt_address: 0, extended_checksum: 0,
        }
    }

    pub fn new_v1(rsdt_addr: u32) -> Self {
        Self {
            signature: SIG_RSDP, checksum: 0, oem_id: *b"SHIVACO",
            revision: 1, rsdt_address: rsdt_addr, length: 20,
            xsdt_address: 0, extended_checksum: 0,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.signature == SIG_RSDP
    }

    pub fn is_acpi_v2(&self) -> bool { self.revision >= 2 }
}

// ─── FADT (Fixed ACPI Description Table) ───────────────────
#[derive(Clone, Debug)]
pub struct Fadt {
    pub header: AcpiTableHeader,
    pub firmware_ctrl: u32,
    pub dsdt_address: u32,
    pub sci_interrupt: u16,
    pub smi_cmd_port: u32,
    pub acpi_enable: u8,
    pub acpi_disable: u8,
    pub s4bios_req: u8,
    pub pstate_cnt: u8,
    pub pm1a_evt_blk: u32,
    pub pm1b_evt_blk: u32,
    pub pm1a_cnt_blk: u32,
    pub pm1b_cnt_blk: u32,
    pub pm2_cnt_blk: u32,
    pub pm_tmr_blk: u32,
    pub gpe0_blk: u32,
    pub gpe1_blk: u32,
    pub pm1_evt_len: u8,
    pub pm1_cnt_len: u8,
    pub pm2_cnt_len: u8,
    pub pm_tmr_len: u8,
    pub gpe0_blk_len: u8,
    pub gpe1_blk_len: u8,
    pub minor_revision: u8,
    pub flags: u32,
}

impl Fadt {
    pub fn new() -> Self {
        Self {
            header: AcpiTableHeader::new(SIG_FADT, 268, 5),
            firmware_ctrl: 0, dsdt_address: 0x1000,
            sci_interrupt: 9, smi_cmd_port: 0xB2,
            acpi_enable: 0xA0, acpi_disable: 0xA1,
            s4bios_req: 0, pstate_cnt: 0,
            pm1a_evt_blk: 0x400, pm1b_evt_blk: 0,
            pm1a_cnt_blk: 0x404, pm2_cnt_blk: 0,
            pm_tmr_blk: 0x408, gpe0_blk: 0x420, gpe1_blk: 0,
            pm1_evt_len: 4, pm1_cnt_len: 2, pm2_cnt_len: 1,
            pm_tmr_len: 4, gpe0_blk_len: 16, gpe1_blk_len: 0,
            minor_revision: 0, flags: 0,
        }
    }

    pub fn supports_hpet(&self) -> bool { self.flags & 0x80 != 0 }
    pub fn supports_wakeup(&self) -> bool { self.flags & 0x20 != 0 }
}

// ─── Power States (S-States) ──────────────────────────────
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PowerState {
    /// S0: Full working state
    Working,
    /// S1: CPU halted, cache maintained, fast wake
    Sleep1,
    /// S2: CPU off, cache lost
    Sleep2,
    /// S3: Suspend to RAM (most common)
    Suspend,
    /// S4: Suspend to disk (hibernate)
    Hibernate,
    /// S5: Soft off
    SoftOff,
    /// G2/S5 transitional
    Transitioning,
}

impl PowerState {
    pub fn s_number(&self) -> u8 {
        match self {
            Self::Working => 0, Self::Sleep1 => 1, Self::Sleep2 => 2,
            Self::Suspend => 3, Self::Hibernate => 4, Self::SoftOff => 5,
            Self::Transitioning => 99,
        }
    }

    pub fn from_s(n: u8) -> Self {
        match n { 0 => Self::Working, 1 => Self::Sleep1, 2 => Self::Sleep2,
            3 => Self::Suspend, 4 => Self::Hibernate, 5 => Self::SoftOff, _ => Self::Working }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Working => "S0 (Working)", Self::Sleep1 => "S1 (Sleep)",
            Self::Sleep2 => "S2 (Deep Sleep)", Self::Suspend => "S3 (Suspend to RAM)",
            Self::Hibernate => "S4 (Hibernate)", Self::SoftOff => "S5 (Soft Off)",
            Self::Transitioning => "Transitioning",
        }
    }

    pub fn is_working(&self) -> bool { *self == Self::Working }
    pub fn is_sleeping(&self) -> bool {
        matches!(self, Self::Sleep1 | Self::Sleep2 | Self::Suspend | Self::Hibernate)
    }
    pub fn is_off(&self) -> bool { *self == Self::SoftOff }
    pub fn wake_latency_ns(&self) -> u64 {
        match self {
            Self::Working => 0, Self::Sleep1 => 100_000,
            Self::Sleep2 => 1_000_000, Self::Suspend => 10_000_000,
            Self::Hibernate => 500_000_000, Self::SoftOff => 1_000_000_000,
            Self::Transitioning => 0,
        }
    }
}

// ─── CPU C-States (idle) ──────────────────────────────────
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CpuState {
    /// C0: Active
    C0,
    /// C1: Halt (MWAIT)
    C1,
    /// C2: Stop-Clock
    C2,
    /// C3: Deep Sleep (cache flushed)
    C3,
}

impl CpuState {
    pub fn from_u8(n: u8) -> Self {
        match n { 0 => Self::C0, 1 => Self::C1, 2 => Self::C2, 3 => Self::C3, _ => Self::C0 }
    }

    pub fn as_u8(&self) -> u8 {
        match self { Self::C0 => 0, Self::C1 => 1, Self::C2 => 2, Self::C3 => 3 }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::C0 => "C0 (Active)", Self::C1 => "C1 (Halt)",
            Self::C2 => "C2 (Stop-Clock)", Self::C3 => "C3 (Deep Sleep)",
        }
    }

    pub fn is_active(&self) -> bool { *self == Self::C0 }
    pub fn is_idle(&self) -> bool { !self.is_active() }
    pub fn power_savings_pct(&self) -> u8 {
        match self { Self::C0 => 0, Self::C1 => 10, Self::C2 => 40, Self::C3 => 80 }
    }
}

// ─── Thermal Zone ────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct ThermalZone {
    pub zone_id: u32,
    pub current_temp: u32,    // millidegrees Celsius
    pub critical_temp: u32,
    pub hot_temp: u32,
    pub passive_temp: u32,
    pub active_temp: u32,
    pub polling_freq_hz: u32,
}

impl ThermalZone {
    pub fn new(zone_id: u32) -> Self {
        Self {
            zone_id, current_temp: 35_000, critical_temp: 95_000,
            hot_temp: 80_000, passive_temp: 70_000, active_temp: 55_000,
            polling_freq_hz: 10,
        }
    }

    pub fn temp_celsius(&self) -> f32 { self.current_temp as f32 / 1000.0 }
    pub fn is_critical(&self) -> bool { self.current_temp >= self.critical_temp }
    pub fn is_hot(&self) -> bool { self.current_temp >= self.hot_temp }
    pub fn is_warm(&self) -> bool { self.current_temp >= self.passive_temp }
    pub fn is_normal(&self) -> bool { !self.is_warm() }

    pub fn set_temp(&mut self, millidegrees: u32) {
        self.current_temp = millidegrees;
    }

    pub fn cooling_needed(&self) -> bool { self.current_temp > self.active_temp }
}

// ─── Battery ──────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct BatteryInfo {
    pub present: bool,
    pub capacity_pct: u8,      // 0-100
    pub voltage_mv: u16,
    pub current_ma: i16,       // negative = discharging, positive = charging
    pub charging: bool,
    pub discharging: bool,
    pub critical: bool,
    pub low: bool,
    pub cycles: u32,
    pub model: [u8; 32],
    pub serial: [u8; 32],
}

impl BatteryInfo {
    pub fn new() -> Self {
        Self {
            present: true, capacity_pct: 100, voltage_mv: 12000,
            current_ma: 0, charging: false, discharging: false,
            critical: false, low: false, cycles: 0,
            model: [0; 32], serial: [0; 32],
        }
    }

    pub fn is_full(&self) -> bool { self.capacity_pct >= 100 }
    pub fn is_empty(&self) -> bool { self.capacity_pct == 0 }
    pub fn is_low(&self) -> bool { self.capacity_pct <= 15 }
    pub fn is_critical_level(&self) -> bool { self.capacity_pct <= 5 }

    pub fn time_remaining_min(&self) -> Option<u32> {
        if self.discharging && self.current_ma < 0 && self.voltage_mv > 0 {
            let mah = (self.capacity_pct as u32 * 50) / 100; // assume 5000mAh full
            let drain = (-self.current_ma) as u32;
            if drain > 0 { Some((mah * 60) / drain) } else { None }
        } else { None }
    }

    pub fn update(&mut self, capacity: u8, charging: bool, discharging: bool) {
        self.capacity_pct = capacity;
        self.charging = charging;
        self.discharging = discharging;
        self.critical = capacity <= 5;
        self.low = capacity <= 15;
    }
}

// ─── Power Event ───────────────────────────────────────────
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PowerEvent {
    PowerButton,
    SleepButton,
    LidClose,
    LidOpen,
    BatteryLow,
    BatteryCritical,
    ThermalCritical,
    ThermalHot,
    Wakeup,
    AcpiEnable,
    AcpiDisable,
}

impl PowerEvent {
    pub fn name(&self) -> &'static str {
        match self {
            Self::PowerButton => "power_button", Self::SleepButton => "sleep_button",
            Self::LidClose => "lid_close", Self::LidOpen => "lid_open",
            Self::BatteryLow => "battery_low", Self::BatteryCritical => "battery_critical",
            Self::ThermalCritical => "thermal_critical", Self::ThermalHot => "thermal_hot",
            Self::Wakeup => "wakeup", Self::AcpiEnable => "acpi_enable",
            Self::AcpiDisable => "acpi_disable",
        }
    }

    pub fn triggers_suspend(&self) -> bool {
        matches!(self, Self::SleepButton | Self::LidClose)
    }

    pub fn triggers_shutdown(&self) -> bool {
        matches!(self, Self::PowerButton | Self::BatteryCritical | Self::ThermalCritical)
    }
}

// ─── Power Manager ────────────────────────────────────────
pub struct PowerManager {
    pub rsdp: Option<Rsdp>,
    pub fadt: Option<Fadt>,
    pub state: PowerState,
    pub cpu_state: CpuState,
    pub thermal: Vec<ThermalZone>,
    pub batteries: Vec<BatteryInfo>,
    pub ac_online: bool,
    pub events: Vec<PowerEvent>,
    pub acpi_enabled: bool,
    pub total_suspend_count: u32,
    pub last_sleep_ns: u64,
    pub last_wake_ns: u64,
    pub total_sleep_time_ns: u64,
}

impl PowerManager {
    pub fn new() -> Self {
        Self {
            rsdp: None, fadt: None,
            state: PowerState::Working, cpu_state: CpuState::C0,
            thermal: Vec::new(), batteries: Vec::new(),
            ac_online: true, events: Vec::new(),
            acpi_enabled: false,
            total_suspend_count: 0, last_sleep_ns: 0, last_wake_ns: 0,
            total_sleep_time_ns: 0,
        }
    }

    pub fn init_acpi(&mut self, rsdp: Rsdp, fadt: Fadt) {
        self.rsdp = Some(rsdp);
        self.fadt = Some(fadt);
        self.acpi_enabled = true;
        self.events.push(PowerEvent::AcpiEnable);
    }

    pub fn disable_acpi(&mut self) {
        self.acpi_enabled = false;
        self.events.push(PowerEvent::AcpiDisable);
    }

    pub fn is_acpi_enabled(&self) -> bool { self.acpi_enabled }

    pub fn add_thermal_zone(&mut self, zone: ThermalZone) {
        self.thermal.push(zone);
    }

    pub fn add_battery(&mut self, battery: BatteryInfo) {
        self.batteries.push(battery);
    }

    pub fn suspend(&mut self, timestamp_ns: u64) -> Result<(), PowerError> {
        if !self.acpi_enabled { return Err(PowerError::AcpiNotEnabled); }
        if self.state.is_sleeping() { return Err(PowerError::AlreadySleeping); }
        self.state = PowerState::Transitioning;
        self.last_sleep_ns = timestamp_ns;
        self.state = PowerState::Suspend;
        self.total_suspend_count += 1;
        Ok(())
    }

    pub fn hibernate(&mut self, timestamp_ns: u64) -> Result<(), PowerError> {
        if !self.acpi_enabled { return Err(PowerError::AcpiNotEnabled); }
        if self.state.is_sleeping() { return Err(PowerError::AlreadySleeping); }
        self.state = PowerState::Transitioning;
        self.last_sleep_ns = timestamp_ns;
        self.state = PowerState::Hibernate;
        self.total_suspend_count += 1;
        Ok(())
    }

    pub fn resume(&mut self, timestamp_ns: u64) -> Result<(), PowerError> {
        if !self.state.is_sleeping() { return Err(PowerError::NotSleeping); }
        self.state = PowerState::Transitioning;
        self.last_wake_ns = timestamp_ns;
        self.total_sleep_time_ns += timestamp_ns - self.last_sleep_ns;
        self.state = PowerState::Working;
        self.cpu_state = CpuState::C0;
        self.events.push(PowerEvent::Wakeup);
        Ok(())
    }

    pub fn shutdown(&mut self) -> Result<(), PowerError> {
        if !self.acpi_enabled { return Err(PowerError::AcpiNotEnabled); }
        self.state = PowerState::SoftOff;
        Ok(())
    }

    pub fn reboot(&mut self) -> Result<(), PowerError> {
        if !self.acpi_enabled { return Err(PowerError::AcpiNotEnabled); }
        self.state = PowerState::Transitioning;
        self.state = PowerState::Working;
        Ok(())
    }

    pub fn set_cpu_idle(&mut self, state: CpuState) {
        self.cpu_state = state;
    }

    pub fn handle_event(&mut self, event: PowerEvent, timestamp_ns: u64) -> Result<(), PowerError> {
        self.events.push(event.clone());
        if event.triggers_suspend() {
            self.suspend(timestamp_ns)?;
        } else if event.triggers_shutdown() {
            self.shutdown()?;
        }
        Ok(())
    }

    pub fn check_thermal(&mut self) -> Option<PowerEvent> {
        for zone in &self.thermal {
            if zone.is_critical() { return Some(PowerEvent::ThermalCritical); }
            if zone.is_hot() { return Some(PowerEvent::ThermalHot); }
        }
        None
    }

    pub fn check_battery(&mut self) -> Option<PowerEvent> {
        for bat in &self.batteries {
            if bat.is_critical_level() { return Some(PowerEvent::BatteryCritical); }
            if bat.is_low() { return Some(PowerEvent::BatteryLow); }
        }
        None
    }

    pub fn avg_temp_celsius(&self) -> f32 {
        if self.thermal.is_empty() { return 0.0; }
        let sum: f32 = self.thermal.iter().map(|z| z.temp_celsius()).sum();
        sum / self.thermal.len() as f32
    }

    pub fn avg_battery_pct(&self) -> u8 {
        if self.batteries.is_empty() { return 100; }
        let sum: u32 = self.batteries.iter().filter(|b| b.present).map(|b| b.capacity_pct as u32).sum();
        let count = self.batteries.iter().filter(|b| b.present).count().max(1);
        (sum / count as u32) as u8
    }

    pub fn power_source(&self) -> &'static str {
        if self.ac_online { "AC" } else { "Battery" }
    }

    pub fn status_string(&self) -> String {
        format!(
            "Power: {} | CPU: {} | Temp: {:.1}°C | Battery: {}% | Source: {}",
            self.state.name(), self.cpu_state.name(),
            self.avg_temp_celsius(), self.avg_battery_pct(), self.power_source()
        )
    }

    pub fn thermal_zone_count(&self) -> usize { self.thermal.len() }
    pub fn battery_count(&self) -> usize { self.batteries.len() }
    pub fn event_count(&self) -> usize { self.events.len() }
}

impl Default for PowerManager {
    fn default() -> Self { Self::new() }
}

// ─── Power Error ──────────────────────────────────────────
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PowerError {
    AcpiNotEnabled,
    AlreadySleeping,
    NotSleeping,
    InvalidTransition,
    NoThermalZone,
    NoBattery,
}

// ─── Tests ─────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    // ── AcpiTableHeader ──
    #[test]
    fn test_acpi_header_new() {
        let h = AcpiTableHeader::new(SIG_FADT, 268, 5);
        assert_eq!(h.signature, SIG_FADT);
        assert_eq!(h.length, 268);
        assert_eq!(h.revision, 5);
    }

    #[test]
    fn test_acpi_header_matches() {
        let h = AcpiTableHeader::new(SIG_MADT, 100, 3);
        assert!(h.matches(&SIG_MADT));
        assert!(!h.matches(&SIG_FADT));
    }

    #[test]
    fn test_acpi_header_name() {
        assert_eq!(AcpiTableHeader::new(SIG_FADT, 0, 0).name(), "FADT");
        assert_eq!(AcpiTableHeader::new(SIG_DSDT, 0, 0).name(), "DSDT");
        assert_eq!(AcpiTableHeader::new(SIG_MADT, 0, 0).name(), "MADT");
        assert_eq!(AcpiTableHeader::new(SIG_HPET, 0, 0).name(), "HPET");
    }

    // ── Rsdp ──
    #[test]
    fn test_rsdp_new() {
        let r = Rsdp::new(0x1FFE_0000);
        assert!(r.is_valid());
        assert!(r.is_acpi_v2());
        assert_eq!(r.rsdt_address, 0x1FFE_0000);
    }

    #[test]
    fn test_rsdp_v1() {
        let r = Rsdp::new_v1(0x1FFE_0000);
        assert!(r.is_valid());
        assert!(!r.is_acpi_v2());
    }

    #[test]
    fn test_rsdp_invalid_sig() {
        let mut r = Rsdp::new(0);
        r.signature = *b"BADPTR  ";
        assert!(!r.is_valid());
    }

    // ── Fadt ──
    #[test]
    fn test_fadt_new() {
        let f = Fadt::new();
        assert_eq!(f.dsdt_address, 0x1000);
        assert_eq!(f.sci_interrupt, 9);
        assert_eq!(f.smi_cmd_port, 0xB2);
        assert_eq!(f.acpi_enable, 0xA0);
    }

    // ── PowerState ──
    #[test]
    fn test_power_state_s_numbers() {
        assert_eq!(PowerState::Working.s_number(), 0);
        assert_eq!(PowerState::Suspend.s_number(), 3);
        assert_eq!(PowerState::Hibernate.s_number(), 4);
        assert_eq!(PowerState::SoftOff.s_number(), 5);
    }

    #[test]
    fn test_power_state_from_s() {
        assert_eq!(PowerState::from_s(0), PowerState::Working);
        assert_eq!(PowerState::from_s(3), PowerState::Suspend);
        assert_eq!(PowerState::from_s(5), PowerState::SoftOff);
    }

    #[test]
    fn test_power_state_is_working() {
        assert!(PowerState::Working.is_working());
        assert!(!PowerState::Suspend.is_working());
    }

    #[test]
    fn test_power_state_is_sleeping() {
        assert!(PowerState::Suspend.is_sleeping());
        assert!(PowerState::Hibernate.is_sleeping());
        assert!(PowerState::Sleep1.is_sleeping());
        assert!(!PowerState::Working.is_sleeping());
    }

    #[test]
    fn test_power_state_is_off() {
        assert!(PowerState::SoftOff.is_off());
        assert!(!PowerState::Working.is_off());
    }

    #[test]
    fn test_power_state_names() {
        assert_eq!(PowerState::Working.name(), "S0 (Working)");
        assert_eq!(PowerState::Suspend.name(), "S3 (Suspend to RAM)");
        assert_eq!(PowerState::Hibernate.name(), "S4 (Hibernate)");
    }

    #[test]
    fn test_power_state_wake_latency() {
        assert_eq!(PowerState::Working.wake_latency_ns(), 0);
        assert!(PowerState::Suspend.wake_latency_ns() > 0);
        assert!(PowerState::Hibernate.wake_latency_ns() > PowerState::Suspend.wake_latency_ns());
    }

    // ── CpuState ──
    #[test]
    fn test_cpu_state_from_u8() {
        assert_eq!(CpuState::from_u8(0), CpuState::C0);
        assert_eq!(CpuState::from_u8(1), CpuState::C1);
        assert_eq!(CpuState::from_u8(3), CpuState::C3);
    }

    #[test]
    fn test_cpu_state_as_u8() {
        assert_eq!(CpuState::C0.as_u8(), 0);
        assert_eq!(CpuState::C3.as_u8(), 3);
    }

    #[test]
    fn test_cpu_state_active_idle() {
        assert!(CpuState::C0.is_active());
        assert!(!CpuState::C0.is_idle());
        assert!(CpuState::C1.is_idle());
        assert!(CpuState::C3.is_idle());
    }

    #[test]
    fn test_cpu_state_power_savings() {
        assert_eq!(CpuState::C0.power_savings_pct(), 0);
        assert!(CpuState::C1.power_savings_pct() > 0);
        assert!(CpuState::C3.power_savings_pct() > CpuState::C1.power_savings_pct());
    }

    #[test]
    fn test_cpu_state_names() {
        assert_eq!(CpuState::C0.name(), "C0 (Active)");
        assert_eq!(CpuState::C3.name(), "C3 (Deep Sleep)");
    }

    // ── ThermalZone ──
    #[test]
    fn test_thermal_new() {
        let z = ThermalZone::new(0);
        assert_eq!(z.zone_id, 0);
        assert_eq!(z.current_temp, 35_000);
        assert!(!z.is_critical());
        assert!(!z.is_hot());
        assert!(z.is_normal());
    }

    #[test]
    fn test_thermal_temp_celsius() {
        let z = ThermalZone::new(0);
        assert!((z.temp_celsius() - 35.0).abs() < 0.01);
    }

    #[test]
    fn test_thermal_critical() {
        let mut z = ThermalZone::new(0);
        z.set_temp(95_000);
        assert!(z.is_critical());
    }

    #[test]
    fn test_thermal_hot() {
        let mut z = ThermalZone::new(0);
        z.set_temp(82_000);
        assert!(z.is_hot());
    }

    #[test]
    fn test_thermal_warm() {
        let mut z = ThermalZone::new(0);
        z.set_temp(72_000);
        assert!(z.is_warm());
        assert!(!z.is_normal());
    }

    #[test]
    fn test_thermal_cooling_needed() {
        let mut z = ThermalZone::new(0);
        z.set_temp(60_000);
        assert!(z.cooling_needed());
        z.set_temp(40_000);
        assert!(!z.cooling_needed());
    }

    // ── BatteryInfo ──
    #[test]
    fn test_battery_new() {
        let b = BatteryInfo::new();
        assert!(b.present);
        assert_eq!(b.capacity_pct, 100);
        assert!(b.is_full());
        assert!(!b.is_empty());
    }

    #[test]
    fn test_battery_levels() {
        let mut b = BatteryInfo::new();
        b.update(50, false, true);
        assert!(!b.is_full());
        assert!(!b.is_low());
        b.update(10, false, true);
        assert!(b.is_low());
        b.update(3, false, true);
        assert!(b.is_critical_level());
        b.update(0, false, true);
        assert!(b.is_empty());
    }

    #[test]
    fn test_battery_charging() {
        let mut b = BatteryInfo::new();
        b.update(50, true, false);
        assert!(b.charging);
        assert!(!b.discharging);
    }

    #[test]
    fn test_battery_discharging() {
        let mut b = BatteryInfo::new();
        b.update(80, false, true);
        assert!(b.discharging);
        assert!(!b.charging);
    }

    #[test]
    fn test_battery_time_remaining() {
        let mut b = BatteryInfo::new();
        b.update(50, false, true);
        b.current_ma = -1000;
        b.voltage_mv = 12000;
        let t = b.time_remaining_min();
        assert!(t.is_some());
        assert!(t.unwrap() > 0);
    }

    #[test]
    fn test_battery_time_remaining_no_discharge() {
        let b = BatteryInfo::new();
        assert!(b.time_remaining_min().is_none());
    }

    // ── PowerEvent ──
    #[test]
    fn test_power_event_triggers_suspend() {
        assert!(PowerEvent::SleepButton.triggers_suspend());
        assert!(PowerEvent::LidClose.triggers_suspend());
        assert!(!PowerEvent::PowerButton.triggers_suspend());
    }

    #[test]
    fn test_power_event_triggers_shutdown() {
        assert!(PowerEvent::PowerButton.triggers_shutdown());
        assert!(PowerEvent::BatteryCritical.triggers_shutdown());
        assert!(PowerEvent::ThermalCritical.triggers_shutdown());
        assert!(!PowerEvent::SleepButton.triggers_shutdown());
    }

    #[test]
    fn test_power_event_names() {
        assert_eq!(PowerEvent::PowerButton.name(), "power_button");
        assert_eq!(PowerEvent::BatteryLow.name(), "battery_low");
        assert_eq!(PowerEvent::Wakeup.name(), "wakeup");
    }

    // ── PowerManager ──
    #[test]
    fn test_pm_new() {
        let pm = PowerManager::new();
        assert_eq!(pm.state, PowerState::Working);
        assert_eq!(pm.cpu_state, CpuState::C0);
        assert!(!pm.is_acpi_enabled());
        assert!(pm.ac_online);
    }

    #[test]
    fn test_pm_init_acpi() {
        let mut pm = PowerManager::new();
        pm.init_acpi(Rsdp::new(0), Fadt::new());
        assert!(pm.is_acpi_enabled());
        assert!(pm.rsdp.is_some());
        assert!(pm.fadt.is_some());
    }

    #[test]
    fn test_pm_disable_acpi() {
        let mut pm = PowerManager::new();
        pm.init_acpi(Rsdp::new(0), Fadt::new());
        pm.disable_acpi();
        assert!(!pm.is_acpi_enabled());
    }

    #[test]
    fn test_pm_suspend() {
        let mut pm = PowerManager::new();
        pm.init_acpi(Rsdp::new(0), Fadt::new());
        assert!(pm.suspend(1000).is_ok());
        assert_eq!(pm.state, PowerState::Suspend);
        assert_eq!(pm.total_suspend_count, 1);
    }

    #[test]
    fn test_pm_suspend_no_acpi() {
        let mut pm = PowerManager::new();
        assert_eq!(pm.suspend(1000), Err(PowerError::AcpiNotEnabled));
    }

    #[test]
    fn test_pm_suspend_already_sleeping() {
        let mut pm = PowerManager::new();
        pm.init_acpi(Rsdp::new(0), Fadt::new());
        pm.suspend(1000).unwrap();
        assert_eq!(pm.suspend(2000), Err(PowerError::AlreadySleeping));
    }

    #[test]
    fn test_pm_hibernate() {
        let mut pm = PowerManager::new();
        pm.init_acpi(Rsdp::new(0), Fadt::new());
        assert!(pm.hibernate(1000).is_ok());
        assert_eq!(pm.state, PowerState::Hibernate);
    }

    #[test]
    fn test_pm_resume() {
        let mut pm = PowerManager::new();
        pm.init_acpi(Rsdp::new(0), Fadt::new());
        pm.suspend(1000).unwrap();
        assert!(pm.resume(2000).is_ok());
        assert_eq!(pm.state, PowerState::Working);
        assert_eq!(pm.cpu_state, CpuState::C0);
        assert_eq!(pm.total_sleep_time_ns, 1000);
    }

    #[test]
    fn test_pm_resume_not_sleeping() {
        let mut pm = PowerManager::new();
        pm.init_acpi(Rsdp::new(0), Fadt::new());
        assert_eq!(pm.resume(1000), Err(PowerError::NotSleeping));
    }

    #[test]
    fn test_pm_shutdown() {
        let mut pm = PowerManager::new();
        pm.init_acpi(Rsdp::new(0), Fadt::new());
        assert!(pm.shutdown().is_ok());
        assert!(pm.state.is_off());
    }

    #[test]
    fn test_pm_shutdown_no_acpi() {
        let mut pm = PowerManager::new();
        assert_eq!(pm.shutdown(), Err(PowerError::AcpiNotEnabled));
    }

    #[test]
    fn test_pm_reboot() {
        let mut pm = PowerManager::new();
        pm.init_acpi(Rsdp::new(0), Fadt::new());
        pm.shutdown().unwrap();
        assert!(pm.reboot().is_ok());
        assert_eq!(pm.state, PowerState::Working);
    }

    #[test]
    fn test_pm_set_cpu_idle() {
        let mut pm = PowerManager::new();
        pm.set_cpu_idle(CpuState::C1);
        assert_eq!(pm.cpu_state, CpuState::C1);
        pm.set_cpu_idle(CpuState::C3);
        assert_eq!(pm.cpu_state, CpuState::C3);
    }

    #[test]
    fn test_pm_handle_suspend_event() {
        let mut pm = PowerManager::new();
        pm.init_acpi(Rsdp::new(0), Fadt::new());
        pm.handle_event(PowerEvent::SleepButton, 1000).unwrap();
        assert_eq!(pm.state, PowerState::Suspend);
    }

    #[test]
    fn test_pm_handle_shutdown_event() {
        let mut pm = PowerManager::new();
        pm.init_acpi(Rsdp::new(0), Fadt::new());
        pm.handle_event(PowerEvent::PowerButton, 1000).unwrap();
        assert!(pm.state.is_off());
    }

    #[test]
    fn test_pm_handle_thermal_critical() {
        let mut pm = PowerManager::new();
        pm.init_acpi(Rsdp::new(0), Fadt::new());
        pm.handle_event(PowerEvent::ThermalCritical, 1000).unwrap();
        assert!(pm.state.is_off());
    }

    #[test]
    fn test_pm_add_thermal() {
        let mut pm = PowerManager::new();
        pm.add_thermal_zone(ThermalZone::new(0));
        pm.add_thermal_zone(ThermalZone::new(1));
        assert_eq!(pm.thermal_zone_count(), 2);
    }

    #[test]
    fn test_pm_add_battery() {
        let mut pm = PowerManager::new();
        pm.add_battery(BatteryInfo::new());
        assert_eq!(pm.battery_count(), 1);
    }

    #[test]
    fn test_pm_check_thermal_critical() {
        let mut pm = PowerManager::new();
        let mut z = ThermalZone::new(0);
        z.set_temp(95_000);
        pm.add_thermal_zone(z);
        let event = pm.check_thermal();
        assert_eq!(event, Some(PowerEvent::ThermalCritical));
    }

    #[test]
    fn test_pm_check_thermal_hot() {
        let mut pm = PowerManager::new();
        let mut z = ThermalZone::new(0);
        z.set_temp(82_000);
        pm.add_thermal_zone(z);
        let event = pm.check_thermal();
        assert_eq!(event, Some(PowerEvent::ThermalHot));
    }

    #[test]
    fn test_pm_check_thermal_ok() {
        let mut pm = PowerManager::new();
        pm.add_thermal_zone(ThermalZone::new(0));
        assert!(pm.check_thermal().is_none());
    }

    #[test]
    fn test_pm_check_battery_low() {
        let mut pm = PowerManager::new();
        let mut b = BatteryInfo::new();
        b.update(10, false, true);
        pm.add_battery(b);
        let event = pm.check_battery();
        assert_eq!(event, Some(PowerEvent::BatteryLow));
    }

    #[test]
    fn test_pm_check_battery_critical() {
        let mut pm = PowerManager::new();
        let mut b = BatteryInfo::new();
        b.update(3, false, true);
        pm.add_battery(b);
        let event = pm.check_battery();
        assert_eq!(event, Some(PowerEvent::BatteryCritical));
    }

    #[test]
    fn test_pm_check_battery_ok() {
        let mut pm = PowerManager::new();
        pm.add_battery(BatteryInfo::new());
        assert!(pm.check_battery().is_none());
    }

    #[test]
    fn test_pm_avg_temp() {
        let mut pm = PowerManager::new();
        let mut z1 = ThermalZone::new(0);
        z1.set_temp(40_000);
        let mut z2 = ThermalZone::new(1);
        z2.set_temp(50_000);
        pm.add_thermal_zone(z1);
        pm.add_thermal_zone(z2);
        assert!((pm.avg_temp_celsius() - 45.0).abs() < 0.1);
    }

    #[test]
    fn test_pm_avg_temp_no_zones() {
        let pm = PowerManager::new();
        assert_eq!(pm.avg_temp_celsius(), 0.0);
    }

    #[test]
    fn test_pm_avg_battery() {
        let mut pm = PowerManager::new();
        let mut b1 = BatteryInfo::new();
        b1.capacity_pct = 80;
        let mut b2 = BatteryInfo::new();
        b2.capacity_pct = 60;
        pm.add_battery(b1);
        pm.add_battery(b2);
        assert_eq!(pm.avg_battery_pct(), 70);
    }

    #[test]
    fn test_pm_avg_battery_no_batteries() {
        let pm = PowerManager::new();
        assert_eq!(pm.avg_battery_pct(), 100);
    }

    #[test]
    fn test_pm_power_source() {
        let mut pm = PowerManager::new();
        assert_eq!(pm.power_source(), "AC");
        pm.ac_online = false;
        assert_eq!(pm.power_source(), "Battery");
    }

    #[test]
    fn test_pm_status_string() {
        let mut pm = PowerManager::new();
        pm.init_acpi(Rsdp::new(0), Fadt::new());
        let s = pm.status_string();
        assert!(s.contains("S0"));
        assert!(s.contains("C0"));
        assert!(s.contains("AC"));
    }

    #[test]
    fn test_pm_event_count() {
        let mut pm = PowerManager::new();
        pm.init_acpi(Rsdp::new(0), Fadt::new());
        pm.handle_event(PowerEvent::SleepButton, 1000).unwrap();
        pm.resume(2000).unwrap();
        assert!(pm.event_count() >= 3); // AcpiEnable + SleepButton + Wakeup
    }

    // ── Integration ──
    #[test]
    fn test_full_power_lifecycle() {
        let mut pm = PowerManager::new();
        pm.init_acpi(Rsdp::new(0), Fadt::new());
        let mut z = ThermalZone::new(0);
        z.set_temp(40_000);
        pm.add_thermal_zone(z);
        pm.add_battery(BatteryInfo::new());

        // Normal operation
        assert!(pm.state.is_working());

        // Suspend
        pm.suspend(1000).unwrap();
        assert!(pm.state.is_sleeping());

        // Resume
        pm.resume(2000).unwrap();
        assert!(pm.state.is_working());
        assert_eq!(pm.total_sleep_time_ns, 1000);

        // Suspend via event
        pm.handle_event(PowerEvent::LidClose, 3000).unwrap();
        assert!(pm.state.is_sleeping());

        pm.resume(4000).unwrap();

        // Shutdown
        pm.handle_event(PowerEvent::PowerButton, 5000).unwrap();
        assert!(pm.state.is_off());

        // Reboot
        pm.reboot().unwrap();
        assert!(pm.state.is_working());
    }

    #[test]
    fn test_multiple_thermal_zones() {
        let mut pm = PowerManager::new();
        let mut z0 = ThermalZone::new(0);
        z0.set_temp(45_000);
        let mut z1 = ThermalZone::new(1);
        z1.set_temp(65_000);
        let mut z2 = ThermalZone::new(2);
        z2.set_temp(96_000);
        pm.add_thermal_zone(z0);
        pm.add_thermal_zone(z1);
        pm.add_thermal_zone(z2);

        let event = pm.check_thermal();
        assert_eq!(event, Some(PowerEvent::ThermalCritical));
        assert_eq!(pm.thermal_zone_count(), 3);
    }

    #[test]
    fn test_multiple_batteries() {
        let mut pm = PowerManager::new();
        let mut b1 = BatteryInfo::new();
        b1.update(90, true, false);
        let mut b2 = BatteryInfo::new();
        b2.update(40, false, true);
        pm.add_battery(b1);
        pm.add_battery(b2);

        assert_eq!(pm.battery_count(), 2);
        assert_eq!(pm.avg_battery_pct(), 65);
    }

    #[test]
    fn test_thermal_then_shutdown() {
        let mut pm = PowerManager::new();
        pm.init_acpi(Rsdp::new(0), Fadt::new());
        let mut z = ThermalZone::new(0);
        z.set_temp(97_000);
        pm.add_thermal_zone(z);

        let event = pm.check_thermal().unwrap();
        pm.handle_event(event, 1000).unwrap();
        assert!(pm.state.is_off());
    }
}
