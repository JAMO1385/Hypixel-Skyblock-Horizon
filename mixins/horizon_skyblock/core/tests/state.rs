use horizon_skyblock_core::{Engine, Settings};

fn sky() -> Engine {
    let mut e = Engine::default();
    e.set_server("mc.hypixel.net:25565");
    e.sidebar("SKYBLOCK", &["⏣ Village".into(), "Purse: 1,234,567".into()]);
    e.tick(100);
    e
}

#[test]
fn actual_host_and_sidebar_gate_skyblock_and_reset_stale_stats() {
    let mut e = Engine::default();
    e.set_server("hypixel.net.evil.example");
    e.sidebar("SKYBLOCK", &[]);
    assert!(!e.skyblock);
    e.set_server("mc.hypixel.net");
    e.sidebar("SKYBLOCK", &[]);
    assert!(e.skyblock);
    e.action("1,000/1,200❤  350❈ Defense  125/250✎ Mana");
    assert_eq!(e.health, Some((1000.0, 1200.0)));
    assert_eq!(e.mana, Some((125.0, 250.0)));
    e.sidebar("HYPIXEL", &[]);
    assert!(!e.skyblock);
    assert!(e.health.is_none());
    assert!(e.metrics.is_empty());
}

#[test]
fn extracts_visible_economy_area_pet_and_mining_stats() {
    let mut e = sky();
    e.tab("Area: Crystal Hollows\nPet: [Lvl 100] Bal\nMithril Powder: 23,456\nGemstone Powder: 3.2M\nSpeed: 400\nProfile: Apple");
    assert_eq!(e.area, "Crystal Hollows");
    assert_eq!(e.metrics.get("Purse").map(String::as_str), Some("1.23M"));
    assert_eq!(
        e.metrics.get("Gemstone Powder").map(String::as_str),
        Some("3.20M")
    );
    assert_eq!(
        e.metrics.get("Pet").map(String::as_str),
        Some("[Lvl 100] Bal")
    );
    assert!(
        e.lines(&Settings::default())
            .iter()
            .any(|l| l.text.contains("Crystal Hollows"))
    );
}

#[test]
fn player_chat_cannot_forge_drops_or_alerts() {
    let mut e = sky();
    e.chat("RARE DROP! Overflux Capacitor", false);
    assert_eq!(e.session.rare_drops, 0);
    assert!(e.alerts.is_empty());
    e.chat("RARE DROP! Overflux Capacitor", true);
    assert_eq!(e.session.rare_drops, 1);
    assert!(!e.alerts.is_empty());
}

#[test]
fn tracks_slayer_dungeon_and_fishing_from_system_text() {
    let mut e = sky();
    e.chat("SLAYER QUEST STARTED!", true);
    e.chat("Slay the boss!", true);
    assert_eq!(e.slayer, "Boss spawned");
    e.chat("SLAYER QUEST COMPLETE!", true);
    assert_eq!(e.session.slayer, 1);
    e.chat("You caught a Sea Walker.", true);
    assert_eq!(e.session.sea_creatures, 1);
    e.sidebar(
        "SKYBLOCK",
        &[
            "The Catacombs (F7)".into(),
            "Time Elapsed: 2m 35s".into(),
            "Dungeon Cleared: 47%".into(),
        ],
    );
    assert_eq!(
        e.metrics.get("Time Elapsed").map(String::as_str),
        Some("2m 35s")
    );
    e.chat("☠ Player became a ghost.", true);
    assert_eq!(e.session.party_deaths, 1);
    e.action("+25 Mining (1,250/10,000)");
    assert_eq!(e.session.skill_xp, 25.0);
}

#[test]
fn low_resource_alerts_have_hysteresis_and_expire() {
    let mut e = sky();
    e.action("100/1,000❤ 10/100✎ Mana");
    e.tick(101);
    let first = e.alerts.len();
    assert!(first >= 1);
    e.tick(102);
    assert_eq!(e.alerts.len(), first);
    e.action("900/1,000❤ 90/100✎ Mana");
    e.tick(103);
    e.action("100/1,000❤ 10/100✎ Mana");
    e.tick(104);
    assert_eq!(
        e.alerts.len(),
        first,
        "active duplicate warnings are suppressed"
    );
    e.tick(112);
    assert!(e.alerts.is_empty());
    e.action("900/1,000❤ 90/100✎ Mana");
    e.tick(113);
    e.action("100/1,000❤ 10/100✎ Mana");
    e.tick(114);
    assert!(!e.alerts.is_empty());
    e.tick(150);
    assert!(e.alerts.is_empty());
}

#[test]
fn timers_finish_once_and_settings_survive_roundtrip() {
    let mut e = sky();
    let r = e.command("timer add 2m30s Forge");
    assert!(r[0].contains("Forge"));
    assert_eq!(e.timers.len(), 1);
    let saved = e.save_timers();
    let mut other = sky();
    other.load_timers(&saved);
    other.tick(249);
    assert!(other.notices().is_empty());
    other.tick(250);
    assert!(other.notices().iter().any(|s| s.contains("Forge")));
    assert!(other.timers.is_empty());
    other.tick(251);
    assert!(other.notices().is_empty());
    let mut s = Settings::default();
    s.toggle("mining");
    let saved = s.store();
    let s2 = Settings::read(|k| saved.iter().find(|p| p.0 == k).map(|p| p.1.clone()));
    assert_eq!(s, s2);
}

#[test]
fn waypoint_commands_are_scoped_and_serialized() {
    let mut e = sky();
    e.position = Some(([0.0, 64.0, 0.0], "minecraft:overworld".into()));
    assert!(e.command("wp add Portal 3 64 4")[0].contains("saved"));
    assert!(e.waypoint_lines().iter().any(|s| s.contains("5m")));
    let save = e.save_waypoints();
    let mut e2 = sky();
    e2.load_waypoints(&save);
    assert_eq!(e2.waypoints.len(), 1);
    e.area = "The End".into();
    assert!(e.waypoint_lines().is_empty());
}

#[test]
fn invalid_commands_never_create_timers_or_nan_waypoints() {
    let mut e = sky();
    for cmd in [
        "timer add 0 a",
        "timer add NaN x",
        "timer add 99999999999999999999h x",
        "wp add a NaN 0 0",
        "wp add a 999999999 0 0",
    ] {
        let r = e.command(cmd);
        assert!(r[0].starts_with("Error:"), "{cmd}: {r:?}");
    }
    assert!(e.timers.is_empty());
    assert!(e.waypoints.is_empty());
    assert!(e.command("calc 100 125 10 2")[0].contains("225"));
}

#[test]
fn settings_hide_panels_and_reset_does_not_delete_waypoints() {
    let mut e = sky();
    e.position = Some(([0.0, 64.0, 0.0], "minecraft:overworld".into()));
    e.command("wp add Home");
    e.action("500/1000❤");
    let mut s = Settings::default();
    s.toggle("stats");
    assert!(!e.lines(&s).iter().any(|l| l.text.starts_with("HP")));
    e.disconnect();
    assert!(!e.hypixel);
    assert!(!e.skyblock);
    assert_eq!(e.waypoints.len(), 1);
    assert!(e.health.is_none());
}

#[test]
fn malformed_unicode_and_long_text_are_bounded() {
    let mut e = sky();
    for s in [
        "☠✎❤§",
        "∞/0❤",
        "99999999999999999999999999999999999999/1❤",
        &"x".repeat(100000),
    ] {
        e.action(s);
        e.chat(s, true);
        e.tab(s);
    }
    assert!(e.lines(&Settings::default()).len() <= 32);
    assert!(e.metrics.len() <= 48);
    assert!(e.alerts.len() <= 8);
}
