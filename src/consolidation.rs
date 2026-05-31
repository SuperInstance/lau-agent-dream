use crate::dream_cycle::DreamType;
use crate::experience::{random_id, Experience};
use crate::tile::DreamTile;

/// Merges similar experiences into consolidated dream tiles.
pub struct ConsolidationEngine {
    pub similarity_threshold: f64,
}

impl ConsolidationEngine {
    pub fn new(similarity_threshold: f64) -> Self {
        Self { similarity_threshold }
    }

    /// Cluster experiences by similarity and produce one tile per cluster.
    pub fn consolidate(&self, experiences: &[Experience]) -> Vec<DreamTile> {
        if experiences.is_empty() {
            return Vec::new();
        }

        // Simple greedy clustering
        let mut clusters: Vec<Vec<usize>> = Vec::new();
        let mut assigned = vec![false; experiences.len()];

        for i in 0..experiences.len() {
            if assigned[i] {
                continue;
            }
            let mut cluster = vec![i];
            assigned[i] = true;
            for j in (i + 1)..experiences.len() {
                if assigned[j] {
                    continue;
                }
                if experiences[i].similarity(&experiences[j]) >= self.similarity_threshold {
                    cluster.push(j);
                    assigned[j] = true;
                }
            }
            clusters.push(cluster);
        }

        clusters
            .into_iter()
            .map(|indices| {
                let sources: Vec<&Experience> = indices.iter().map(|&i| &experiences[i]).collect();
                let source_ids: Vec<String> = sources.iter().map(|e| e.id.clone()).collect();

                // Merge content
                let combined_content = sources
                    .iter()
                    .map(|e| e.content.as_str())
                    .collect::<Vec<_>>()
                    .join(" | ");

                // Merge tags (dedup)
                let mut all_tags: Vec<String> = sources
                    .iter()
                    .flat_map(|e| e.tags.clone())
                    .collect();
                all_tags.sort();
                all_tags.dedup();

                let content = if sources.len() == 1 {
                    sources[0].content.clone()
                } else {
                    let tag_str = all_tags.join(", ");
                    format!("[{}] {}", tag_str, combined_content)
                };

                DreamTile {
                    id: random_id(),
                    content,
                    source_experiences: source_ids,
                    novelty: 0.0, // Pure consolidation adds no new info
                    confidence: 1.0,
                    dream_type: DreamType::Consolidation,
                }
            })
            .collect()
    }
}
