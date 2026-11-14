fn skyblock_packet(
    game: &mut crate::game::Game<'_>,
    packet: &horizon_client::mod_api::ClientboundPlayPacket,
) {
    use crate::game::horizon_skyblock as m;
    use horizon_protocol::packets::play::Clientbound as C;
    if !game.native_window() || !game.native_on(m::ID) {
        return;
    }
    m::initialize(game);
    m::sync(game);
    let mut r = m::runtime();
    if !r.engine.hypixel {
        return;
    }
    let mut sidebar = false;
    let mut tab = false;
    match packet {
        C::SetObjective(p) => {
            let title = p
                .display
                .as_ref()
                .map(|p| m::tag_text(&p.0))
                .unwrap_or_default();
            r.board.objective(&p.name, p.method, &title);
            sidebar = true;
        }
        C::SetDisplayObjective(p) => {
            r.board.display(p.slot, &p.objective);
            sidebar = true;
        }
        C::SetScore(p) => {
            let text = p.display.as_ref().map(m::tag_text);
            r.board
                .score(&p.owner, &p.objective, p.score, text.as_deref());
            sidebar = true;
        }
        C::ResetScore(p) => {
            r.board.reset(&p.owner, p.objective.as_deref());
            sidebar = true;
        }
        C::SetPlayerTeam(p) => {
            let params = p
                .parameters
                .as_ref()
                .map(|p| (m::tag_text(&p.prefix), m::tag_text(&p.suffix)));
            r.board.team(
                &p.name,
                p.method,
                params.as_ref().map(|p| (p.0.as_str(), p.1.as_str())),
                &p.players,
            );
            sidebar = true;
        }
        C::SetActionBarText(p) => r.engine.action(&m::tag_text(&p.text)),
        C::SystemChat(p) => {
            let s = m::tag_text(&p.content);
            if p.overlay {
                r.engine.action(&s);
            } else {
                r.engine.chat(&s, true);
            }
        }
        C::SetTitleText(p) | C::SetSubtitleText(p) => r.engine.chat(&m::tag_text(&p.text), true),
        C::TabList(p) => {
            r.header = format!("{}\n{}", m::tag_text(&p.header), m::tag_text(&p.footer));
            tab = true;
        }
        C::PlayerInfoUpdate(p) => {
            for e in p.entries.iter().take(256) {
                if e.uuid == r.own {
                    if let Some(ms) = e.latency {
                        r.engine.latency = (ms >= 0).then_some(ms);
                    }
                }
                if let Some(display) = &e.display_name {
                    if let Some(t) = display {
                        if r.tabs.len() < 256 || r.tabs.contains_key(&e.uuid) {
                            r.tabs.insert(e.uuid, m::tag_text(t));
                        }
                    } else {
                        r.tabs.remove(&e.uuid);
                    }
                    tab = true;
                }
            }
        }
        C::PlayerInfoRemove(p) => {
            for id in &p.uuids {
                r.tabs.remove(id);
            }
            tab = true;
        }
        C::Login(_) | C::Respawn(_) => {
            r.board = m::Board::default();
            r.tabs.clear();
            r.header.clear();
            r.engine.sidebar("", &[]);
        }
        _ => {}
    }
    if sidebar {
        let (title, rows) = r.board.snapshot();
        r.engine.sidebar(&title, &rows);
    }
    if tab {
        m::refresh_tab(&mut r);
    }
}
