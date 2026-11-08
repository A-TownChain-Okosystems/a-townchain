use atc_genesis_runtime::lifecycle::{FixedTimestep, RuntimeConfig};

#[test]
fn default_runtime_tick_is_60hz() {
    let mut clock = FixedTimestep::new(RuntimeConfig::default()).unwrap();
    let plan = clock.advance(1.0 / 60.0);
    assert_eq!(plan.simulation_steps, 1);
    assert!(plan.interpolation_alpha.abs() < 1e-12);
}

#[test]
fn frame_spike_is_bounded() {
    let mut clock = FixedTimestep::new(RuntimeConfig::default()).unwrap();
    let plan = clock.advance(1.0);
    assert_eq!(plan.simulation_steps, 8);
    assert!(plan.dropped_time > 0.0);
}
