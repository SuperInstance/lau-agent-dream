use crate::dream_cycle::DreamType;
use crate::experience::{random_id, Experience};
use crate::tile::DreamTile;

/// Processes failures and negative emotional experiences intensely.
pub struct NightmareEngine {
    /// How intensely to examine failures.
    pub intensity: f64,
}

impl NightmareEngine {
    pub fn new(intensity: f64) -> Self {
        Self { intensity }
    }

    /// Filter negative emotion experiences, amplify them, extract lessons.
    pub fn process(&self, experiences: &[Experience]) -> Vec<DreamTile> {
        let negative: Vec<&Experience> = experiences
            .iter()
            .filter(|e| e.emotion < 0.0)
            .collect();

        if negative.is_empty() {
            return Vec::new();
        }

        negative
            .into_iter()
            .map(|exp| {
                let amplification = exp.emotion.abs() * self.intensity;
                let lesson = format!(
                    "nightmare processed (intensity {:.2}): {} — lesson learned, emotional weight {:.2}",
                    self.intensity, exp.content, amplification
                );

                DreamTile {
                    id: random_id(),
                    content: lesson,
                    source_experiences: vec![exp.id.clone()],
                    novelty: 0.3, // Moderate: new perspective on failure
                    confidence: 0.9, // High: failures teach reliably
                    dream_type: DreamType::Nightmare,
                }
            })
            .collect()
    }
}
