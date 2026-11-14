fn skyblock_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use crate::game::horizon_skyblock as m;
    let Some(args) = command
        .strip_prefix("hs")
        .filter(|s| s.is_empty() || s.starts_with(char::is_whitespace))
    else {
        return;
    };
    if *ran || !game.native_window() || !game.native_on(m::ID) {
        return;
    }
    *ran = true;
    m::initialize(game);
    m::sync(game);
    let args = args.trim();
    if let Some(command) = m::shortcut(args) {
        let okay = {
            let mut r = m::runtime();
            let now = m::now();
            if !r.engine.hypixel || now.saturating_sub(r.shortcut_at) < 2 {
                false
            } else {
                r.shortcut_at = now;
                true
            }
        };
        if okay {
            if let Err(e) = game.native_send_chat(command) {
                game.native_chat(&format!("[Horizon SB] {e}"));
            }
        } else {
            game.native_chat("[Horizon SB] Join Hypixel; shortcuts have a 2-second cooldown");
        }
        return;
    }
    let lines = {
        let mut r = m::runtime();
        if let Some(panel) = args.strip_prefix("toggle ") {
            if r.settings.toggle(panel.trim()) {
                vec![format!("Panel {panel}: {}", r.settings.on(panel.trim()))]
            } else {
                vec!["Error: stats economy mining farming slayer dungeon session timers waypoints performance keys alerts".into()]
            }
        } else {
            r.engine.command(args)
        }
    };
    for line in lines {
        game.native_chat(&format!("[Horizon SB] {line}"));
    }
    m::persist(game);
}
