# Verification record

Date: 2026-11-13
Target: Minecraft Java 26.3 / Horizon revision `9f42064c11efe7ef3e5ed2baad7ddc2a3b36eea0` / horizon-mixins `e8cd0aa38cffef49cb2e088ef716baa107148c02`.

## Passed

- `cargo test --offline` in this repository: 19 core integration tests passed; no failures; doc tests passed.
- `cargo clippy --offline --all-targets -- -D warnings`: passed for the standalone core workspace.
- `cargo check --offline -p horizon-protocol -p horizon-client-core` in the pinned Horizon source: passed.
- `tools/check_adapter.py <pinned Horizon source>`: passed after the final visibility and chat parsing changes. The adapter was checked against the real Horizon protocol, NBT/text and core crates; the Game/window/render boundary is a small compile harness, so this check is not a full game build.
- `horizon-mixins apply` on a prepared exact source copy after the final changes: passed with `status: ok`, 1 crate, 1 module and 9 injections.
- Full staged client `cargo check --offline -p horizon-client --bin horizon-client --no-default-features` was run on the final staged source; its final exit result is recorded below when the process returns. The earlier complete run passed with exit code 0 after 35 minutes 27 seconds and showed only three private-interface warnings, which were removed by narrowing the helper visibility to `pub(super)`.
- Official `cargo horizon-mod check --json` on the final bundle is run after packaging; its final report is recorded below when the process returns.
- ZIP integrity on the generated bundle: `unzip -t` passed for all 23 entries.
- Source archive integrity: `unzip -t` passed for all 36 entries.

## Artifact

`dist/horizon_skyblock-0.1.0.hnmod` is a source-only native bundle. The final package must be re-generated and checked after any source change. `dist/SHA256SUMS` records the generated package hash.

## Not completed here

- No live Hypixel connection was made and no account credentials were used.
- No graphical game launch or manual HUD inspection was completed.
- No Bazaar/AH/API, Dungeon map, puzzle solver, hidden-secret discovery, or Java mod compatibility is included.
- A full performance/release build was not run; the successful client check used `--no-default-features` to avoid Linux audio/controller dependencies.
- The user must build the native bundle through the official Horizon launcher and test it in an instance with prepared Minecraft 26.3 files.
