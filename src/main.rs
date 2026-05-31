use lau_agent_dream::*;

fn main() {
    println!("=== Lau Agent Dream ===\n");

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
            timestamp: 2000,
            content: "Explored the southern corridor".into(),
            importance: 0.7,
            emotion: 0.3,
            tags: vec!["exploration".into()],
            room_id: "corridor".into(),
        },
        Experience {
            id: "e3".into(),
            timestamp: 3000,
            content: "Failed to open the sealed door".into(),
            importance: 0.9,
            emotion: -0.8,
            tags: vec!["failure".into(), "door".into()],
            room_id: "sealed".into(),
        },
        Experience {
            id: "e4".into(),
            timestamp: 4000,
            content: "Found a glowing artifact".into(),
            importance: 1.0,
            emotion: 0.9,
            tags: vec!["discovery".into(), "artifact".into()],
            room_id: "vault".into(),
        },
        Experience {
            id: "e5".into(),
            timestamp: 5000,
            content: "Encountered a locked puzzle mechanism".into(),
            importance: 0.6,
            emotion: -0.3,
            tags: vec!["puzzle".into(), "locked".into()],
            room_id: "puzzle".into(),
        },
    ];

    println!("Waking experiences: {}", experiences.len());
    for e in &experiences {
        println!("  [{}] {} (importance={:.1}, emotion={:.1})", e.id, e.content, e.importance, e.emotion);
    }
    println!();

    let mut journal = DreamJournal::new("agent-7");

    // Run all dream types
    for dream_type in [
        DreamType::Consolidation,
        DreamType::Replay,
        DreamType::Generation,
        DreamType::Nightmare,
        DreamType::Lucid,
        DreamType::Prophetic,
    ] {
        let mut cycle = DreamCycle::new("agent-7", dream_type, experiences.clone());
        let tiles = cycle.run();

        println!("--- {:?} Dream ---", dream_type);
        println!("Duration: {}ms", cycle.duration_ms());
        println!("Energy used: {:.3}", cycle.energy_used);
        println!("Compression ratio: {:.2}", cycle.compression_ratio());
        println!("Tiles produced: {}", tiles.len());
        for tile in &tiles {
            println!("  [{}] {} (novelty={:.2}, confidence={:.2})",
                &tile.id[..8], tile.content, tile.novelty, tile.confidence);
        }
        println!();

        journal.record(cycle);
    }

    println!("=== Dream Journal Summary ===");
    println!("Total dreams: {}", journal.total_dreams());
    println!("Total energy spent: {:.3}", journal.energy_spent());
    println!("Total tiles produced: {}", journal.tiles_produced());
    println!("Average compression: {:.2}", journal.average_compression());
    println!("Distribution: {:?}", journal.dream_types_distribution());
    println!("Most productive type: {:?}", journal.most_productive_type());

    // Scheduler decision
    let scheduler = DreamScheduler::new(5.0, 3000);
    println!("\nScheduler: should dream after 5s idle with 5 unprocessed? {}", scheduler.should_dream(5000, 5));
    println!("Scheduler chose: {:?}", scheduler.choose_dream_type(&experiences));
}
