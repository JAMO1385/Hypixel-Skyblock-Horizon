use horizon_skyblock_core::{scoreboard::Board, text};

#[test]
fn strips_minecraft_formatting_without_damaging_unicode() {
    assert_eq!(
        text::clean("§a1,245/§c1,500❤ §b100/200✎ Mana"),
        "1,245/1,500❤ 100/200✎ Mana"
    );
    assert_eq!(text::clean("§x§f§f§0§0§a§aArea\u{0000} §"), "Area ");
}

#[test]
fn reads_compact_numbers_and_rejects_bad_or_nonfinite_values() {
    for (input, want) in [
        ("1,234.5", 1234.5),
        ("2.5k", 2500.0),
        ("3M", 3_000_000.0),
        ("1.25b", 1_250_000_000.0),
    ] {
        assert_eq!(text::number(input), Some(want));
    }
    for input in ["NaN", "inf", "-2", "1,2,3", "2kk", "1e99", ""] {
        assert_eq!(text::number(input), None, "{input}");
    }
}

#[test]
fn parses_bounded_durations() {
    for (input, want) in [
        ("1h30m", 5400),
        ("2m 5s", 125),
        ("01:30", 90),
        ("1:02:03", 3723),
        ("60", 60),
    ] {
        assert_eq!(text::duration(input), Some(want), "{input}");
    }
    for input in ["0", "-1", "1:90", "999999999999999999d", "1mgarbage", "NaN"] {
        assert_eq!(text::duration(input), None, "{input}");
    }
}

#[test]
fn reconstructs_sidebar_teams_and_removes_scores() {
    let mut b = Board::default();
    b.objective("sb", 0, "§eSKYBLOCK");
    b.display(1, "sb");
    b.score("§1", "sb", 10, None);
    b.team("line1", 0, Some(("§6Purse: 1,", "234")), &["§1".into()]);
    assert_eq!(
        b.snapshot(),
        ("SKYBLOCK".into(), vec!["Purse: 1,234".to_string()])
    );
    b.team("line1", 2, Some(("Purse: 2,", "500")), &[]);
    assert_eq!(b.snapshot().1, vec!["Purse: 2,500"]);
    b.reset("§1", Some("sb"));
    assert!(b.snapshot().1.is_empty());
}

#[test]
fn hidden_objectives_do_not_activate_skyblock() {
    let mut b = Board::default();
    b.objective("hidden", 0, "SKYBLOCK");
    assert!(b.snapshot().0.is_empty());
    b.display(1, "hidden");
    b.objective("hidden", 1, "");
    assert!(b.snapshot().0.is_empty());
}
