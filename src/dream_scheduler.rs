use std::collections::HashMap;

use crate::dream_cycle::DreamType;
use crate::experience::Experience;

/// Decides when and what to dream.
pub struct DreamScheduler {
    pub energy_budget: f64,
    pub min_idle_time_ms: u64,
}

impl DreamScheduler {
    pub fn new(energy_budget: f64, min_idle_time_ms: u64) -> Self {
        Self { energy_budget, min_idle_time_ms }
    }

    pub fn should_dream(&self, idle_time_ms: u64, unprocessed: usize) -> bool {
        idle_time_ms >= self.min_idle_time_ms && unprocessed > 0
    }

    /// Choose dream type based on the distribution of experiences.
    pub fn choose_dream_type(&self, experiences: &[Experience]) -> DreamType {
        if experiences.is_empty() {
            return DreamType::Consolidation;
        }

        let n = experiences.len() as f64;

        // Fraction that are negative
        let neg_frac = experiences.iter().filter(|e| e.emotion < 0.0).count() as f64 / n;

        // Average pairwise similarity (sample if too many)
        let avg_sim = compute_avg_similarity(experiences);

        // Variety: number of unique tag sets
        let unique_tags: usize = {
            let mut sets: Vec<u64> = experiences
                .iter()
                .map(|e| {
                    let mut h = std::collections::hash_map::DefaultHasher::new();
                    let mut sorted = e.tags.clone();
                    sorted.sort();
                    for t in &sorted {
                        std::hash::Hash::hash_slice(t.as_bytes(), &mut h);
                    }
                    std::hash::Hasher::finish(&h)
                })
                .collect();
            sets.sort();
            sets.dedup();
            sets.len()
        };
        let variety = unique_tags as f64 / n;

        // Max importance
        let max_importance = experiences
            .iter()
            .map(|e| e.importance)
            .fold(0.0_f64, f64::max);

        // Decision logic
        if neg_frac > 0.6 {
            DreamType::Nightmare
        } else if avg_sim > 0.5 {
            DreamType::Consolidation
        } else if variety < 0.3 {
            DreamType::Generation
        } else if max_importance > 0.8 {
            DreamType::Replay
        } else if self.energy_budget > 1.0 {
            DreamType::Lucid
        } else {
            DreamType::Prophetic
        }
    }
}

fn compute_avg_similarity(experiences: &[Experience]) -> f64 {
    if experiences.len() < 2 {
        return 0.0;
    }
    let max_pairs = 50;
    let mut sum = 0.0;
    let mut count = 0;
    for i in 0..experiences.len() {
        for j in (i + 1)..experiences.len() {
            sum += experiences[i].similarity(&experiences[j]);
            count += 1;
            if count >= max_pairs {
                break;
            }
        }
        if count >= max_pairs {
            break;
        }
    }
    if count == 0 { 0.0 } else { sum / count as f64 }
}

/// For use in scheduler choice tests — compute distribution of dream types.
pub fn _dream_type_distribution(cycles: &[(DreamType, usize)]) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for (dt, count) in cycles {
        *map.entry(dt.name().to_string()).or_insert(0) += count;
    }
    map
}
