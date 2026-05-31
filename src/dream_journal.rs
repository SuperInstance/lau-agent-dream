use std::collections::HashMap;

use crate::dream_cycle::{DreamCycle, DreamType};

/// Tracks dream history across cycles.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DreamJournal {
    pub entries: Vec<DreamCycle>,
    pub agent_id: String,
}

impl DreamJournal {
    pub fn new(agent_id: &str) -> Self {
        Self {
            entries: Vec::new(),
            agent_id: agent_id.to_string(),
        }
    }

    pub fn record(&mut self, cycle: DreamCycle) {
        self.entries.push(cycle);
    }

    pub fn total_dreams(&self) -> usize {
        self.entries.len()
    }

    pub fn dream_types_distribution(&self) -> HashMap<String, usize> {
        let mut map = HashMap::new();
        for entry in &self.entries {
            *map.entry(entry.dream_type.name().to_string()).or_insert(0) += 1;
        }
        map
    }

    pub fn average_compression(&self) -> f64 {
        if self.entries.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.entries.iter().map(|e| e.compression_ratio()).sum();
        sum / self.entries.len() as f64
    }

    pub fn most_productive_type(&self) -> DreamType {
        let mut counts: HashMap<DreamType, usize> = HashMap::new();
        for entry in &self.entries {
            let tile_count = entry.consolidated.len();
            *counts.entry(entry.dream_type).or_insert(0) += tile_count;
        }
        counts
            .into_iter()
            .max_by_key(|(_, v)| *v)
            .map(|(k, _)| k)
            .unwrap_or(DreamType::Consolidation)
    }

    pub fn energy_spent(&self) -> f64 {
        self.entries.iter().map(|e| e.energy_used).sum()
    }

    pub fn tiles_produced(&self) -> usize {
        self.entries.iter().map(|e| e.consolidated.len()).sum()
    }
}
