use crate::text;
use std::collections::{BTreeMap, BTreeSet};
#[derive(Default)]
struct Team {
    prefix: String,
    suffix: String,
    members: BTreeSet<String>,
}
#[derive(Default)]
pub struct Board {
    titles: BTreeMap<String, String>,
    active: String,
    scores: BTreeMap<(String, String), (i32, Option<String>)>,
    teams: BTreeMap<String, Team>,
}
impl Board {
    pub fn objective(&mut self, name: &str, method: i8, title: &str) {
        if method == 1 {
            self.titles.remove(name);
            self.scores.retain(|(o, _), _| o != name);
            if self.active == name {
                self.active.clear();
            }
        } else if self.titles.len() < 64 || self.titles.contains_key(name) {
            self.titles
                .insert(text::short(name, 256), text::short(title, 256));
        }
    }
    pub fn display(&mut self, slot: i32, name: &str) {
        if slot == 1 {
            self.active = text::short(name, 256);
        }
    }
    pub fn score(&mut self, owner: &str, objective: &str, value: i32, display: Option<&str>) {
        if self.scores.len() >= 2048 && !self.scores.contains_key(&(objective.into(), owner.into()))
        {
            return;
        }
        // Raw owner is significant: legacy formatting is used as an invisible identity.
        self.scores.insert(
            (
                objective.chars().take(256).collect(),
                owner.chars().take(256).collect(),
            ),
            (value, display.map(|s| s.chars().take(512).collect())),
        );
    }
    pub fn reset(&mut self, owner: &str, objective: Option<&str>) {
        self.scores
            .retain(|(o, n), _| n != owner || objective.is_some_and(|want| want != o));
    }
    pub fn team(
        &mut self,
        name: &str,
        method: i8,
        parameters: Option<(&str, &str)>,
        members: &[String],
    ) {
        if name.len() > 256 {
            return;
        }
        if method == 1 {
            self.teams.remove(name);
            return;
        }
        if self.teams.len() >= 512 && !self.teams.contains_key(name) {
            return;
        }
        if method == 0 {
            self.teams.remove(name);
        }
        if matches!(method, 0 | 3) {
            for t in self.teams.values_mut() {
                for m in members.iter().take(512) {
                    t.members.remove(m);
                }
            }
        }
        let t = self.teams.entry(name.into()).or_default();
        if let Some((p, s)) = parameters {
            t.prefix = p.chars().take(512).collect();
            t.suffix = s.chars().take(512).collect();
        }
        if matches!(method, 0 | 3) {
            for m in members.iter().take(512) {
                if t.members.len() < 512 {
                    t.members.insert(m.clone());
                }
            }
        }
        if method == 4 {
            for m in members {
                t.members.remove(m);
            }
        }
    }
    pub fn snapshot(&self) -> (String, Vec<String>) {
        let title = self.titles.get(&self.active).cloned().unwrap_or_default();
        let mut rows: Vec<_> = self
            .scores
            .iter()
            .filter(|((o, n), _)| o == &self.active && !n.starts_with('#'))
            .collect();
        rows.sort_by(|a, b| b.1.0.cmp(&a.1.0).then(a.0.1.cmp(&b.0.1)));
        let rows = rows
            .into_iter()
            .take(15)
            .map(|((_, owner), (_, display))| {
                if let Some(d) = display {
                    return text::short(d, 256);
                }
                if let Some(t) = self.teams.values().find(|t| t.members.contains(owner)) {
                    text::short(&format!("{}{}{}", t.prefix, owner, t.suffix), 256)
                } else {
                    text::short(owner, 256)
                }
            })
            .collect();
        (title, rows)
    }
}
