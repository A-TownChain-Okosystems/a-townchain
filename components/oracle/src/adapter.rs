// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Datenquellen-Adapter Interface & Implementierungen.

use crate::feed::Report;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdapterError {
    FetchFailed(String),
    InvalidFormat,
    Offline,
}

/// Trait für Datenquellen-Adapter (Oracle Data Source Adapter)
pub trait DataSource {
    fn source_id(&self) -> u64;
    fn fetch(&self, key: &str) -> Result<Report, AdapterError>;
}

/// Statischer/Mock Datenquellen-Adapter
pub struct StaticDataSource {
    pub id: u64,
    pub values: HashMap<String, u64>,
}

impl StaticDataSource {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            values: HashMap::new(),
        }
    }

    pub fn set_value(&mut self, key: impl Into<String>, value: u64) {
        self.values.insert(key.into(), value);
    }
}

impl DataSource for StaticDataSource {
    fn source_id(&self) -> u64 {
        self.id
    }

    fn fetch(&self, key: &str) -> Result<Report, AdapterError> {
        match self.values.get(key) {
            Some(&val) => Ok(Report {
                source: self.id,
                value: val,
            }),
            None => Err(AdapterError::FetchFailed(format!("Key '{}' nicht vorhanden", key))),
        }
    }
}
