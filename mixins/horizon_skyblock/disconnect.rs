fn skyblock_disconnect(game: &crate::game::Game<'_>) {
    use crate::game::horizon_skyblock as m;
    if !game.native_window() {
        return;
    }
    let mut r = m::runtime();
    r.engine.disconnect();
    r.board = m::Board::default();
    r.tabs.clear();
    r.header.clear();
    r.held.clear();
    r.own = 0;
}
