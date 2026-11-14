fn skyblock_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::horizon_skyblock as m;
    use horizon_client::extras::Corner;
    use horizon_skyblock_core::state::Line;
    if !game.native_window() || !game.native_on(m::ID) || game.native_player().is_none() {
        return;
    }
    m::initialize(game);
    let (mut lines, settings, latency) = {
        let r = m::runtime();
        if !r.visible {
            return;
        }
        (
            r.engine.lines(&r.settings),
            r.settings.clone(),
            r.engine.latency,
        )
    };
    if lines.is_empty() {
        return;
    }
    if settings.on("performance") {
        let (fps, min, low) = game.native_frame_rate();
        let ping = latency
            .map(|n| format!("{n}ms"))
            .unwrap_or_else(|| "--".into());
        lines.push(Line {
            text: format!("{fps} FPS | 1% {low} | min {min} | Ping {ping}"),
            color: 0x85CFFF,
        });
    }
    if settings.on("keys") {
        let keys = [
            ("key.forward", "W"),
            ("key.left", "A"),
            ("key.back", "S"),
            ("key.right", "D"),
            ("key.jump", "Space"),
        ];
        let held = keys
            .iter()
            .filter(|(k, _)| game.native_mapping(k).is_some_and(|p| p.0))
            .map(|p| p.1)
            .collect::<Vec<_>>()
            .join(" ");
        let (l, r) = game.native_clicks();
        lines.push(Line {
            text: format!("Keys [{held}] | CPS {l}/{r}"),
            color: 0xA3A3BC,
        });
    }
    let corner = [
        Corner::TopLeft,
        Corner::TopRight,
        Corner::BottomLeft,
        Corner::BottomRight,
    ][settings.corner];
    if !game.native_corner_shown(corner) {
        return;
    }
    let max = game
        .native_client_state()
        .map(|s| ((s.gui_size[1] - 32.0) / 11.0).max(1.0) as usize)
        .unwrap_or(25);
    lines.truncate(max.min(34));
    let width = lines
        .iter()
        .map(|l| game.native_text_width(&l.text))
        .fold(0.0f32, f32::max)
        + 12.0;
    let height = lines.len() as f32 * 11.0 + 9.0;
    let (x, y) = corners.place(corner, width, height);
    game.native_fill(batch, [x, y, width, height], 0xBA10111C);
    game.native_fill(batch, [x, y, 2.0, height], 0xFFAA88FF);
    for (i, l) in lines.iter().enumerate() {
        game.native_text(
            batch,
            &l.text,
            [x + 6.0, y + 5.0 + i as f32 * 11.0],
            l.color,
            true,
        );
    }
}
