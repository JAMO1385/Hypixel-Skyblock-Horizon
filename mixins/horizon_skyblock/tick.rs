fn skyblock_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::horizon_skyblock as m;
    if !game.native_window() || !game.native_on(m::ID) {
        return;
    }
    m::initialize(game);
    m::sync(game);
    let (notices, save) = {
        let mut r = m::runtime();
        let now = m::now();
        if now == r.last_tick {
            return;
        }
        r.last_tick = now;
        let n = r.engine.timers.len();
        r.engine.tick(now);
        let save = n != r.engine.timers.len();
        (r.engine.notices(), save)
    };
    let show = m::runtime().settings.on("alerts");
    if show {
        for line in notices {
            game.native_chat(&format!("[Horizon SB] {line}"));
        }
    }
    if save {
        m::persist(game);
    }
}
