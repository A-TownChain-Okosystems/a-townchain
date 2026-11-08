// PROVENANZ (Org-Adoption 13.09.2026): Quelle: privates Repo ShivaCoreDev/A-TownChain--kosystems- (05.07.2026). Archiv-Referenz (nicht buildbar gepflegt).

#![no_std]
#![no_main]

use shiva_agent::{agent_task, ShivaAgent, AgentContext};
use shiva_tensor::Tensor;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use alloc::vec::Vec;
use alloc::string::String;

// ===================================================
// 1. Task-Definitionen
// ===================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataTask {
    Summary { data: Vec<f64> },
    Regression { x: Vec<f64>, y: Vec<f64> },
    Correlation { data: Vec<Vec<f64>> },
    DetectOutliers { data: Vec<f64>, threshold: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataResult {
    Summary { count: usize, mean: f64, median: f64, std_dev: f64, min: f64, max: f64 },
    Regression { slope: f64, intercept: f64, r2: f64 },
    Correlation { matrix: Vec<Vec<f64>> },
    Outliers { indices: Vec<usize>, count: usize },
    Error(String),
}

// ===================================================
// 2. Agent-Implementierung
// ===================================================
#[derive(Default)]
pub struct DataAnalystAgent {
    model: Option<NeuralNetwork>,
}

#[shiva_agent]
impl DataAnalystAgent {
    pub fn new() -> Self {
        DataAnalystAgent { model: None }
    }

    #[agent_task("summary")]
    pub fn summary(&self, data: Vec<f64>) -> DataResult {
        if data.is_empty() {
            return DataResult::Error("Empty dataset".to_string());
        }
        let mut sorted = data.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let count = data.len();
        let mean = data.iter().sum::<f64>() / count as f64;
        let median = if count % 2 == 0 {
            (sorted[count/2 - 1] + sorted[count/2]) / 2.0
        } else {
            sorted[count/2]
        };
        let variance = data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / count as f64;
        let std_dev = variance.sqrt();
        let min = *sorted.first().unwrap();
        let max = *sorted.last().unwrap();
        
        DataResult::Summary { count, mean, median, std_dev, min, max }
    }

    #[agent_task("regression")]
    pub fn regression(&self, x: Vec<f64>, y: Vec<f64>) -> DataResult {
        if x.len() != y.len() || x.is_empty() {
            return DataResult::Error("Invalid input".to_string());
        }
        let n = x.len() as f64;
        let sum_x = x.iter().sum::<f64>();
        let sum_y = y.iter().sum::<f64>();
        let sum_xy = x.iter().zip(&y).map(|(a, b)| a * b).sum::<f64>();
        let sum_x2 = x.iter().map(|&a| a * a).sum::<f64>();
        
        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);
        let intercept = (sum_y - slope * sum_x) / n;
        
        // R² berechnen
        let y_mean = sum_y / n;
        let ss_tot = y.iter().map(|&yi| (yi - y_mean).powi(2)).sum::<f64>();
        let ss_res = y.iter().zip(&x).map(|(&yi, &xi)| (yi - (slope * xi + intercept)).powi(2)).sum::<f64>();
        let r2 = if ss_tot != 0.0 { 1.0 - ss_res / ss_tot } else { 0.0 };
        
        DataResult::Regression { slope, intercept, r2 }
    }

    #[agent_task("detect_outliers")]
    pub fn detect_outliers(&self, data: Vec<f64>, threshold: f64) -> DataResult {
        if data.is_empty() {
            return DataResult::Error("Empty dataset".to_string());
        }
        let mean = data.iter().sum::<f64>() / data.len() as f64;
        let std_dev = (data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64).sqrt();
        let mut indices = Vec::new();
        for (i, &value) in data.iter().enumerate() {
            if (value - mean).abs() > threshold * std_dev {
                indices.push(i);
            }
        }
        DataResult::Outliers { indices, count: indices.len() }
    }

    pub fn init(&mut self, ctx: &AgentContext) -> Result<(), String> {
        log::info!("📊 DataAnalyst initialisiert mit ID: {}", ctx.agent_id);
        Ok(())
    }

    pub fn shutdown(&self) {
        log::info!("📊 DataAnalyst wird beendet.");
    }
}

// ===================================================
// 4. WASM-Exporte
// ===================================================
#[wasm_bindgen]
pub fn process_task(task_json: &str) -> String {
    let agent = DataAnalystAgent::new();
    let task: DataTask = serde_json::from_str(task_json).unwrap();
    let result = match task {
        DataTask::Summary { data } => agent.summary(data),
        DataTask::Regression { x, y } => agent.regression(x, y),
        DataTask::DetectOutliers { data, threshold } => agent.detect_outliers(data, threshold),
        DataTask::Correlation { data } => {
            // Vereinfacht: Korrelationsmatrix berechnen
            let mut matrix = Vec::new();
            for i in 0..data.len() {
                let mut row = Vec::new();
                for j in 0..data.len() {
                    if i == j {
                        row.push(1.0);
                    } else {
                        row.push(0.0); // Vereinfacht
                    }
                }
                matrix.push(row);
            }
            DataResult::Correlation { matrix }
        }
    };
    serde_json::to_string(&result).unwrap()
}

#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        console_log::init_with_level(log::Level::Info).unwrap();
    }
}
