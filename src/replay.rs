use crate::dream_cycle::DreamType;
use crate::experience::{random_id, Experience};
use crate::tile::DreamTile;

/// Re-experiences important moments with slight reinterpretation.
pub struct ReplayEngine {
    pub replay_count: usize,
}

impl ReplayEngine {
    pub fn new(replay_count: usize) -> Self {
        Self { replay_count }
    }

    /// Sort by importance * |emotion|, take top n, re-express with slight variation.
    pub fn replay(&self, experiences: &[Experience], n: usize) -> Vec<DreamTile> {
        if experiences.is_empty() {
            return Vec::new();
        }

        let mut indexed: Vec<(usize, f64)> = experiences
            .iter()
            .enumerate()
            .map(|(i, e)| (i, e.importance * e.emotion.abs()))
            .collect();
        indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        indexed
            .into_iter()
            .take(n)
            .map(|(i, _)| {
                let exp = &experiences[i];
                DreamTile {
                    id: random_id(),
                    content: format!("replayed: {}", exp.content),
                    source_experiences: vec![exp.id.clone()],
                    novelty: 0.1, // Slight reinterpretation
                    confidence: 0.95,
                    dream_type: DreamType::Replay,
                }
            })
            .collect()
    }
}
