use serde::{Deserialize, Serialize};

/// The kind of dream cycle being executed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DreamType {
    Consolidation,
    Replay,
    Generation,
    Nightmare,
    Lucid,
    Prophetic,
}

impl DreamType {
    pub fn name(&self) -> &'static str {
        match self {
            DreamType::Consolidation => "Consolidation",
            DreamType::Replay => "Replay",
            DreamType::Generation => "Generation",
            DreamType::Nightmare => "Nightmare",
            DreamType::Lucid => "Lucid",
            DreamType::Prophetic => "Prophetic",
        }
    }
}

/// A complete dream session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamCycle {
    pub agent_id: String,
    pub dream_type: DreamType,
    pub experiences: Vec<crate::experience::Experience>,
    pub consolidated: Vec<crate::tile::DreamTile>,
    pub start_time: u64,
    pub end_time: u64,
    pub energy_used: f64,
}

impl DreamCycle {
    pub fn new(agent_id: &str, dream_type: DreamType, experiences: Vec<crate::experience::Experience>) -> Self {
        Self {
            agent_id: agent_id.to_string(),
            dream_type,
            experiences,
            consolidated: Vec::new(),
            start_time: 0,
            end_time: 0,
            energy_used: 0.0,
        }
    }

    /// Execute the dream cycle using the appropriate engine.
    pub fn run(&mut self) -> Vec<crate::tile::DreamTile> {
        self.start_time = now_ms();

        let tiles = match self.dream_type {
            DreamType::Consolidation => {
                let engine = crate::consolidation::ConsolidationEngine::new(0.3);
                engine.consolidate(&self.experiences)
            }
            DreamType::Replay => {
                let engine = crate::replay::ReplayEngine::new(10);
                engine.replay(&self.experiences, 10)
            }
            DreamType::Generation => {
                let engine = crate::generation::GenerationEngine::new(0.5);
                engine.generate(&self.experiences)
            }
            DreamType::Nightmare => {
                let engine = crate::nightmare::NightmareEngine::new(1.0);
                engine.process(&self.experiences)
            }
            DreamType::Lucid => {
                // Lucid dreams use generation with higher creativity
                let engine = crate::generation::GenerationEngine::new(0.8);
                let mut tiles = engine.generate(&self.experiences);
                for t in &mut tiles {
                    t.dream_type = DreamType::Lucid;
                }
                tiles
            }
            DreamType::Prophetic => {
                // Prophetic dreams use consolidation + generation hybrid
                let cons = crate::consolidation::ConsolidationEngine::new(0.4);
                let gen = crate::generation::GenerationEngine::new(0.6);
                let mut tiles = cons.consolidate(&self.experiences);
                tiles.extend(gen.generate(&self.experiences));
                tiles
            }
        };

        // Calculate energy: proportional to experiences processed
        let num_tiles = tiles.len() as f64;
        let num_exp = self.experiences.len().max(1) as f64;
        self.energy_used = num_exp * 0.1 + num_tiles * 0.05;

        self.end_time = now_ms();
        self.consolidated = tiles.clone();
        tiles
    }

    pub fn duration_ms(&self) -> u64 {
        self.end_time.saturating_sub(self.start_time)
    }

    /// Experiences compressed into tiles: experiences / tiles.
    pub fn compression_ratio(&self) -> f64 {
        if self.consolidated.is_empty() {
            if self.experiences.is_empty() {
                1.0
            } else {
                f64::INFINITY
            }
        } else {
            self.experiences.len() as f64 / self.consolidated.len() as f64
        }
    }

    /// Tiles produced per energy unit.
    pub fn energy_efficiency(&self) -> f64 {
        if self.energy_used == 0.0 {
            0.0
        } else {
            self.consolidated.len() as f64 / self.energy_used
        }
    }
}

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}
