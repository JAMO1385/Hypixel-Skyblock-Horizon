use horizon_skyblock_core::Engine;

#[test]
fn player_message_wrapped_in_system_packet_is_not_a_game_event() {
    let mut e = Engine::default();
    e.set_server("mc.hypixel.net");
    e.sidebar("SKYBLOCK", &[]);
    e.chat("[MVP+] Someone: RARE DROP! Overflux Capacitor", true);
    e.chat("Party > Someone: SLAYER QUEST COMPLETE!", true);
    assert_eq!(e.session.rare_drops, 0);
    assert_eq!(e.session.slayer, 0);
}

#[test]
fn system_messages_with_colons_are_not_discarded() {
    let mut e = Engine::default();
    e.set_server("mc.hypixel.net");
    e.sidebar("SKYBLOCK", &[]);
    e.chat("You earned 1,000 coins: daily bonus", true);
    assert_eq!(e.session.coins, 1_000.0);
}
#[test]
fn partial_action_bars_do_not_leave_unrelated_values_stale() {
    let mut e = Engine::default();
    e.set_server("mc.hypixel.net");
    e.sidebar("SKYBLOCK", &[]);
    e.tick(100);
    e.action("100/100❤ 50/100✎ Mana");
    e.tick(115);
    e.action("+25 Mining (10/100)");
    e.tick(116);
    assert!(e.health.is_none());
    assert!(e.mana.is_none());
}
#[test]
fn changing_areas_clears_old_dungeon_tab_data() {
    let mut e = Engine::default();
    e.set_server("mc.hypixel.net");
    e.sidebar("SKYBLOCK", &["⏣ The Catacombs".into()]);
    e.tab("Secrets Found: 10\nArea: The Catacombs");
    e.sidebar("SKYBLOCK", &["⏣ Village".into()]);
    assert!(!e.metrics.contains_key("Secrets Found"));
    assert_eq!(e.area, "Village");
}
