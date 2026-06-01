# lau-agent-dream

> The dream cycle — what happens when an agent is idle

## What This Does

The dream cycle — what happens when an agent is idle. Part of the PLATO/LAU ecosystem — a mathematically rigorous framework for building educational agents that learn, teach, and evolve.

## The Key Idea

This crate implements the core abstractions needed for its domain, with a focus on correctness, composability, and conservation guarantees. Every public type is serializable (serde), every algorithm is tested, and every invariant is verified.

## Install

```bash
cargo add lau-agent-dream
```

## Quick Start

See the API Reference below for complete usage. Key entry points:

```rust
use lau_agent_dream::*;
// See types and methods below for complete usage
```

## API Reference

```rust
pub struct NightmareEngine 
    pub fn new(intensity: f64) -> Self 
    pub fn process(&self, experiences: &[Experience]) -> Vec<DreamTile> 
pub struct DreamTile 
    pub fn new(content: String, source_experiences: Vec<String>, novelty: f64, confidence: f64, dream_type: DreamType) -> Self 
pub struct ConsolidationEngine 
    pub fn new(similarity_threshold: f64) -> Self 
    pub fn consolidate(&self, experiences: &[Experience]) -> Vec<DreamTile> 
pub struct DreamScheduler 
    pub fn new(energy_budget: f64, min_idle_time_ms: u64) -> Self 
    pub fn should_dream(&self, idle_time_ms: u64, unprocessed: usize) -> bool 
    pub fn choose_dream_type(&self, experiences: &[Experience]) -> DreamType 
pub fn _dream_type_distribution(cycles: &[(DreamType, usize)]) -> HashMap<String, usize> 
pub enum DreamType 
    pub fn name(&self) -> &'static str 
pub struct DreamCycle 
    pub fn new(agent_id: &str, dream_type: DreamType, experiences: Vec<crate::experience::Experience>) -> Self 
    pub fn run(&mut self) -> Vec<crate::tile::DreamTile> 
    pub fn duration_ms(&self) -> u64 
    pub fn compression_ratio(&self) -> f64 
    pub fn energy_efficiency(&self) -> f64 
pub struct DreamJournal 
    pub fn new(agent_id: &str) -> Self 
    pub fn record(&mut self, cycle: DreamCycle) 
    pub fn total_dreams(&self) -> usize 
    pub fn dream_types_distribution(&self) -> HashMap<String, usize> 
    pub fn average_compression(&self) -> f64 
    pub fn most_productive_type(&self) -> DreamType 
    pub fn energy_spent(&self) -> f64 
    pub fn tiles_produced(&self) -> usize 
pub struct ReplayEngine 
    pub fn new(replay_count: usize) -> Self 
    pub fn replay(&self, experiences: &[Experience], n: usize) -> Vec<DreamTile> 
pub struct GenerationEngine 
    pub fn new(creativity: f64) -> Self 
    pub fn generate(&self, experiences: &[Experience]) -> Vec<DreamTile> 
pub struct Experience 
    pub fn similarity(&self, other: &Experience) -> f64 
pub fn random_id() -> String 
pub fn _pick_random<T: Clone>(items: &[T], n: usize) -> Vec<T> 
```

## How It Works

Read the source in `src/` for full implementation details. All algorithms are documented with inline comments explaining the mathematical foundations.

## The Math

This crate implements formal mathematical constructs. See the source documentation for theorem statements and proofs of correctness.

## Testing

**54 tests** covering construction, serialization, correctness properties, edge cases, and composability with other lau-* crates.

## License

MIT
