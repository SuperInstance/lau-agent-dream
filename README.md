# lau-agent-dream

**The dream cycle — what happens when an agent is idle.**

When an AI agent isn't actively exploring or deciding, it should still be learning. `lau-agent-dream` implements a complete offline experience-processing pipeline inspired by sleep-phase memory consolidation in biological brains. Waking experiences go in; consolidated, replayed, or freshly generated "dream tiles" come out.

---

## What This Does

An autonomous agent accumulates raw `Experience` records during its active phase — things it saw, did, failed at, or discovered. During idle periods, the **dream cycle** processes those experiences through one of six specialized engines:

| Dream Type   | Purpose                                                                 |
|-------------|-------------------------------------------------------------------------|
| **Consolidation** | Cluster similar memories into compressed summaries                |
| **Replay**        | Re-experience important moments with slight reinterpretation      |
| **Generation**    | Blend dissimilar experiences into novel combinations              |
| **Nightmare**     | Intensely process failures to extract lessons                    |
| **Lucid**         | High-creativity generation (dreamer-aware exploration)           |
| **Prophetic**     | Hybrid consolidation + generation for predictive insights        |

The output of every dream is a set of **DreamTiles** — structured, compressed memory fragments with novelty and confidence scores that the agent can use in future decisions.

---

## Key Idea

Biological sleep isn't passive. During REM and deep sleep the brain **replays, consolidates, and recombines** memories — strengthening important ones, weakening noise, and discovering hidden connections. This crate brings that same principle to autonomous agents:

- **Consolidation** ≈ slow-wave sleep: merge redundant memories, free cognitive space.
- **Replay** ≈ hippocampal replay: rehearse high-salience events to lock in learning.
- **Generation** ≈ REM creativity: smash unrelated experiences together to discover novelty.
- **Nightmares** ≈ threat simulation: process failures intensely so they don't repeat.
- **Lucid/Prophetic** ≈ hybrid modes for rich exploration.

The `DreamScheduler` decides *when* to dream and *which* engine to use, based on emotional distribution, tag variety, similarity clustering, and energy budget.

---

## Install

Add to your `Cargo.toml`:

```toml
[dependencies]
lau-agent-dream = "0.1"
```

Or use via git:

```toml
[dependencies]
lau-agent-dream = { git = "https://github.com/SuperInstance/lau-agent-dream" }
```

Requires **Rust 2021 edition** (1.56+).

### Dependencies

| Crate      | Purpose                          |
|-----------|----------------------------------|
| `serde`   | Serialization of experiences & tiles |
| `serde_json` | JSON serialization support   |
| `rand`    | Randomized generation and shuffling |

---

## Quick Start

```rust
use lau_agent_dream::*;

// 1. Collect experiences during the agent's active phase
let experiences = vec![
    Experience {
        id: "e1".into(),
        timestamp: 1000,
        content: "Explored the northern corridor".into(),
        importance: 0.7,
        emotion: 0.4,
        tags: vec!["exploration".into()],
        room_id: "corridor".into(),
    },
    Experience {
        id: "e2".into(),
        timestamp: 3000,
        content: "Failed to open the sealed door".into(),
        importance: 0.9,
        emotion: -0.8,
        tags: vec!["failure".into(), "door".into()],
        room_id: "sealed".into(),
    },
];

// 2. Run a dream cycle
let mut cycle = DreamCycle::new("agent-7", DreamType::Consolidation, experiences.clone());
let tiles = cycle.run();

for tile in &tiles {
    println!("[{}] {} (novelty={:.2}, confidence={:.2})",
        &tile.id[..8], tile.content, tile.novelty, tile.confidence);
}

// 3. Let the scheduler decide
let scheduler = DreamScheduler::new(5.0, 3000); // energy budget, min idle ms
if scheduler.should_dream(5000, experiences.len()) {
    let dream_type = scheduler.choose_dream_type(&experiences);
    println!("Scheduler chose: {:?}", dream_type);
}

// 4. Track history in a journal
let mut journal = DreamJournal::new("agent-7");
journal.record(cycle);
println!("Average compression: {:.2}", journal.average_compression());
println!("Most productive type: {:?}", journal.most_productive_type());
```

Run the bundled demo:

```bash
cargo run
```

---

## API Reference

### `Experience`

A waking memory fed into the dream cycle.

| Field          | Type         | Description                                    |
|---------------|-------------|------------------------------------------------|
| `id`          | `String`    | Unique identifier                              |
| `timestamp`   | `u64`       | Unix timestamp (ms)                            |
| `content`     | `String`    | Free-text description of what happened         |
| `importance`  | `f64`       | How significant (0.0–1.0)                      |
| `emotion`     | `f64`       | Valence: −1.0 (very negative) to +1.0 (very positive) |
| `tags`        | `Vec<String>` | Categorical tags for similarity matching     |
| `room_id`     | `String`    | Spatial/agent context identifier               |

**`similarity(&self, other: &Experience) -> f64`** — Computes a similarity score (0.0–1.0) using a 50/50 blend of Jaccard tag overlap and word-level content overlap.

---

### `DreamTile`

A product of dreaming — a consolidated, replayed, or generated memory tile.

| Field                 | Type           | Description                             |
|----------------------|----------------|-----------------------------------------|
| `id`                 | `String`       | Unique tile ID                          |
| `content`            | `String`       | Processed content                       |
| `source_experiences` | `Vec<String>`  | IDs of experiences that produced this   |
| `novelty`            | `f64`          | How novel the tile is (0.0–1.0)        |
| `confidence`         | `f64`          | Reliability of the tile (0.0–1.0)      |
| `dream_type`         | `DreamType`    | Which engine produced it               |

---

### `DreamCycle`

A complete dream session. Wraps the engine selection and execution.

```rust
let mut cycle = DreamCycle::new("agent-id", DreamType::Replay, experiences);
let tiles = cycle.run();
```

| Method                  | Returns  | Description                                        |
|------------------------|----------|----------------------------------------------------|
| `run()`                | `Vec<DreamTile>` | Execute the dream and return tiles          |
| `duration_ms()`        | `u64`    | Wall-clock time the dream took                     |
| `compression_ratio()`  | `f64`    | `experiences / tiles` (∞ if no tiles produced)    |
| `energy_efficiency()`  | `f64`    | Tiles per energy unit                              |

---

### `DreamType`

```rust
pub enum DreamType {
    Consolidation,  // Merge similar experiences
    Replay,         // Rehearse important moments
    Generation,     // Blend dissimilar into novel
    Nightmare,      // Process failures intensely
    Lucid,          // High-creativity generation
    Prophetic,      // Consolidation + generation hybrid
}
```

---

### Engines

#### `ConsolidationEngine`

```rust
let engine = ConsolidationEngine::new(similarity_threshold); // e.g. 0.3
let tiles = engine.consolidate(&experiences);
```

Greedy clustering: groups experiences whose pairwise similarity exceeds the threshold, then merges each cluster into a single tile. Pure consolidation produces zero novelty and perfect confidence.

#### `ReplayEngine`

```rust
let engine = ReplayEngine::new(replay_count);
let tiles = engine.replay(&experiences, n);
```

Sorts experiences by `importance × |emotion|`, takes the top `n`, and produces replayed tiles with slight novelty (0.1) and high confidence (0.95).

#### `GenerationEngine`

```rust
let engine = GenerationEngine::new(creativity); // 0.0–1.0
let tiles = engine.generate(&experiences);
```

Randomly pairs experiences, skips pairs that are too similar (gated by creativity), and blends their content via word-level splicing. Novelty = dissimilarity; confidence = inverse of novelty.

#### `NightmareEngine`

```rust
let engine = NightmareEngine::new(intensity);
let tiles = engine.process(&experiences);
```

Filters for negative-valence experiences and produces amplified lesson tiles. Higher intensity = stronger emotional processing. Always yields moderate novelty (0.3) and high confidence (0.9).

---

### `DreamScheduler`

```rust
let scheduler = DreamScheduler::new(energy_budget, min_idle_time_ms);
```

| Method                        | Returns     | Description                                    |
|------------------------------|-------------|------------------------------------------------|
| `should_dream(idle_ms, unprocessed)` | `bool` | Is it time to dream?                    |
| `choose_dream_type(experiences)`    | `DreamType` | Pick the best engine based on emotional distribution, tag variety, similarity, importance, and energy budget |

**Decision logic:**
1. >60% negative emotions → **Nightmare**
2. High average similarity → **Consolidation**
3. Low tag variety → **Generation** (agent needs novelty)
4. Very high importance → **Replay**
5. Surplus energy budget → **Lucid**
6. Default → **Prophetic** (hybrid)

---

### `DreamJournal`

Tracks dream history across cycles.

```rust
let mut journal = DreamJournal::new("agent-7");
journal.record(cycle);
```

| Method                     | Returns            | Description                            |
|---------------------------|--------------------|----------------------------------------|
| `total_dreams()`          | `usize`            | Number of recorded dream cycles        |
| `dream_types_distribution()` | `HashMap<String, usize>` | Count per dream type             |
| `average_compression()`   | `f64`              | Mean compression ratio across cycles   |
| `most_productive_type()`  | `DreamType`        | Engine that produced the most tiles    |
| `energy_spent()`          | `f64`              | Total energy consumed                  |
| `tiles_produced()`        | `usize`            | Total tiles across all cycles          |

---

## How It Works

### Architecture Overview

```
┌─────────────┐
│  Experiences │  (agent's waking memories)
└──────┬──────┘
       │
       ▼
┌──────────────────┐    ┌─────────────────┐
│  DreamScheduler   │───▶│  should_dream?   │
│  choose_dream_type│    └─────────────────┘
└──────┬───────────┘
       │
       ▼
┌──────────────────┐
│   DreamCycle.run  │
│  (engine dispatch)│
└──────┬───────────┘
       │
       ├─▶ ConsolidationEngine  (greedy similarity clustering)
       ├─▶ ReplayEngine          (importance-ranked rehearsal)
       ├─▶ GenerationEngine      (dissimilar pair blending)
       ├─▶ NightmareEngine       (negative-emotion amplification)
       ├─▶ GenerationEngine(hi)  (lucid mode)
       └─▶ Consolidation+Gen     (prophetic mode)
       │
       ▼
┌──────────────┐     ┌──────────────┐
│  DreamTiles   │────▶│ DreamJournal │
└──────────────┘     └──────────────┘
```

### Data Flow

1. **Collect** experiences during the agent's active phase (exploration, interaction, failures).
2. **Schedule**: `DreamScheduler.should_dream()` checks idle time and unprocessed count.
3. **Choose**: `choose_dream_type()` analyzes emotional distribution, tag variety, similarity, and importance.
4. **Execute**: `DreamCycle.run()` dispatches to the appropriate engine.
5. **Record**: Each `DreamCycle` produces `DreamTile`s and is logged in the `DreamJournal`.
6. **Utilize**: Tiles feed back into the agent's decision-making with their novelty and confidence scores.

### Energy Model

Dreaming costs energy proportional to work done:

```
energy_used = (num_experiences × 0.1) + (num_tiles × 0.05)
```

The scheduler respects an `energy_budget`, ensuring the agent doesn't overspend on dreaming.

---

## The Math

### Experience Similarity (Jaccard-Weighted)

```
similarity(A, B) = 0.5 × J_tags(A, B) + 0.5 × J_words(A, B)
```

Where `J` is the **Jaccard index**:

```
J(X, Y) = 2|X ∩ Y| / (|X| + |Y|)
```

Tag similarity operates on categorical tags; content similarity operates on word-level token sets. The 50/50 weighting ensures both structural (tags) and semantic (content) similarity contribute equally.

### Consolidation Clustering

Greedy single-linkage clustering:

1. For each unassigned experience `i`, form a new cluster.
2. For each subsequent unassigned `j > i`, if `similarity(i, j) ≥ threshold`, assign `j` to `i`'s cluster.
3. Merge each cluster into one tile.

Time complexity: **O(n²)** per consolidation pass.

### Replay Scoring

Experiences are ranked by:

```
score = importance × |emotion|
```

High-importance, emotionally charged memories are replayed first.

### Generation Blending

For a pair of experiences `(A, B)` with dissimilarity `d = 1 − similarity(A, B)`:

- **Admission gate**: skip if `d < (1 − creativity) × 0.3`
- **Blend**: take `⌈|words_A| × f⌉` words from A and `⌈|words_B| × (1−f)⌉` words from B, where `f ~ Uniform(0.3, 0.7)`
- **Novelty** = `d`
- **Confidence** = `1 − 0.5d`

### Nightmare Amplification

For each negative-emotion experience `e`:

```
emotional_weight = |e.emotion| × intensity
```

Nightmare tiles always have: novelty = 0.3, confidence = 0.9.

### Compression Ratio

```
ratio = num_experiences / num_tiles
```

- `ratio > 1`: successful compression (more experiences than tiles)
- `ratio = 1`: no compression (1:1 mapping)
- `ratio = ∞`: experiences produced no tiles (ineffective dream)

---

## Testing

54 tests covering all engines, the scheduler, journal, and edge cases:

```bash
cargo test
```

Test categories:
- **Consolidation** (5 tests): clustering, empty input, identical experiences, dissimilar separation
- **Replay** (5 tests): source preservation, importance sorting, novelty, empty input, overflow
- **Generation** (6 tests): novelty creation, empty/single input, two-source verification, dissimilarity correlation
- **Nightmare** (5 tests): negative filtering, empty-when-positive, intensity scaling, confidence
- **DreamCycle** (6 tests): run dispatch, compression ratio, energy calculation, duration tracking
- **DreamJournal** (8 tests): recording, distribution, compression averages, most productive type
- **DreamScheduler** (8 tests): should_dream gating, dream type selection across emotional distributions
- **DreamTile** (4 tests): construction, serialization
- **Experience** (7 tests): similarity scoring, edge cases

---

## License

MIT © SuperInstance
