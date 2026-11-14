# Horizon SkyBlock

Rust native Horizon mod for Hypixel lobbies and SkyBlock. [Persian installation and usage](README.fa.md).

Source-pinned first release: Minecraft 26.3, Horizon `9f42064c11efe7ef3e5ed2baad7ddc2a3b36eea0`, horizon-mixins `e8cd0aa38cffef49cb2e088ef716baa107148c02`.

- Server-visible HUD data, session counters, Slayer/dungeon information and warnings.
- Persistent user timers and area-scoped local waypoints.
- Lobby commands, FPS/CPS/keys and server-reported local-player latency.
- Twelve configurable panels, HUD corner and compact mode.
- No account credentials, external API calls, polling or gameplay automation.

Data is parsed only when present in the server's UI packets. No live market prices, profile API, puzzle solvers, dungeon map or hidden-secret discovery. This is not feature parity with all established Java SkyBlock mods. No live Hypixel or graphical game test has been completed; see `VERIFICATION.md` for the exact checks.

## Install a local bundle

Unzip the `.hnmod` as a ZIP archive, then use the official launcher:

```sh
horizon-launcher native-build "/path/to/unpacked-mod" --trust-native-code --profile dev
```

The unpacked folder contains `horizon-native.json` and `mixins/`. Select the desired instance first. Sign in yourself using the launcher's official Microsoft flow. Start the built instance, connect to Hypixel, use `/hs help` and Options > Mods > Horizon SkyBlock.

A pinned bundle can conflict with other mods pinned to different revisions. The launcher must provide compatible prepared game files. Hypixel must accept the client's protocol; this mod does not implement version translation.

## Development

```sh
cargo test --offline
cargo clippy --offline --all-targets -- -D warnings
python tools/manifest.py
python tools/package.py
```

`tools/manifest.py` writes the manifest from the checked-in compatibility record. `tools/package.py` makes a deterministic source-only native bundle and SHA256SUMS. Bundle format follows upstream `native/tools/native.py`; the hooks metadata is deliberately omitted to keep the revision pinned because local UUID lookup uses an internal method. The core has no external dependencies. See the design/plan in `docs/superpowers/` and full validation record in `VERIFICATION.md`.
