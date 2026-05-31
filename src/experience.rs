use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

/// A waking memory fed into the dream cycle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    pub id: String,
    pub timestamp: u64,
    pub content: String,
    pub importance: f64,
    /// Valence: -1.0 (very negative) to 1.0 (very positive).
    pub emotion: f64,
    pub tags: Vec<String>,
    pub room_id: String,
}

impl Experience {
    /// Compute similarity to another experience based on tag overlap + content overlap.
    pub fn similarity(&self, other: &Experience) -> f64 {
        let tag_sim = if self.tags.is_empty() && other.tags.is_empty() {
            0.0
        } else {
            let common = self
                .tags
                .iter()
                .filter(|t| other.tags.contains(t))
                .count() as f64;
            let total = (self.tags.len() + other.tags.len()) as f64;
            2.0 * common / total
        };

        let content_sim = if self.content.is_empty() && other.content.is_empty() {
            1.0
        } else if self.content.is_empty() || other.content.is_empty() {
            0.0
        } else {
            let words_a: std::collections::HashSet<&str> =
                self.content.split_whitespace().collect();
            let words_b: std::collections::HashSet<&str> =
                other.content.split_whitespace().collect();
            let common = words_a.intersection(&words_b).count() as f64;
            let total = (words_a.len() + words_b.len()) as f64;
            if total == 0.0 {
                0.0
            } else {
                2.0 * common / total
            }
        };

        0.5 * tag_sim + 0.5 * content_sim
    }
}

/// Helper to create a random ID.
pub fn random_id() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    format!("{:016x}", rng.gen::<u64>())
}

/// Pick `n` random items from a slice.
pub fn _pick_random<T: Clone>(items: &[T], n: usize) -> Vec<T> {
    let mut rng = rand::thread_rng();
    let mut indices: Vec<usize> = (0..items.len()).collect();
    indices.shuffle(&mut rng);
    indices.into_iter().take(n).map(|i| items[i].clone()).collect()
}
