fn skyblock_settings(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::horizon_skyblock as m;
    if mod_id != m::ID {
        return;
    }
    m::initialize(game);
    buttons.extend(m::runtime().settings.labels());
}
