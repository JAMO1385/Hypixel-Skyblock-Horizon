fn skyblock_settings_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::horizon_skyblock as m;
    if mod_id != m::ID {
        return;
    }
    m::initialize(game);
    m::runtime().settings.click(button);
    m::persist(game);
}
