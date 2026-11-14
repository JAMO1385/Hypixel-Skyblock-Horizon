# Horizon Hypixel Companion

Requested: a Rust native Horizon mod for Hypixel lobbies and a substantial SkyBlock experience. User approved the proposed HUD/lobby/settings scope and asked for the SkyBlock portion to be complete. This is a first release, not a claim of parity with all existing Java mods.

Deliverables: installable `horizon_skyblock-0.1.0.hnmod`, full source archive, Persian installation guide, recorded real checks. Target Minecraft 26.3, Horizon nightly 9f42064c11efe7ef3e5ed2baad7ddc2a3b36eea0 (2026-11-13), stable native hooks >=411. No redistribution of Minecraft game files. No publication to registry without a user request.

Implementation decisions delegated to builder: native mixin helpers + a reusable dependency-free Rust core crate, automatic context from visible server UI, English in-game labels (Minecraft RTL rendering is unverified), Persian README. Live API services are excluded from this offline first release; unknown values remain unknown. No invented ping or stats.

Features:
- Automatic Hypixel/SkyBlock detection from received scoreboard and Tab data, isolated state on disconnect and area changes.
- Health, defense, mana, purse/bits/copper, area/profile/server/time, speed, pet, mining powder and farming contest data when the server supplies them.
- Slayer quest/boss state and session kills; dungeon time/floor/clear/secrets/puzzles/party deaths from server text; session death/rare drop/fishing/skill XP counters; warning notifications.
- Cooldown/countdown and custom reminder management, low-health/mana and full-inventory alerts.
- Local waypoints with dimension/area scope and distance/bearing; no hidden-object discovery.
- FPS, genuine tab-list latency for the local player, keystrokes; manual one-command shortcuts for SkyBlock/lobby/hub/island/dungeon hub/wardrobe/pets/storage.
- Configurable panels, HUD corner, compact mode, per-feature toggles persisted using Horizon settings. Mod off means no actions/overlay.
- `/hs` local command family for help, status, timers, waypoints, resets, toggles and arithmetic sale/margin calculator. No polling, combat/interaction automation, packet changes or unsolicited server commands.

Architecture: pure Rust core handles formatting/parsing/state/config/scoreboard/commands/view models; small native helpers adapt real Horizon packet/Text types, keys, HUD, settings, tooltip and disconnect. Mutex-protected global runtime supports reentrant hook constraints; never hold a lock while calling into game hooks. Packet counts/text, timers, maps and notifications bounded. No panics on malformed server strings.

Validation: core tests exercise hand-authored protocol/text fixtures, not live captures; real horizon-mixins apply; cargo check on real pinned game where environment permits; archive checker on resulting .hnmod; runtime adapter compile with upstream types. Document separately any missing in-game test and build blocker. Never describe synthetic fixtures as live Hypixel output.
