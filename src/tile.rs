use serde::{Deserialize, Serialize};

use crate::dream_cycle::DreamType;
use crate::experience::random_id;

/// A product of dreaming — a consolidated, replayed, or generated memory tile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamTile {
    pub id: String,
    pub content: String,
    pub source_experiences: Vec<String>,
    pub novelty: f64,
    pub confidence: f64,
    pub dream_type: DreamType,
}

impl DreamTile {
    pub fn new(content: String, source_experiences: Vec<String>, novelty: f64, confidence: f64, dream_type: DreamType) -> Self {
        Self {
            id: random_id(),
            content,
            source_experiences,
            novelty,
            confidence,
            dream_type,
        }
    }
}
