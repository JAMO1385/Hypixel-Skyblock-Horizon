use crate::{Settings, text};
use std::collections::{BTreeMap, VecDeque};
#[derive(Default, Debug)]
pub struct Session {
    pub slayer: u32,
    pub rare_drops: u32,
    pub sea_creatures: u32,
    pub deaths: u32,
    pub party_deaths: u32,
    pub skill_xp: f64,
    pub coins: f64,
    pub started: u64,
}
#[derive(Clone, Debug)]
pub struct Timer {
    pub name: String,
    pub deadline: u64,
}
#[derive(Clone, Debug)]
pub struct Waypoint {
    pub name: String,
    pub at: [f64; 3],
    pub dimension: String,
    pub area: String,
}
#[derive(Clone, Debug)]
pub struct Alert {
    pub text: String,
    pub until: u64,
}
#[derive(Clone, Debug)]
pub struct Line {
    pub text: String,
    pub color: u32,
}
#[derive(Default)]
pub struct Engine {
    pub hypixel: bool,
    pub skyblock: bool,
    pub area: String,
    pub health: Option<(f64, f64)>,
    pub mana: Option<(f64, f64)>,
    pub metrics: BTreeMap<String, String>,
    pub slayer: String,
    pub session: Session,
    pub timers: Vec<Timer>,
    pub waypoints: Vec<Waypoint>,
    pub alerts: VecDeque<Alert>,
    pub position: Option<([f64; 3], String)>,
    pub now: u64,
    pub latency: Option<i32>,
    sidebar_rows: Vec<String>,
    tab_rows: Vec<String>,
    low_health: bool,
    low_mana: bool,
    action_at: u64,
    pending: Vec<String>,
    last_skill: String,
    skill_at: u64,
    mana_at: u64,
}
impl Engine {
    pub fn set_server(&mut self, addr: &str) {
        let host = addr
            .split(':')
            .next()
            .unwrap_or("")
            .trim_end_matches('.')
            .to_ascii_lowercase();
        let yes = host == "hypixel.net" || host.ends_with(".hypixel.net");
        if self.hypixel && !yes {
            self.disconnect();
        }
        self.hypixel = yes;
    }
    pub fn disconnect(&mut self) {
        let timers = std::mem::take(&mut self.timers);
        let waypoints = std::mem::take(&mut self.waypoints);
        *self = Self::default();
        self.timers = timers;
        self.waypoints = waypoints;
    }
    fn clear_sky(&mut self) {
        self.health = None;
        self.mana = None;
        self.metrics.clear();
        self.slayer.clear();
        self.area.clear();
        self.tab_rows.clear();
        self.low_health = false;
        self.low_mana = false;
        self.alerts.clear();
    }
    pub fn sidebar(&mut self, title: &str, rows: &[String]) {
        let active = self.hypixel && text::clean(title).to_ascii_uppercase().contains("SKYBLOCK");
        if self.skyblock && !active {
            self.clear_sky();
        }
        self.skyblock = active;
        self.sidebar_rows = rows.iter().take(32).map(|s| text::short(s, 256)).collect();
        if active {
            let next = self
                .sidebar_rows
                .iter()
                .find_map(|s| s.split_once('⏣').map(|p| p.1.trim().to_owned()));
            if next
                .as_ref()
                .is_some_and(|a| !self.area.is_empty() && a != &self.area)
            {
                self.tab_rows.clear();
            }
            self.rebuild();
        }
    }
    pub fn tab(&mut self, s: &str) {
        if !self.skyblock {
            return;
        }
        self.tab_rows = text::clean(s)
            .lines()
            .take(160)
            .map(|s| text::short(s, 256))
            .collect();
        self.rebuild();
    }
    fn rebuild(&mut self) {
        self.metrics.clear();
        let mut area = String::new();
        for s in self.sidebar_rows.iter().chain(&self.tab_rows) {
            let s = s.trim();
            if let Some((_, name)) = s.split_once('⏣') {
                area = name.trim().into();
            }
            let Some((k, v)) = s.split_once(':') else {
                if s.contains("The Catacombs") || s.contains("Master Mode") {
                    self.metrics.insert("Dungeon".into(), s.into());
                }
                continue;
            };
            let k = k.trim();
            let v = v.trim();
            if v.is_empty() {
                continue;
            }
            if k == "Area" {
                area = v.into();
                continue;
            }
            let canonical = match k {
                "Purse" | "Piggy" => "Purse",
                "Bits" => "Bits",
                "Copper" => "Copper",
                "Bank" => "Bank",
                "Mithril Powder" => "Mithril Powder",
                "Gemstone Powder" => "Gemstone Powder",
                "Glacite Powder" => "Glacite Powder",
                "Pet" => "Pet",
                "Profile" => "Profile",
                "Server" => "Server",
                "Speed" => "Speed",
                "Time Elapsed" => "Time Elapsed",
                "Dungeon Cleared" | "Cleared" => "Dungeon Cleared",
                "Secrets Found" => "Secrets Found",
                "Secrets" => "Secrets",
                "Puzzles" => "Puzzles",
                "Deaths" => "Deaths",
                "Crypts" => "Crypts",
                "Score" => "Score",
                "Cookie Buff" => "Cookie Buff",
                "God Potion" => "God Potion",
                "Soulflow" => "Soulflow",
                "Farming Fortune" => "Farming Fortune",
                "Mining Speed" => "Mining Speed",
                "Mining Fortune" => "Mining Fortune",
                "Pristine" => "Pristine",
                "Jacob's Contest" => "Jacob's Contest",
                "Collected" => "Collected",
                "Medal" => "Medal",
                "Crop" => "Crop",
                "Contest" => "Contest",
                "Commissions" => "Commissions",
                "Location" => "Location",
                "Time Left" => "Time Left",
                "Sacks" => "Sacks",
                "Fishing Speed" => "Fishing Speed",
                "Sea Creature Chance" => "Sea Creature Chance",
                _ => continue,
            };
            let value = if matches!(
                canonical,
                "Purse"
                    | "Bits"
                    | "Copper"
                    | "Bank"
                    | "Mithril Powder"
                    | "Gemstone Powder"
                    | "Glacite Powder"
            ) {
                text::first_number(v)
                    .map(text::compact)
                    .unwrap_or_else(|| v.into())
            } else {
                text::short(v, 80)
            };
            if self.metrics.len() < 48 {
                self.metrics.insert(canonical.into(), value);
            }
        }
        if !area.is_empty() && self.area != area {
            if !self.area.is_empty() {
                self.slayer.clear();
                self.health = None;
                self.mana = None;
            }
            self.area = area;
        }
    }
    pub fn action(&mut self, input: &str) {
        if !self.skyblock {
            return;
        }
        let s = text::clean(input);
        if let Some(v) = text::ratio_before(&s, '❤') {
            self.health = Some(v);
            self.action_at = self.now;
        }
        if let Some(v) = text::ratio_before(&s, '✎') {
            self.mana = Some(v);
            self.mana_at = self.now;
        }
        if let Some((before, _)) = s.split_once('❈')
            && let Some(v) = before.split_whitespace().next_back().and_then(text::number)
        {
            self.metrics.insert("Defense".into(), text::compact(v));
        }
        if let Some((_, after)) = s.split_once('+')
            && let Some((num, tail)) = after.split_once(' ')
            && [
                "Farming",
                "Mining",
                "Combat",
                "Foraging",
                "Fishing",
                "Enchanting",
                "Alchemy",
                "Taming",
                "Carpentry",
                "Runecrafting",
                "Social",
            ]
            .iter()
            .any(|k| tail.starts_with(k))
            && (s != self.last_skill || self.now > self.skill_at + 1)
            && let Some(n) = text::number(num)
        {
            self.session.skill_xp += n;
            self.metrics.insert("Skill".into(), text::short(tail, 80));
            self.last_skill = s.clone();
            self.skill_at = self.now;
        }
    }
    pub fn chat(&mut self, input: &str, system: bool) {
        if !self.skyblock || !system {
            return;
        }
        let s = text::short(input, 1024);
        let player_prefix = s.split_once(':').map(|(prefix, _)| prefix.trim());
        let is_player_chat = player_prefix.is_some_and(|prefix| {
            prefix.starts_with('[')
                || ["Party >", "Guild >", "Officer >", "Co-op >", "From "]
                    .iter()
                    .any(|kind| prefix.starts_with(kind))
        });
        if is_player_chat {
            return;
        }
        let lower = s.to_ascii_lowercase();
        if s.contains("SLAYER QUEST STARTED") {
            self.slayer = "Quest active".into();
        }
        if lower.contains("slay the boss") {
            self.slayer = "Boss spawned".into();
            self.alert("Slayer boss spawned!");
        }
        if s.contains("SLAYER QUEST COMPLETE") {
            self.slayer = "Complete".into();
            self.session.slayer = self.session.slayer.saturating_add(1);
            self.alert("Slayer quest complete");
        }
        if s.contains("RARE DROP!") || s.contains("CRAZY RARE DROP!") || s.contains("RNGesus DROP!")
        {
            self.session.rare_drops = self.session.rare_drops.saturating_add(1);
            self.alert(&s);
        }
        if lower.starts_with("you caught a ") || lower.contains("a sea creature has spawned") {
            self.session.sea_creatures = self.session.sea_creatures.saturating_add(1);
        }
        if lower.contains("became a ghost") {
            self.session.party_deaths = self.session.party_deaths.saturating_add(1);
        }
        if lower.starts_with("you died") || s.starts_with("☠ You ") {
            self.session.deaths = self.session.deaths.saturating_add(1);
            self.alert("You died");
        }
        if lower.contains("inventory is full") || lower.contains("your inventory is full") {
            self.alert("Inventory full");
        }
        if lower.contains("cookie buff has expired") || lower.contains("god potion has expired") {
            self.alert(&s);
        }
        for (needle, label) in [
            (
                "your pickaxe ability is now available",
                "Pickaxe ability ready",
            ),
            ("your ability is now available", "Ability ready"),
            ("a special zealot has spawned", "Special Zealot spawned"),
            (
                "you have unclaimed slayer rewards",
                "Claim your Slayer rewards",
            ),
        ] {
            if lower.contains(needle) {
                self.alert(label);
            }
        }
        if let Some(tail) = s
            .strip_prefix("You earned ")
            .or_else(|| s.strip_prefix("You found "))
            && tail.contains(" coins")
            && let Some(n) = text::first_number(tail)
        {
            self.session.coins += n;
        }
    }
    pub fn alert(&mut self, s: &str) {
        if self
            .alerts
            .iter()
            .any(|a| a.text == s && a.until > self.now)
        {
            return;
        }
        if self.alerts.len() >= 8 {
            self.alerts.pop_front();
        }
        let text = text::short(s, 100);
        self.alerts.push_back(Alert {
            text: text.clone(),
            until: self.now.saturating_add(7),
        });
    }
    pub fn tick(&mut self, now: u64) {
        self.now = now;
        if self.session.started == 0 {
            self.session.started = now;
        }
        self.alerts.retain(|a| a.until > now);
        let mut ready = Vec::new();
        self.timers.retain(|t| {
            if t.deadline <= now {
                ready.push(format!("Timer finished: {}", t.name));
                false
            } else {
                true
            }
        });
        for s in ready {
            self.alert(&s);
            self.pending.push(s);
        }
        if !self.skyblock {
            return;
        }
        if self.action_at > 0 && now.saturating_sub(self.action_at) > 15 {
            self.health = None;
        }
        if self.mana_at > 0 && now.saturating_sub(self.mana_at) > 15 {
            self.mana = None;
        }
        if let Some((a, b)) = self.health {
            let low = a / b <= 0.2;
            if low && !self.low_health {
                self.alert("Low health (20% or less)");
            }
            self.low_health = low;
        }
        if let Some((a, b)) = self.mana {
            let low = a / b <= 0.1;
            if low && !self.low_mana {
                self.alert("Low mana (10% or less)");
            }
            self.low_mana = low;
        }
    }
    pub fn notices(&mut self) -> Vec<String> {
        std::mem::take(&mut self.pending)
    }
    pub fn waypoint_lines(&self) -> Vec<String> {
        let Some((pos, dim)) = &self.position else {
            return vec![];
        };
        self.waypoints
            .iter()
            .filter(|w| &w.dimension == dim && w.area == self.area)
            .take(4)
            .map(|w| {
                let dx = w.at[0] - pos[0];
                let dz = w.at[2] - pos[2];
                let d = (dx * dx + (w.at[1] - pos[1]).powi(2) + dz * dz).sqrt();
                let direction = if dx.abs() > dz.abs() {
                    if dx > 0.0 { "E" } else { "W" }
                } else if dz > 0.0 {
                    "S"
                } else {
                    "N"
                };
                format!("{}: {d:.0}m {direction}", w.name)
            })
            .collect()
    }
    pub fn lines(&self, settings: &Settings) -> Vec<Line> {
        let mut v = Vec::new();
        let mut add = |s: String, color: u32| {
            if v.len() < 32 {
                v.push(Line { text: s, color });
            }
        };
        if !self.hypixel {
            return v;
        }
        add(
            if self.skyblock {
                format!(
                    "Horizon SkyBlock | {}",
                    if self.area.is_empty() {
                        "Loading..."
                    } else {
                        &self.area
                    }
                )
            } else {
                "Horizon | Hypixel lobby".into()
            },
            0xB79CFF,
        );
        if self.skyblock {
            if settings.on("stats") {
                if let Some((a, b)) = self.health {
                    add(
                        format!("HP {}/{}", text::compact(a), text::compact(b)),
                        0xFF7373,
                    );
                }
                if let Some((a, b)) = self.mana {
                    add(
                        format!("Mana {}/{}", text::compact(a), text::compact(b)),
                        0x69CFFF,
                    );
                }
            }
            for (k, val) in &self.metrics {
                let feature = match k.as_str() {
                    "Purse" | "Bank" | "Bits" | "Copper" => "economy",
                    k if k.contains("Powder")
                        || k.starts_with("Mining")
                        || k == "Pristine"
                        || k == "Commissions" =>
                    {
                        "mining"
                    }
                    "Crop" | "Contest" | "Jacob's Contest" | "Collected" | "Medal"
                    | "Farming Fortune" | "Time Left" => "farming",
                    "Dungeon" | "Time Elapsed" | "Dungeon Cleared" | "Secrets Found"
                    | "Secrets" | "Puzzles" | "Crypts" | "Deaths" | "Score" => "dungeon",
                    _ => "stats",
                };
                if settings.on(feature) {
                    add(format!("{k}: {val}"), 0xE2E2EF);
                }
            }
            if settings.on("slayer") && !self.slayer.is_empty() {
                add(
                    format!(
                        "Slayer: {} | {} completed",
                        self.slayer, self.session.slayer
                    ),
                    0xFFA966,
                );
            }
            if settings.on("session") && !settings.compact {
                add(
                    format!(
                        "Session {} | XP {} | Slayer {}",
                        text::clock(self.now.saturating_sub(self.session.started)),
                        text::compact(self.session.skill_xp),
                        self.session.slayer
                    ),
                    0xA3A3BC,
                );
                add(
                    format!(
                        "Drops {} | Sea creatures {} | Deaths {}",
                        self.session.rare_drops, self.session.sea_creatures, self.session.deaths
                    ),
                    0xA3A3BC,
                );
                if self.session.party_deaths > 0 {
                    add(
                        format!("Party deaths: {}", self.session.party_deaths),
                        0xFFA966,
                    );
                }
            }
        }
        if settings.on("timers") {
            for t in self.timers.iter().take(4) {
                add(
                    format!(
                        "{}: {}",
                        t.name,
                        text::clock(t.deadline.saturating_sub(self.now))
                    ),
                    0xFFD369,
                );
            }
        }
        if settings.on("waypoints") {
            for line in self.waypoint_lines() {
                add(line, 0x65E5BD);
            }
        }
        if settings.on("alerts") {
            for a in self.alerts.iter().rev().take(2) {
                add(format!("! {}", a.text), 0xFFAE69);
            }
        }
        v
    }
    pub fn save_timers(&self) -> String {
        self.timers
            .iter()
            .map(|t| format!("{}\t{}", t.deadline, t.name))
            .collect::<Vec<_>>()
            .join("\n")
    }
    pub fn load_timers(&mut self, s: &str) {
        self.timers.clear();
        for line in s.lines().take(16) {
            if let Some((d, n)) = line.split_once('\t')
                && let Ok(deadline) = d.parse::<u64>()
                && deadline > 0
            {
                self.timers.push(Timer {
                    name: text::short(n, 40).replace('\t', " "),
                    deadline,
                });
            }
        }
    }
    pub fn save_waypoints(&self) -> String {
        self.waypoints
            .iter()
            .map(|w| {
                format!(
                    "{}\t{}\t{}\t{}\t{}\t{}",
                    w.name, w.at[0], w.at[1], w.at[2], w.dimension, w.area
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
    pub fn load_waypoints(&mut self, s: &str) {
        self.waypoints.clear();
        for line in s.lines().take(32) {
            let p: Vec<_> = line.split('\t').collect();
            if p.len() != 6 {
                continue;
            }
            let xyz: Option<Vec<f64>> = p[1..4]
                .iter()
                .map(|p| {
                    p.parse::<f64>()
                        .ok()
                        .filter(|n| n.is_finite() && n.abs() <= 30_000_000.0)
                })
                .collect();
            if let Some(xyz) = xyz {
                self.waypoints.push(Waypoint {
                    name: text::short(p[0], 40),
                    at: [xyz[0], xyz[1], xyz[2]],
                    dimension: text::short(p[4], 100),
                    area: text::short(p[5], 100),
                });
            }
        }
    }
}
