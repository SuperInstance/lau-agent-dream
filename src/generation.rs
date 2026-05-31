use rand::seq::SliceRandom;
use rand::Rng;

use crate::dream_cycle::DreamType;
use crate::experience::{random_id, Experience};
use crate::tile::DreamTile;

/// Creates novel combinations from dissimilar experiences.
pub struct GenerationEngine {
    /// How far to push combinations (0.0-1.0).
    pub creativity: f64,
}

impl GenerationEngine {
    pub fn new(creativity: f64) -> Self {
        Self { creativity }
    }

    /// Pick random pairs of dissimilar experiences and blend them.
    pub fn generate(&self, experiences: &[Experience]) -> Vec<DreamTile> {
        if experiences.len() < 2 {
            return Vec::new();
        }

        let mut rng = rand::thread_rng();
        let mut tiles = Vec::new();

        let mut indices: Vec<usize> = (0..experiences.len()).collect();
        indices.shuffle(&mut rng);

        for chunk in indices.chunks(2) {
            if chunk.len() < 2 {
                continue;
            }
            let a = &experiences[chunk[0]];
            let b = &experiences[chunk[1]];

            let dissim = 1.0 - a.similarity(b);

            // Only combine if dissimilar enough (modulated by creativity)
            if dissim < (1.0 - self.creativity) * 0.3 {
                continue;
            }

            let blend_factor: f64 = rng.gen_range(0.3..0.7);
            let novelty = dissim;
            let confidence = 1.0 - (novelty * 0.5);

            // Blend content
            let words_a: Vec<&str> = a.content.split_whitespace().collect();
            let words_b: Vec<&str> = b.content.split_whitespace().collect();

            let blended = if words_a.is_empty() && words_b.is_empty() {
                String::from("(generated)")
            } else if words_a.is_empty() {
                b.content.clone()
            } else if words_b.is_empty() {
                a.content.clone()
            } else {
                let take_a = ((words_a.len() as f64) * blend_factor).ceil() as usize;
                let take_b = ((words_b.len() as f64) * (1.0 - blend_factor)).ceil() as usize;
                let mut parts: Vec<&str> = words_a.into_iter().take(take_a).collect();
                parts.extend(words_b.into_iter().take(take_b));
                parts.join(" ")
            };

            tiles.push(DreamTile {
                id: random_id(),
                content: format!("generated: {}", blended),
                source_experiences: vec![a.id.clone(), b.id.clone()],
                novelty,
                confidence,
                dream_type: DreamType::Generation,
            });
        }

        tiles
    }
}
