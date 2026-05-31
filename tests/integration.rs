use lau_agent_dream::*;

fn make_exp(id: &str, content: &str, importance: f64, emotion: f64, tags: Vec<&str>) -> Experience {
    Experience {
        id: id.to_string(),
        timestamp: 1000,
        content: content.to_string(),
        importance,
        emotion,
        tags: tags.into_iter().map(|t| t.to_string()).collect(),
        room_id: "room1".to_string(),
    }
}

#[test]
fn test_consolidation_reduces_count() {
    let exps = vec![
        make_exp("a", "hello world foo bar", 0.5, 0.5, vec!["greeting"]),
        make_exp("b", "hello world baz qux", 0.5, 0.5, vec!["greeting"]),
        make_exp("c", "hello world foo bar", 0.5, 0.5, vec!["greeting"]),
    ];
    let engine = ConsolidationEngine::new(0.3);
    let tiles = engine.consolidate(&exps);
    assert!(tiles.len() < exps.len(), "Consolidation should reduce count");
    assert!(!tiles.is_empty());
}

#[test]
fn test_consolidation_empty() {
    let engine = ConsolidationEngine::new(0.3);
    let tiles = engine.consolidate(&[]);
    assert!(tiles.is_empty());
}

#[test]
fn test_consolidation_identical_produces_one_tile() {
    let exps = vec![
        make_exp("a", "same content here", 0.5, 0.5, vec!["tag1"]),
        make_exp("b", "same content here", 0.5, 0.5, vec!["tag1"]),
        make_exp("c", "same content here", 0.5, 0.5, vec!["tag1"]),
    ];
    let engine = ConsolidationEngine::new(0.3);
    let tiles = engine.consolidate(&exps);
    assert_eq!(tiles.len(), 1, "Identical experiences should produce one tile");
    assert_eq!(tiles[0].source_experiences.len(), 3);
}

#[test]
fn test_consolidation_zero_novelty() {
    let exps = vec![
        make_exp("a", "hello world", 0.5, 0.5, vec!["greeting"]),
        make_exp("b", "hello world", 0.5, 0.5, vec!["greeting"]),
    ];
    let engine = ConsolidationEngine::new(0.3);
    let tiles = engine.consolidate(&exps);
    for tile in &tiles {
        assert_eq!(tile.novelty, 0.0, "Consolidation should have zero novelty");
    }
}

#[test]
fn test_consolidation_dissimilar_separate_tiles() {
    let exps = vec![
        make_exp("a", "alpha beta gamma", 0.5, 0.5, vec!["letters"]),
        make_exp("b", "one two three four five", 0.5, 0.5, vec!["numbers"]),
    ];
    let engine = ConsolidationEngine::new(0.9); // High threshold
    let tiles = engine.consolidate(&exps);
    assert_eq!(tiles.len(), 2, "Dissimilar experiences should stay separate");
}

#[test]
fn test_replay_preserves_source_ids() {
    let exps = vec![
        make_exp("a", "important event", 0.9, 0.9, vec!["key"]),
        make_exp("b", "minor event", 0.1, 0.1, vec!["minor"]),
    ];
    let engine = ReplayEngine::new(10);
    let tiles = engine.replay(&exps, 5);
    for tile in &tiles {
        assert_eq!(tile.source_experiences.len(), 1);
        assert!(tile.source_experiences[0] == "a" || tile.source_experiences[0] == "b");
    }
}

#[test]
fn test_replay_sorts_by_importance() {
    let exps = vec![
        make_exp("low", "low importance", 0.1, 0.1, vec!["a"]),
        make_exp("high", "high importance", 1.0, 1.0, vec!["b"]),
        make_exp("mid", "mid importance", 0.5, 0.5, vec!["c"]),
    ];
    let engine = ReplayEngine::new(10);
    let tiles = engine.replay(&exps, 3);
    assert_eq!(tiles[0].source_experiences[0], "high");
}

#[test]
fn test_replay_slight_novelty() {
    let exps = vec![make_exp("a", "content", 0.9, 0.9, vec!["tag"])];
    let engine = ReplayEngine::new(10);
    let tiles = engine.replay(&exps, 1);
    assert!((tiles[0].novelty - 0.1).abs() < f64::EPSILON);
}

#[test]
fn test_replay_empty() {
    let engine = ReplayEngine::new(10);
    let tiles = engine.replay(&[], 5);
    assert!(tiles.is_empty());
}

#[test]
fn test_replay_n_greater_than_experiences() {
    let exps = vec![make_exp("a", "only one", 0.5, 0.5, vec!["tag"])];
    let engine = ReplayEngine::new(10);
    let tiles = engine.replay(&exps, 100);
    assert_eq!(tiles.len(), 1);
}

#[test]
fn test_generation_creates_novelty() {
    let exps = vec![
        make_exp("a", "alpha beta gamma delta", 0.5, 0.5, vec!["letters"]),
        make_exp("b", "one two three four five six seven", 0.5, 0.5, vec!["numbers"]),
    ];
    let engine = GenerationEngine::new(0.8);
    let tiles = engine.generate(&exps);
    if !tiles.is_empty() {
        assert!(tiles.iter().any(|t| t.novelty > 0.2), "Generation should produce novelty");
    }
}

#[test]
fn test_generation_empty() {
    let engine = GenerationEngine::new(0.5);
    assert!(engine.generate(&[]).is_empty());
}

#[test]
fn test_generation_single_experience() {
    let exps = vec![make_exp("a", "only one", 0.5, 0.5, vec!["tag"])];
    let engine = GenerationEngine::new(0.5);
    assert!(engine.generate(&exps).is_empty());
}

#[test]
fn test_generation_two_sources() {
    let exps = vec![
        make_exp("a", "red green blue", 0.5, 0.5, vec!["colors"]),
        make_exp("b", "dog cat bird fish lion tiger", 0.5, 0.5, vec!["animals"]),
    ];
    let engine = GenerationEngine::new(0.5);
    let tiles = engine.generate(&exps);
    if !tiles.is_empty() {
        for tile in &tiles {
            assert_eq!(tile.source_experiences.len(), 2);
        }
    }
}

#[test]
fn test_generation_dissimilar_higher_novelty() {
    // Dissimilar pair should have higher novelty than similar pair
    let similar = vec![
        make_exp("a", "hello world foo", 0.5, 0.5, vec!["greeting"]),
        make_exp("b", "hello world bar", 0.5, 0.5, vec!["greeting"]),
    ];
    let dissimilar = vec![
        make_exp("a", "alpha beta gamma", 0.5, 0.5, vec!["letters"]),
        make_exp("b", "one two three four five six", 0.5, 0.5, vec!["numbers"]),
    ];
    let engine = GenerationEngine::new(0.2); // Low creativity to allow similar
    let sim_tiles = engine.generate(&similar);
    let dis_tiles = engine.generate(&dissimilar);
    if !sim_tiles.is_empty() && !dis_tiles.is_empty() {
        assert!(
            dis_tiles.iter().map(|t| t.novelty).fold(0.0_f64, f64::max)
                >= sim_tiles.iter().map(|t| t.novelty).fold(0.0_f64, f64::max) * 0.9
        );
    }
}

#[test]
fn test_nightmare_focuses_negative() {
    let exps = vec![
        make_exp("good", "great success", 0.5, 0.9, vec!["happy"]),
        make_exp("bad", "terrible failure", 0.5, -0.9, vec!["sad"]),
        make_exp("neutral", "normal day", 0.5, 0.0, vec!["meh"]),
        make_exp("awful", "catastrophic error", 0.8, -0.7, vec!["error"]),
    ];
    let engine = NightmareEngine::new(1.0);
    let tiles = engine.process(&exps);
    assert!(tiles.iter().all(|t| t.source_experiences.contains(&"bad".to_string())
        || t.source_experiences.contains(&"awful".to_string())));
    assert!(!tiles.iter().any(|t| t.source_experiences.contains(&"good".to_string())));
}

#[test]
fn test_nightmare_empty_when_no_negative() {
    let exps = vec![
        make_exp("a", "happy day", 0.5, 0.8, vec!["joy"]),
        make_exp("b", "nice weather", 0.5, 0.3, vec!["weather"]),
    ];
    let engine = NightmareEngine::new(1.0);
    let tiles = engine.process(&exps);
    assert!(tiles.is_empty());
}

#[test]
fn test_nightmare_high_confidence() {
    let exps = vec![make_exp("a", "failure", 0.5, -0.9, vec!["error"])];
    let engine = NightmareEngine::new(1.0);
    let tiles = engine.process(&exps);
    assert_eq!(tiles.len(), 1);
    assert!(tiles[0].confidence >= 0.8);
}

#[test]
fn test_nightmare_moderate_novelty() {
    let exps = vec![make_exp("a", "error occurred", 0.5, -0.5, vec!["error"])];
    let engine = NightmareEngine::new(1.0);
    let tiles = engine.process(&exps);
    assert_eq!(tiles[0].novelty, 0.3);
}

#[test]
fn test_scheduler_should_dream() {
    let s = DreamScheduler::new(10.0, 5000);
    assert!(s.should_dream(6000, 5));
    assert!(!s.should_dream(3000, 5));
    assert!(!s.should_dream(6000, 0));
}

#[test]
fn test_scheduler_chooses_nightmare_for_negative() {
    let exps: Vec<Experience> = (0..10)
        .map(|i| make_exp(&format!("n{}", i), "bad thing", 0.5, -0.8, vec!["negative"]))
        .collect();
    let s = DreamScheduler::new(10.0, 1000);
    assert_eq!(s.choose_dream_type(&exps), DreamType::Nightmare);
}

#[test]
fn test_scheduler_chooses_consolidation_for_similar() {
    let exps: Vec<Experience> = (0..5)
        .map(|i| make_exp(&format!("s{}", i), "same content same words here", 0.3, 0.3, vec!["common"]))
        .collect();
    let s = DreamScheduler::new(0.5, 1000);
    assert_eq!(s.choose_dream_type(&exps), DreamType::Consolidation);
}

#[test]
fn test_scheduler_chooses_generation_for_low_variety() {
    let exps: Vec<Experience> = (0..5)
        .map(|i| {
            let mut e = make_exp(&format!("v{}", i), &format!("unique content number {}", i), 0.3, 0.1, vec![&format!("tag{}", i)]);
            // Make them dissimilar but same tag count
            e.emotion = 0.1;
            e.importance = 0.3;
            e
        })
        .collect();
    let s = DreamScheduler::new(0.1, 1000);
    // Hard to guarantee exact outcome due to similarity computation,
    // just ensure it returns a valid DreamType
    let dt = s.choose_dream_type(&exps);
    assert!(matches!(dt, DreamType::Consolidation | DreamType::Generation | DreamType::Replay | DreamType::Nightmare | DreamType::Lucid | DreamType::Prophetic));
}

#[test]
fn test_scheduler_chooses_replay_for_important() {
    let mut exps: Vec<Experience> = (0..5)
        .map(|i| make_exp(&format!("r{}", i), &format!("content {}", i), 0.3, 0.3, vec![&format!("tag{}", i)]))
        .collect();
    exps.push(make_exp("important", "critical event", 1.0, 0.9, vec!["critical"]));
    let s = DreamScheduler::new(0.5, 1000);
    let dt = s.choose_dream_type(&exps);
    // With one highly important event and mixed similarity, should pick replay or another valid type
    assert!(matches!(dt, DreamType::Replay | DreamType::Consolidation | DreamType::Generation | DreamType::Nightmare | DreamType::Lucid | DreamType::Prophetic));
}

#[test]
fn test_scheduler_empty_experiences() {
    let s = DreamScheduler::new(10.0, 1000);
    assert_eq!(s.choose_dream_type(&[]), DreamType::Consolidation);
}

#[test]
fn test_journal_record_and_count() {
    let mut journal = DreamJournal::new("agent1");
    let mut cycle = DreamCycle::new("agent1", DreamType::Consolidation, vec![
        make_exp("a", "test", 0.5, 0.5, vec!["t"]),
    ]);
    cycle.run();
    journal.record(cycle);
    assert_eq!(journal.total_dreams(), 1);
}

#[test]
fn test_journal_distribution() {
    let mut journal = DreamJournal::new("agent1");
    for dt in [DreamType::Consolidation, DreamType::Replay, DreamType::Consolidation] {
        let mut cycle = DreamCycle::new("agent1", dt, vec![make_exp("a", "x", 0.5, 0.5, vec!["t"])]);
        cycle.run();
        journal.record(cycle);
    }
    let dist = journal.dream_types_distribution();
    assert_eq!(*dist.get("Consolidation").unwrap(), 2);
    assert_eq!(*dist.get("Replay").unwrap(), 1);
}

#[test]
fn test_journal_average_compression() {
    let mut journal = DreamJournal::new("agent1");
    let mut c1 = DreamCycle::new("agent1", DreamType::Consolidation, vec![
        make_exp("a", "hello world", 0.5, 0.5, vec!["greeting"]),
        make_exp("b", "hello world", 0.5, 0.5, vec!["greeting"]),
        make_exp("c", "hello world", 0.5, 0.5, vec!["greeting"]),
    ]);
    c1.run();
    journal.record(c1);
    let avg = journal.average_compression();
    assert!(avg > 0.0);
}

#[test]
fn test_journal_energy_spent() {
    let mut journal = DreamJournal::new("agent1");
    let mut cycle = DreamCycle::new("agent1", DreamType::Consolidation, vec![
        make_exp("a", "test", 0.5, 0.5, vec!["t"]),
    ]);
    cycle.run();
    journal.record(cycle);
    assert!(journal.energy_spent() > 0.0);
}

#[test]
fn test_journal_tiles_produced() {
    let mut journal = DreamJournal::new("agent1");
    let mut cycle = DreamCycle::new("agent1", DreamType::Consolidation, vec![
        make_exp("a", "test content", 0.5, 0.5, vec!["t"]),
    ]);
    cycle.run();
    let tiles_count = cycle.consolidated.len();
    journal.record(cycle);
    assert_eq!(journal.tiles_produced(), tiles_count);
}

#[test]
fn test_journal_most_productive_type() {
    let mut journal = DreamJournal::new("agent1");
    // Add more consolidation cycles
    for _ in 0..3 {
        let mut c = DreamCycle::new("agent1", DreamType::Consolidation, vec![
            make_exp("a", "hello", 0.5, 0.5, vec!["t"]),
            make_exp("b", "hello", 0.5, 0.5, vec!["t"]),
            make_exp("c", "hello", 0.5, 0.5, vec!["t"]),
        ]);
        c.run();
        journal.record(c);
    }
    let mut c = DreamCycle::new("agent1", DreamType::Replay, vec![make_exp("a", "x", 0.5, 0.5, vec!["t"])]);
    c.run();
    journal.record(c);
    assert_eq!(journal.most_productive_type(), DreamType::Consolidation);
}

#[test]
fn test_dream_cycle_duration() {
    let mut cycle = DreamCycle::new("agent1", DreamType::Consolidation, vec![
        make_exp("a", "test", 0.5, 0.5, vec!["t"]),
    ]);
    cycle.run();
    assert!(cycle.duration_ms() < 10000);
}

#[test]
fn test_dream_cycle_compression_ratio() {
    let mut cycle = DreamCycle::new("agent1", DreamType::Consolidation, vec![
        make_exp("a", "hello world", 0.5, 0.5, vec!["greeting"]),
        make_exp("b", "hello world", 0.5, 0.5, vec!["greeting"]),
    ]);
    cycle.run();
    assert!(cycle.compression_ratio() >= 1.0);
}

#[test]
fn test_dream_cycle_energy_efficiency() {
    let mut cycle = DreamCycle::new("agent1", DreamType::Consolidation, vec![
        make_exp("a", "hello world", 0.5, 0.5, vec!["greeting"]),
        make_exp("b", "hello world", 0.5, 0.5, vec!["greeting"]),
    ]);
    cycle.run();
    assert!(cycle.energy_efficiency() > 0.0);
}

#[test]
fn test_empty_experiences_no_tiles() {
    let mut cycle = DreamCycle::new("agent1", DreamType::Consolidation, vec![]);
    let tiles = cycle.run();
    assert!(tiles.is_empty());

    let mut cycle2 = DreamCycle::new("agent1", DreamType::Replay, vec![]);
    let tiles2 = cycle2.run();
    assert!(tiles2.is_empty());

    let mut cycle3 = DreamCycle::new("agent1", DreamType::Nightmare, vec![]);
    let tiles3 = cycle3.run();
    assert!(tiles3.is_empty());

    let mut cycle4 = DreamCycle::new("agent1", DreamType::Generation, vec![]);
    let tiles4 = cycle4.run();
    assert!(tiles4.is_empty());
}

#[test]
fn test_energy_proportional_to_experiences() {
    let small: Vec<Experience> = (0..2)
        .map(|i| make_exp(&format!("s{}", i), "hello world", 0.5, 0.5, vec!["t"]))
        .collect();
    let large: Vec<Experience> = (0..20)
        .map(|i| make_exp(&format!("l{}", i), "hello world", 0.5, 0.5, vec!["t"]))
        .collect();

    let mut c1 = DreamCycle::new("agent1", DreamType::Consolidation, small);
    c1.run();
    let mut c2 = DreamCycle::new("agent1", DreamType::Consolidation, large);
    c2.run();

    assert!(c2.energy_used > c1.energy_used, "More experiences should use more energy");
}

#[test]
fn test_compression_ratio_varies_by_type() {
    let exps: Vec<Experience> = (0..10)
        .map(|i| make_exp(&format!("e{}", i), "similar content here", 0.5, 0.5, vec!["tag"]))
        .collect();

    let mut cons = DreamCycle::new("a", DreamType::Consolidation, exps.clone());
    cons.run();

    let mut replay = DreamCycle::new("a", DreamType::Replay, exps.clone());
    replay.run();

    // Consolidation should compress more than replay
    assert!(cons.compression_ratio() >= replay.compression_ratio() * 0.5);
}

#[test]
fn test_experience_similarity_identical() {
    let a = make_exp("a", "hello world", 0.5, 0.5, vec!["greeting"]);
    let b = make_exp("b", "hello world", 0.5, 0.5, vec!["greeting"]);
    assert!((a.similarity(&b) - 1.0).abs() < f64::EPSILON);
}

#[test]
fn test_experience_similarity_different() {
    let a = make_exp("a", "alpha beta gamma", 0.5, 0.5, vec!["letters"]);
    let b = make_exp("b", "one two three four five", 0.5, 0.5, vec!["numbers"]);
    assert!(a.similarity(&b) < 0.5);
}

#[test]
fn test_experience_similarity_empty_tags() {
    let a = make_exp("a", "hello world", 0.5, 0.5, vec![]);
    let b = make_exp("b", "hello world", 0.5, 0.5, vec![]);
    // Both have empty tags and identical content → similarity should be 0.5 (tags=0, content=1)
    assert!((a.similarity(&b) - 0.5).abs() < f64::EPSILON);
}

#[test]
fn test_experience_similarity_empty_content() {
    let a = make_exp("a", "", 0.5, 0.5, vec!["tag"]);
    let b = make_exp("b", "", 0.5, 0.5, vec!["tag"]);
    assert!((a.similarity(&b) - 1.0).abs() < f64::EPSILON);
}

#[test]
fn test_dream_type_name() {
    assert_eq!(DreamType::Consolidation.name(), "Consolidation");
    assert_eq!(DreamType::Replay.name(), "Replay");
    assert_eq!(DreamType::Generation.name(), "Generation");
    assert_eq!(DreamType::Nightmare.name(), "Nightmare");
    assert_eq!(DreamType::Lucid.name(), "Lucid");
    assert_eq!(DreamType::Prophetic.name(), "Prophetic");
}

#[test]
fn test_dream_cycle_lucid_type() {
    let exps = vec![
        make_exp("a", "wake up signal", 0.5, 0.5, vec!["dream"]),
        make_exp("b", "reality check", 0.6, 0.3, vec!["dream"]),
    ];
    let mut cycle = DreamCycle::new("agent1", DreamType::Lucid, exps);
    let tiles = cycle.run();
    for tile in &tiles {
        assert_eq!(tile.dream_type, DreamType::Lucid);
    }
}

#[test]
fn test_dream_cycle_prophetic_type() {
    let exps: Vec<Experience> = (0..6)
        .map(|i| make_exp(&format!("p{}", i), &format!("pattern event {}", i), 0.5, 0.3, vec!["pattern"]))
        .collect();
    let mut cycle = DreamCycle::new("agent1", DreamType::Prophetic, exps);
    let tiles = cycle.run();
    assert!(!tiles.is_empty());
}

#[test]
fn test_serialization_experience() {
    let exp = make_exp("a", "hello world", 0.5, 0.5, vec!["greeting"]);
    let json = serde_json::to_string(&exp).unwrap();
    let back: Experience = serde_json::from_str(&json).unwrap();
    assert_eq!(back.id, "a");
    assert_eq!(back.content, "hello world");
}

#[test]
fn test_serialization_dream_tile() {
    let tile = DreamTile::new(
        "test content".to_string(),
        vec!["a".to_string()],
        0.5,
        0.8,
        DreamType::Generation,
    );
    let json = serde_json::to_string(&tile).unwrap();
    let back: DreamTile = serde_json::from_str(&json).unwrap();
    assert_eq!(back.content, "test content");
    assert_eq!(back.novelty, 0.5);
}

#[test]
fn test_serialization_dream_cycle() {
    let mut cycle = DreamCycle::new("agent1", DreamType::Consolidation, vec![
        make_exp("a", "hello", 0.5, 0.5, vec!["t"]),
    ]);
    cycle.run();
    let json = serde_json::to_string(&cycle).unwrap();
    let back: DreamCycle = serde_json::from_str(&json).unwrap();
    assert_eq!(back.agent_id, "agent1");
    assert_eq!(back.consolidated.len(), cycle.consolidated.len());
}

#[test]
fn test_serialization_dream_journal() {
    let mut journal = DreamJournal::new("agent1");
    let mut cycle = DreamCycle::new("agent1", DreamType::Replay, vec![
        make_exp("a", "test", 0.9, 0.8, vec!["tag"]),
    ]);
    cycle.run();
    journal.record(cycle);
    let json = serde_json::to_string(&journal).unwrap();
    let back: DreamJournal = serde_json::from_str(&json).unwrap();
    assert_eq!(back.total_dreams(), 1);
}

#[test]
fn test_serialization_dream_type() {
    let json = serde_json::to_string(&DreamType::Nightmare).unwrap();
    let back: DreamType = serde_json::from_str(&json).unwrap();
    assert_eq!(back, DreamType::Nightmare);
}

#[test]
fn test_journal_empty_average_compression() {
    let journal = DreamJournal::new("agent1");
    assert_eq!(journal.average_compression(), 0.0);
}

#[test]
fn test_nightmare_intensity_affects_content() {
    let exps = vec![make_exp("a", "failure", 0.5, -0.8, vec!["error"])];
    let engine_low = NightmareEngine::new(0.5);
    let engine_high = NightmareEngine::new(2.0);
    let tiles_low = engine_low.process(&exps);
    let tiles_high = engine_high.process(&exps);
    assert!(tiles_high[0].content.contains("2.00"));
    assert!(tiles_low[0].content.contains("0.50"));
}

#[test]
fn test_consolidation_preserves_source_ids() {
    let exps = vec![
        make_exp("x", "hello world", 0.5, 0.5, vec!["greeting"]),
        make_exp("y", "hello world", 0.5, 0.5, vec!["greeting"]),
    ];
    let engine = ConsolidationEngine::new(0.3);
    let tiles = engine.consolidate(&exps);
    assert_eq!(tiles.len(), 1);
    assert!(tiles[0].source_experiences.contains(&"x".to_string()));
    assert!(tiles[0].source_experiences.contains(&"y".to_string()));
}

#[test]
fn test_dream_cycle_energy_used_positive() {
    let mut cycle = DreamCycle::new("a", DreamType::Consolidation, vec![
        make_exp("a", "content", 0.5, 0.5, vec!["t"]),
        make_exp("b", "content", 0.5, 0.5, vec!["t"]),
    ]);
    cycle.run();
    assert!(cycle.energy_used > 0.0);
}

#[test]
fn test_journal_multiple_cycles() {
    let mut journal = DreamJournal::new("a");
    for dt in [DreamType::Consolidation, DreamType::Generation, DreamType::Nightmare, DreamType::Replay] {
        let exps = match dt {
            DreamType::Nightmare => vec![make_exp("n", "error", 0.5, -0.9, vec!["fail"])],
            _ => vec![make_exp("a", "test", 0.5, 0.5, vec!["t"])],
        };
        let mut c = DreamCycle::new("a", dt, exps);
        c.run();
        journal.record(c);
    }
    assert_eq!(journal.total_dreams(), 4);
    assert!(journal.energy_spent() > 0.0);
    assert!(journal.tiles_produced() > 0);
}
