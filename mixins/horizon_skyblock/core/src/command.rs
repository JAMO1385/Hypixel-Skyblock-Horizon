use crate::state::{Session, Timer, Waypoint};
use crate::{Engine, text};
impl Engine {
    pub fn command(&mut self, args: &str) -> Vec<String> {
        let args = text::short(args, 512);
        let p: Vec<_> = args.split_whitespace().collect();
        let err = |s: &str| vec![format!("Error: {s}")];
        match p.first().copied().unwrap_or("help") {
            "help" => vec![
                "Horizon SkyBlock 0.1.0 | /hs status | /hs reset".into(),
                "/hs timer add 5m Name | timer list | timer remove Name".into(),
                "/hs wp add Name [x y z] | wp list | wp remove Name".into(),
                "/hs calc BUY SELL QTY [TAX%] | /hs toggle PANEL".into(),
                "/hs lobby | skyblock | hub | island | dungeon | pets | wardrobe | storage".into(),
            ],
            "status" => vec![
                format!(
                    "Hypixel: {} | SkyBlock: {} | Area: {}",
                    self.hypixel, self.skyblock, self.area
                ),
                format!(
                    "Timers: {} | Waypoints: {} | Session coins: {}",
                    self.timers.len(),
                    self.waypoints.len(),
                    text::compact(self.session.coins)
                ),
            ],
            "reset" => {
                self.session = Session {
                    started: self.now,
                    ..Default::default()
                };
                vec!["Session counters reset".into()]
            }
            "timer" => match p.get(1).copied().unwrap_or("list") {
                "list" => {
                    let v: Vec<_> = self
                        .timers
                        .iter()
                        .map(|t| {
                            format!(
                                "{}: {}",
                                t.name,
                                text::clock(t.deadline.saturating_sub(self.now))
                            )
                        })
                        .collect();
                    if v.is_empty() {
                        vec!["No active timers".into()]
                    } else {
                        v
                    }
                }
                "add" => {
                    if p.len() < 4 {
                        return err("/hs timer add 5m Name");
                    }
                    let Some(secs) = text::duration(p[2]) else {
                        return err("Duration must be 1s to 7d, e.g. 5m or 1h30m");
                    };
                    let name = p[3..].join(" ");
                    if name.chars().count() > 40 || self.timers.len() >= 16 {
                        return err("Maximum 16 timers; name up to 40 characters");
                    }
                    self.timers.retain(|t| t.name != name);
                    self.timers.push(Timer {
                        name: name.clone(),
                        deadline: self.now.saturating_add(secs),
                    });
                    vec![format!("Timer {name} set for {}", text::clock(secs))]
                }
                "remove" => {
                    let name = p.get(2..).unwrap_or(&[]).join(" ");
                    let n = self.timers.len();
                    self.timers.retain(|t| !t.name.eq_ignore_ascii_case(&name));
                    if n == self.timers.len() {
                        err("Timer not found")
                    } else {
                        vec![format!("Removed timer {name}")]
                    }
                }
                _ => err("Use timer add, list or remove"),
            },
            "wp" => match p.get(1).copied().unwrap_or("list") {
                "list" => {
                    let v: Vec<_> = self
                        .waypoints
                        .iter()
                        .map(|w| {
                            format!(
                                "{} @ {:.0} {:.0} {:.0} ({})",
                                w.name, w.at[0], w.at[1], w.at[2], w.area
                            )
                        })
                        .collect();
                    if v.is_empty() {
                        vec!["No waypoints".into()]
                    } else {
                        v
                    }
                }
                "add" => {
                    if !self.skyblock {
                        return err("Join SkyBlock to save an area waypoint");
                    }
                    if p.len() != 3 && p.len() != 6 {
                        return err("/hs wp add Name [x y z]");
                    }
                    let Some((pos, dim)) = self.position.clone() else {
                        return err("Player position is unavailable");
                    };
                    let mut at = pos;
                    if p.len() == 6 {
                        for i in 0..3 {
                            let Some(v) = p[3 + i]
                                .parse::<f64>()
                                .ok()
                                .filter(|n| n.is_finite() && n.abs() <= 30_000_000.0)
                            else {
                                return err(
                                    "Coordinates must be finite and within the world border",
                                );
                            };
                            at[i] = v;
                        }
                    }
                    if p[2].chars().count() > 40 || self.waypoints.len() >= 32 {
                        return err("Maximum 32 waypoints; name up to 40 characters");
                    }
                    self.waypoints
                        .retain(|w| w.name != p[2] || w.area != self.area);
                    self.waypoints.push(Waypoint {
                        name: p[2].into(),
                        at,
                        dimension: dim,
                        area: self.area.clone(),
                    });
                    vec![format!("Waypoint {} saved", p[2])]
                }
                "remove" => {
                    let Some(name) = p.get(2) else {
                        return err("/hs wp remove Name");
                    };
                    let n = self.waypoints.len();
                    self.waypoints
                        .retain(|w| !w.name.eq_ignore_ascii_case(name));
                    if n == self.waypoints.len() {
                        err("Waypoint not found")
                    } else {
                        vec![format!("Removed waypoint {name}")]
                    }
                }
                _ => err("Use wp add, list or remove"),
            },
            "calc" => {
                if !(4..=5).contains(&p.len()) {
                    return err("/hs calc BUY SELL QTY [TAX%]");
                }
                let vals: Option<Vec<f64>> = p[1..].iter().map(|s| text::number(s)).collect();
                let Some(v) = vals else {
                    return err("Expected non-negative finite numbers");
                };
                let tax = *v.get(3).unwrap_or(&1.25);
                if tax > 100.0 || v[2] == 0.0 {
                    return err("Tax 0..100%; quantity greater than zero");
                }
                let cost = v[0] * v[2];
                let sale = v[1] * v[2] * (1.0 - tax / 100.0);
                vec![format!(
                    "Profit: {:.2} coins | Cost: {:.2} | Net sale: {:.2} | Tax {:.2}% (manual estimate)",
                    sale - cost,
                    cost,
                    sale,
                    tax
                )]
            }
            _ => err("Unknown command; use /hs help"),
        }
    }
}
