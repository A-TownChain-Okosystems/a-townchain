//! ShivaCore Kernel — DA-HEFT Scheduler (Rust).
//!
//! Portiert daheft.py + accelerator.py (Python, 08.07.2026) nach Rust.
//! Deadline-Aware Heterogeneous Earliest-Finish-Time:
//! verteilt Tasks auf heterogene Beschleuniger (GPU/NPU/CPU) und
//! respektiert Deadlines. Hardware wird ueber ein Trait-Interface
//! abstrahiert — echte Hardware spaeter ohne Algorithmus-Aenderung.

use alloc::vec;
use alloc::boxed::Box;
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use crate::capability::Pid;

/// Beschleuniger-Typ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AcceleratorType { Cpu, Gpu, Npu, Tpu }

/// Hardware-Abstraktion: jeder Beschleuniger implementiert dieses Trait.
pub trait Accelerator {
    fn id(&self) -> u64;
    fn acc_type(&self) -> AcceleratorType;
    fn flops(&self) -> f64;
    fn current_load(&self) -> f64;
    fn temperature(&self) -> f64;
    fn is_thermal_ok(&self) -> bool;
    fn available_memory_mb(&self) -> u64;
}

/// Simulierter Beschleuniger fuer Tests und Software-Validierung
#[derive(Debug, Clone)]
pub struct SimulatedAccelerator {
    pub id: u64,
    pub acc_type: AcceleratorType,
    pub flops: f64,
    pub load: f64,
    pub temp: f64,
    pub mem_mb: u64,
}

impl Accelerator for SimulatedAccelerator {
    fn id(&self) -> u64 { self.id }
    fn acc_type(&self) -> AcceleratorType { self.acc_type }
    fn flops(&self) -> f64 { self.flops }
    fn current_load(&self) -> f64 { self.load }
    fn temperature(&self) -> f64 { self.temp }
    fn is_thermal_ok(&self) -> bool { self.temp < 85.0 }
    fn available_memory_mb(&self) -> u64 { self.mem_mb }
}

/// Task-Typ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TaskKind { Inference, Training, MatMul, Convolution, Transfer }

/// Ein Compute-Task
#[derive(Debug, Clone)]
pub struct Task {
    pub id: u64,
    pub kind: TaskKind,
    pub compute_flops: f64,
    pub memory_mb: u64,
    pub deadline: f64,
    pub dependencies: Vec<u64>,
    pub pid: Option<Pid>,
    pub priority: u8,
}

/// Scheduler-Ergebnis fuer einen Task
#[derive(Debug, Clone)]
pub struct ScheduleEntry {
    pub task_id: u64,
    pub accel_id: u64,
    pub start_time: f64,
    pub finish_time: f64,
    pub deadline_met: bool,
}

/// Der DA-HEFT Scheduler
pub struct DaHeftScheduler {
    accelerators: Vec<Box<dyn Accelerator + Send>>,
    accel_free_at: BTreeMap<u64, f64>,
}

impl DaHeftScheduler {
    pub fn new() -> Self {
        Self { accelerators: Vec::new(), accel_free_at: BTreeMap::new() }
    }

    pub fn add_accelerator(&mut self, accel: Box<dyn Accelerator + Send>) {
        let id = accel.id();
        self.accel_free_at.insert(id, 0.0);
        self.accelerators.push(accel);
    }

    /// Upward-Rank nach HEFT: rank_u(n) = w_n + max over SUCCESSORS { rank_u(s) + c_{n,s} }
    /// Entry-Tasks (keine Predecessors) haben den hoechsten Rank -> werden zuerst gescheduled.
    fn compute_upward_ranks(&self, tasks: &[Task]) -> BTreeMap<u64, f64> {
        let min_flops = self.accelerators.iter()
            .map(|a| a.flops())
            .fold(f64::INFINITY, f64::min);

        // Successor-Map: task_id -> Vec<(successor_id, transfer_cost)>
        let mut successors: BTreeMap<u64, Vec<u64>> = BTreeMap::new();
        for task in tasks {
            for &dep_id in &task.dependencies {
                successors.entry(dep_id).or_default().push(task.id);
            }
        }

        // Iterative Berechnung ( Bottom-up): wiederhole bis stabil
        let mut ranks: BTreeMap<u64, f64> = BTreeMap::new();
        let task_map: BTreeMap<u64, &Task> = tasks.iter().map(|t| (t.id, t)).collect();

        // Init: alle Tasks mit ihrem eigenen compute cost
        for task in tasks {
            ranks.insert(task.id, task.compute_flops / min_flops);
        }

        // Iteriere bis stabil (max tasks.len() Iterationen fuer Konvergenz)
        for _ in 0..tasks.len() + 1 {
            let mut changed = false;
            for task in tasks {
                let succs = successors.get(&task.id);
                let succ_max: f64 = match succs {
                    Some(s_list) => s_list.iter()
                        .filter_map(|s| ranks.get(s))
                        .copied()
                        .fold(0.0_f64, f64::max),
                    None => 0.0,
                };
                let transfer = succs.map(|s| s.len()).unwrap_or(0) as f64 * 0.01;
                let new_rank = task.compute_flops / min_flops + succ_max + transfer;
                let old_rank = ranks.get(&task.id).copied().unwrap_or(0.0);
                if (new_rank - old_rank).abs() > 1e-12 {
                    ranks.insert(task.id, new_rank);
                    changed = true;
                }
            }
            if !changed { break; }
        }

        ranks
    }

    /// Haupt-Scheduler: sortiert Tasks nach upward-rank (absteigend),
    /// weist jedem Task den Beschleuniger mit fruehestem Finish-Time zu.
    pub fn schedule(&mut self, tasks: &[Task]) -> Vec<ScheduleEntry> {
        if self.accelerators.is_empty() || tasks.is_empty() {
            return Vec::new();
        }

        let ranks = self.compute_upward_ranks(tasks);

        // 1. Tasks nach upward-rank sortieren (absteigend)
        let mut sorted: Vec<&Task> = tasks.iter().collect();
        sorted.sort_by(|a, b| {
            let rank_a = ranks.get(&a.id).copied().unwrap_or(0.0);
            let rank_b = ranks.get(&b.id).copied().unwrap_or(0.0);
            rank_b.partial_cmp(&rank_a).unwrap_or(core::cmp::Ordering::Equal)
        });

        let mut schedule_result = Vec::new();
        let mut task_finish: BTreeMap<u64, f64> = BTreeMap::new();

        for task in sorted {
            // Fruehestmoeglicher Start: max aller Abhaengigkeiten
            let mut earliest_start = 0.0_f64;
            for dep_id in &task.dependencies {
                if let Some(&finish) = task_finish.get(dep_id) {
                    earliest_start = earliest_start.max(finish);
                }
            }

            let mut best_entry: Option<ScheduleEntry> = None;
            let mut best_finish = f64::INFINITY;

            for accel in &self.accelerators {
                if !accel.is_thermal_ok() { continue; }
                if accel.available_memory_mb() < task.memory_mb { continue; }

                let free_at = self.accel_free_at.get(&accel.id()).copied().unwrap_or(0.0);
                let start = earliest_start.max(free_at);
                let exec_time = task.compute_flops / accel.flops();
                let finish = start + exec_time;

                if finish < best_finish {
                    best_finish = finish;
                    best_entry = Some(ScheduleEntry {
                        task_id: task.id,
                        accel_id: accel.id(),
                        start_time: start,
                        finish_time: finish,
                        deadline_met: finish <= task.deadline,
                    });
                }
            }

            if let Some(entry) = best_entry {
                self.accel_free_at.insert(entry.accel_id, entry.finish_time);
                task_finish.insert(task.id, entry.finish_time);
                schedule_result.push(entry);
            }
        }

        schedule_result.sort_by_key(|e| e.task_id);
        schedule_result
    }

    pub fn deadline_misses(&self, schedule: &[ScheduleEntry]) -> usize {
        schedule.iter().filter(|e| !e.deadline_met).count()
    }

    pub fn utilization(&self, schedule: &[ScheduleEntry]) -> BTreeMap<u64, f64> {
        let mut util: BTreeMap<u64, f64> = BTreeMap::new();
        if schedule.is_empty() { return util; }
        let max_finish = schedule.iter().map(|e| e.finish_time).fold(0.0_f64, f64::max);
        for accel in &self.accelerators {
            let busy: f64 = schedule.iter()
                .filter(|e| e.accel_id == accel.id())
                .map(|e| e.finish_time - e.start_time)
                .sum();
            util.insert(accel.id(), if max_finish > 0.0 { busy / max_finish } else { 0.0 });
        }
        util
    }

    pub fn accelerator_count(&self) -> usize { self.accelerators.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_task(id: u64, flops: f64, deadline: f64, deps: Vec<u64>) -> Task {
        Task { id, kind: TaskKind::Inference, compute_flops: flops,
            memory_mb: 100, deadline, dependencies: deps, pid: None, priority: 128 }
    }
    fn make_accel(id: u64, at: AcceleratorType, flops: f64) -> SimulatedAccelerator {
        SimulatedAccelerator { id, acc_type: at, flops, load: 0.0, temp: 50.0, mem_mb: 8192 }
    }

    #[test]
    fn test_basic_scheduling() {
        let mut sched = DaHeftScheduler::new();
        sched.add_accelerator(Box::new(make_accel(1, AcceleratorType::Gpu, 10e12)));
        sched.add_accelerator(Box::new(make_accel(2, AcceleratorType::Cpu, 2e12)));
        let tasks = vec![
            make_task(1, 5e12, 10.0, vec![]),
            make_task(2, 3e12, 10.0, vec![]),
            make_task(3, 2e12, 10.0, vec![1, 2]),
        ];
        let result = sched.schedule(&tasks);
        assert_eq!(result.len(), 3);
        let t1 = result.iter().find(|e| e.task_id == 1).unwrap();
        let t3 = result.iter().find(|e| e.task_id == 3).unwrap();
        // Task 1 auf GPU (10 TFLOP/s), 5e12/10e12 = 0.5s
        assert_eq!(t1.accel_id, 1);
        assert!((t1.finish_time - 0.5).abs() < 0.001);
        // Task 3 muss nach Task 1 und 2 starten
        assert!(t3.start_time >= t1.finish_time);
    }

    #[test]
    fn test_heterogeneous_assignment() {
        let mut sched = DaHeftScheduler::new();
        sched.add_accelerator(Box::new(make_accel(1, AcceleratorType::Gpu, 50e12)));
        sched.add_accelerator(Box::new(make_accel(2, AcceleratorType::Cpu, 1e12)));
        let tasks = vec![make_task(1, 10e12, 100.0, vec![])];
        let result = sched.schedule(&tasks);
        assert_eq!(result[0].accel_id, 1);
        assert!((result[0].finish_time - 0.2).abs() < 0.001);
    }

    #[test]
    fn test_dependency_ordering() {
        let mut sched = DaHeftScheduler::new();
        sched.add_accelerator(Box::new(make_accel(1, AcceleratorType::Gpu, 10e12)));
        let tasks = vec![
            make_task(1, 5e12, 100.0, vec![]),
            make_task(2, 3e12, 100.0, vec![1]),
            make_task(3, 2e12, 100.0, vec![2]),
        ];
        let result = sched.schedule(&tasks);
        let t1 = result.iter().find(|e| e.task_id == 1).unwrap();
        let t2 = result.iter().find(|e| e.task_id == 2).unwrap();
        let t3 = result.iter().find(|e| e.task_id == 3).unwrap();
        assert!(t2.start_time >= t1.finish_time);
        assert!(t3.start_time >= t2.finish_time);
    }

    #[test]
    fn test_deadline_awareness() {
        let mut sched = DaHeftScheduler::new();
        sched.add_accelerator(Box::new(make_accel(1, AcceleratorType::Cpu, 1e12)));
        let tasks = vec![
            make_task(1, 5e12, 3.0, vec![]),
            make_task(2, 1e12, 10.0, vec![]),
        ];
        let result = sched.schedule(&tasks);
        let t1 = result.iter().find(|e| e.task_id == 1).unwrap();
        let t2 = result.iter().find(|e| e.task_id == 2).unwrap();
        assert!(!t1.deadline_met);
        assert!(t2.deadline_met);
        assert_eq!(sched.deadline_misses(&result), 1);
    }

    #[test]
    fn test_thermal_throttling() {
        let mut sched = DaHeftScheduler::new();
        let mut hot_gpu = make_accel(1, AcceleratorType::Gpu, 100e12);
        hot_gpu.temp = 90.0;
        sched.add_accelerator(Box::new(hot_gpu));
        sched.add_accelerator(Box::new(make_accel(2, AcceleratorType::Cpu, 2e12)));
        let tasks = vec![make_task(1, 10e12, 100.0, vec![])];
        let result = sched.schedule(&tasks);
        assert_eq!(result[0].accel_id, 2);
    }

    #[test]
    fn test_memory_constraint() {
        let mut sched = DaHeftScheduler::new();
        let mut small_gpu = make_accel(1, AcceleratorType::Gpu, 50e12);
        small_gpu.mem_mb = 50;
        sched.add_accelerator(Box::new(small_gpu));
        sched.add_accelerator(Box::new(make_accel(2, AcceleratorType::Cpu, 2e12)));
        let tasks = vec![make_task(1, 10e12, 100.0, vec![])];
        let result = sched.schedule(&tasks);
        assert_eq!(result[0].accel_id, 2);
    }

    #[test]
    fn test_upward_rank_priority() {
        let mut sched = DaHeftScheduler::new();
        sched.add_accelerator(Box::new(make_accel(1, AcceleratorType::Gpu, 10e12)));
        // DAG: 30 -> 20 -> 10 (30 is entry, 10 is exit)
        let tasks = vec![
            make_task(10, 1e12, 100.0, vec![20]),
            make_task(20, 1e12, 100.0, vec![30]),
            make_task(30, 1e12, 100.0, vec![]),
        ];
        let result = sched.schedule(&tasks);
        assert_eq!(result.len(), 3);
        let t30 = result.iter().find(|e| e.task_id == 30).unwrap();
        let t20 = result.iter().find(|e| e.task_id == 20).unwrap();
        let t10 = result.iter().find(|e| e.task_id == 10).unwrap();
        assert!(t20.start_time >= t30.finish_time);
        assert!(t10.start_time >= t20.finish_time);
    }

    #[test]
    fn test_empty_inputs() {
        let mut sched = DaHeftScheduler::new();
        assert!(sched.schedule(&[]).is_empty());
        sched.add_accelerator(Box::new(make_accel(1, AcceleratorType::Cpu, 1e12)));
        assert!(sched.schedule(&[]).is_empty());
    }

    #[test]
    fn test_utilization_calculation() {
        let mut sched = DaHeftScheduler::new();
        sched.add_accelerator(Box::new(make_accel(1, AcceleratorType::Gpu, 10e12)));
        sched.add_accelerator(Box::new(make_accel(2, AcceleratorType::Cpu, 2e12)));
        let tasks = vec![
            make_task(1, 5e12, 100.0, vec![]),
            make_task(2, 2e12, 100.0, vec![]),
        ];
        let result = sched.schedule(&tasks);
        let util = sched.utilization(&result);
        assert!(util.contains_key(&1));
        assert!(util.contains_key(&2));
    }

    #[test]
    fn test_all_thermal_overload() {
        let mut sched = DaHeftScheduler::new();
        let mut hot1 = make_accel(1, AcceleratorType::Gpu, 10e12);
        hot1.temp = 95.0;
        let mut hot2 = make_accel(2, AcceleratorType::Cpu, 2e12);
        hot2.temp = 90.0;
        sched.add_accelerator(Box::new(hot1));
        sched.add_accelerator(Box::new(hot2));
        let tasks = vec![make_task(1, 1e12, 10.0, vec![])];
        let result = sched.schedule(&tasks);
        assert_eq!(result.len(), 0);
    }
}
