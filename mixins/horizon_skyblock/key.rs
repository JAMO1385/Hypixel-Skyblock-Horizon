fn skyblock_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::horizon_skyblock as m;
    if !game.native_window() || !game.native_on(m::ID) {
        return;
    }
    m::initialize(game);
    {
        let mut r = m::runtime();
        if !pressed {
            r.held.remove(key);
            return;
        }
        if !r.held.insert(key.into()) {
            return;
        }
    }
    if !game.native_playing() {
        return;
    }
    for (name, _, default) in m::KEYS {
        if key != game.native_mod_key(m::ID, name, default) {
            continue;
        }
        if name == "hud" {
            let mut r = m::runtime();
            r.visible = !r.visible;
            return;
        }
        if name == "waypoint" {
            let mut ran = false;
            skyblock_key_command(game, "hs wp add Mark", &mut ran);
            return;
        }
        let mut ran = false;
        skyblock_key_command(game, &format!("hs {name}"), &mut ran);
        return;
    }
    fn skyblock_key_command(game: &mut crate::game::Game<'_>, text: &str, ran: &mut bool) {
        game.native_command(text, ran);
    }
}
