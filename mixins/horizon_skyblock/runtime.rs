// Native runtime. All game calls happen outside the lock.
pub use horizon_skyblock_core::scoreboard::Board;
use horizon_skyblock_core::{Engine, Settings};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Mutex, OnceLock};
pub const ID: &str = "horizon_skyblock";
pub const NAME: &str = "Horizon SkyBlock";
pub const VERSION: &str = "0.1.0";
pub const KEYS: [(&str, &str, &str); 6] = [
    ("hud", "Toggle HUD", "key.keyboard.h"),
    ("skyblock", "Join SkyBlock", "key.keyboard.f6"),
    ("hub", "SkyBlock Hub", "key.keyboard.f7"),
    ("island", "Your island", "key.keyboard.f8"),
    ("lobby", "Hypixel lobby", "key.keyboard.f9"),
    ("waypoint", "Save current waypoint", "key.keyboard.n"),
];
pub struct Runtime {
    pub engine: Engine,
    pub settings: Settings,
    pub board: Board,
    pub loaded: bool,
    pub visible: bool,
    pub held: BTreeSet<String>,
    pub tabs: BTreeMap<u128, String>,
    pub header: String,
    pub own: u128,
    pub last_tick: u64,
    pub shortcut_at: u64,
}
impl Default for Runtime {
    fn default() -> Self {
        Self {
            engine: Engine::default(),
            settings: Settings::default(),
            board: Board::default(),
            loaded: false,
            visible: true,
            held: BTreeSet::new(),
            tabs: BTreeMap::new(),
            header: String::new(),
            own: 0,
            last_tick: 0,
            shortcut_at: 0,
        }
    }
}
pub fn runtime() -> std::sync::MutexGuard<'static, Runtime> {
    static S: OnceLock<Mutex<Runtime>> = OnceLock::new();
    S.get_or_init(|| Mutex::new(Runtime::default()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}
pub fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
pub(super) fn initialize(game: &crate::game::Game<'_>) {
    if runtime().loaded {
        return;
    }
    let settings = Settings::read(|k| game.native_setting(ID, k));
    let timers = game.native_setting(ID, "timers").unwrap_or_default();
    let points = game.native_setting(ID, "waypoints").unwrap_or_default();
    let mut r = runtime();
    r.settings = settings;
    r.engine.tick(now());
    r.engine.load_timers(&timers);
    r.engine.load_waypoints(&points);
    r.loaded = true;
}
pub(super) fn persist(game: &mut crate::game::Game<'_>) {
    let pairs = {
        let r = runtime();
        let mut v = r.settings.store();
        v.push(("timers", r.engine.save_timers()));
        v.push(("waypoints", r.engine.save_waypoints()));
        v
    };
    for (k, v) in pairs {
        game.native_set_setting(ID, k, &v);
    }
}
pub fn tag_text(t: &horizon_world::nbt::Tag) -> String {
    horizon_client::scoreboard::comp_text(&horizon_client::scoreboard::tag_to_json(t)).plain()
}
pub(super) fn sync(game: &crate::game::Game<'_>) {
    let addr = game
        .native_client_state()
        .and_then(|s| s.server_address)
        .unwrap_or_default();
    let position = game
        .native_player()
        .map(|p| ([p.x, p.y, p.z], p.dimension.to_string()));
    let mut r = runtime();
    r.engine.set_server(&addr);
    r.engine.position = position;
    // Only the real local player's UUID is used for the tab latency. This read pins this build.
    r.own = game.own_uuid();
}
pub fn refresh_tab(r: &mut Runtime) {
    let mut s = r.header.clone();
    for text in r.tabs.values().take(160) {
        s.push('\n');
        s.push_str(text);
    }
    r.engine.tab(&s);
}
pub fn shortcut(name: &str) -> Option<&'static str> {
    match name {
        "skyblock" => Some("/play skyblock"),
        "hub" => Some("/hub"),
        "island" => Some("/is"),
        "lobby" => Some("/lobby"),
        "dungeon" => Some("/warp dungeon_hub"),
        "pets" => Some("/pets"),
        "wardrobe" => Some("/wardrobe"),
        "storage" => Some("/storage"),
        _ => None,
    }
}
