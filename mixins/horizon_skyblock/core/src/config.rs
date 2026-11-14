pub const FEATURES: [(&str, &str); 12] = [
    ("stats", "Health, mana, defense"),
    ("economy", "Purse, bits, copper"),
    ("mining", "Mining powders"),
    ("farming", "Farming and contests"),
    ("slayer", "Slayer tracker"),
    ("dungeon", "Dungeon tracker"),
    ("session", "Session statistics"),
    ("timers", "Timers and reminders"),
    ("waypoints", "Local waypoints"),
    ("performance", "FPS and ping"),
    ("keys", "Keystrokes"),
    ("alerts", "Warnings"),
];
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Settings {
    pub mask: u16,
    pub corner: usize,
    pub compact: bool,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            mask: 0xfff,
            corner: 0,
            compact: false,
        }
    }
}
impl Settings {
    pub fn on(&self, key: &str) -> bool {
        FEATURES
            .iter()
            .position(|p| p.0 == key)
            .is_some_and(|i| self.mask & (1 << i) != 0)
    }
    pub fn toggle(&mut self, key: &str) -> bool {
        if let Some(i) = FEATURES.iter().position(|p| p.0 == key) {
            self.mask ^= 1 << i;
            true
        } else {
            false
        }
    }
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        Self {
            mask: get("panels")
                .and_then(|v| v.trim().parse().ok())
                .unwrap_or(0xfff)
                & 0xfff,
            corner: get("corner")
                .and_then(|v| v.trim().parse::<usize>().ok())
                .filter(|v| *v < 4)
                .unwrap_or(0),
            compact: get("compact").is_some_and(|v| v.trim() == "true"),
        }
    }
    pub fn store(&self) -> Vec<(&'static str, String)> {
        vec![
            ("panels", self.mask.to_string()),
            ("corner", self.corner.to_string()),
            ("compact", self.compact.to_string()),
        ]
    }
    pub fn labels(&self) -> Vec<String> {
        let mut v = vec![
            format!(
                "HUD corner: {}",
                ["Top left", "Top right", "Bottom left", "Bottom right"][self.corner]
            ),
            format!("Compact HUD: {}", self.compact),
        ];
        for (k, t) in FEATURES {
            v.push(format!("{t}: {}", if self.on(k) { "ON" } else { "OFF" }));
        }
        v
    }
    pub fn click(&mut self, i: usize) {
        if i == 0 {
            self.corner = (self.corner + 1) % 4;
        } else if i == 1 {
            self.compact = !self.compact;
        } else if let Some((k, _)) = FEATURES.get(i - 2) {
            self.toggle(k);
        }
    }
}
