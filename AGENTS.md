# AGENTS.md

Windows-oriented Tauri 2 + React 19 desktop tool for Heroes of Might & Magic V map/mod generation. The Rust backend mutates game files and a local SQLite DB; the React UI calls it through Tauri `invoke`.

## Layout and boundaries

- `src-tauri/` is the application crate. `src/main.rs` initializes rustls/Tokio and calls `lib.rs::run()`. Every backend command is registered in the single `invoke_handler!` list in `src-tauri/src/lib.rs`.
- Backend features live in `src-tauri/src/services/<feature>/`. To add or enable one, update `src-tauri/src/services/mod.rs`, the feature `prelude.rs` if present, and the handler list in `lib.rs`.
- `src/` is the React app. Routes are in `src/components/body/index.tsx` (`/editor/:state/*`); editor IDs are in `src/stores/EditorStateStore.ts`; each editor owns an `api.ts` and a Zustand `store.ts`. `@/` aliases `src/`.
- Rust crates are independent; there is no root `Cargo.toml` workspace:
  - `homm5-types`: serde models for HoMM5 XDB/XML.
  - `homm5-scaner`: scans `.pak/.h5m/.h5c/.h5u`, populates SQLite, emits Lua/output pak.
  - `editor-tools`: SeaORM repositories and generators for quests, dialogs, fights, banks, reserve heroes.
  - `map-modifier`: rewrites map `.xdb` and generates Lua.
  - `homm5-repacker`: zips directories into `.pak`.
  - `runtime-main`: Frida injection into the game.
  - `google-api`: Google Sheets/Docs sync.

## Commands

- Full app (required for backend work): `npm run tauri dev` (Vite on fixed port 1420).
- UI only: `npm run dev`. Frontend/typecheck build: `npm run build` (`tsc && vite build`). Packaged build: `npm run tauri build`.
- Rust must target a crate because the repo root has no manifest:
  - `cargo check --manifest-path src-tauri/Cargo.toml`
  - `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets`
  - Use the same `--manifest-path` for one crate, e.g. `homm5-scaner/Cargo.toml`.
- Rust crates use edition 2024. Use npm (`package-lock.json`).
- There are no tests, lint script, formatter config, or CI workflows. Do not invent a single-test command; verify with `npm run build`, clippy, and a real Tauri run.

## Runtime data and config

- `cfg/` is resolved relative to the running executable, not the repo: `src-tauri/target/debug/cfg/` in dev, beside the exe in release. No `cfg/` is committed; a fresh run fails or panics until it exists.
- Startup reads `cfg/main.json`, `runtime.json`, `current_map_data.json`, `modifiers.json`, `objects_data.json`, `auth/client-secret.json`, and `cfg/<profile>/profile.json`. In `profile.json`, `map_path`, `texts_path`, `repackers`, and `maps` paths are relative to `game_path`; the loader joins them at startup.
- SQLite is `cfg/<profile>/runtime_database.db`. The app creates a missing file but never creates tables. SQL in `src-tauri/migrations/` is not referenced by build/runtime code; SeaORM entities are the current schema, and several current tables (e.g. `quest_progresses`, `reserve_heroes`, fight-generator tables) have no migration. Prepare and sync the DB manually.
- Most editors read data populated by `execute_scan`; an unscanned/empty DB leaves them empty.
- Windows paths are assumed: hard-coded `\` separators and `D:\...` debug dumps exist in scanner writers, `generate_images`, and `sheets_connector`.

## Behavior you should not "fix" casually

- `execute_scan` scans `data/`, `Maps/`, `UserMODs/` under the profile game path and writes `<profile>-generated.pak` into `data/`.
- `run_game` repacks profile-defined directories, then launches the game through Frida. `homm5-repacker` deliberately stamps entries with 2107-12-31 so generated paks win over originals.
- `apply_modifications` rewrites the selected map `.xdb` in place (delete + recreate) and writes generated Lua beside `map.data_path`; there is no backup.
- XDB handling is serde/quick-xml based. Field names/casing and `@href` attributes matter. Game text files are UTF-16LE with a BOM, and `.pak` `.txt` entries are read as UTF-16LE.
- `runtime-main` uses hard-coded 32-bit addresses and `thiscall` hooks tied to a specific game executable, not a generic launcher.
- Some UI is stale relative to the backend: the Banks editor is routed but bank commands are commented out, and buttons such as `test_creature_sync`, repackers, and creature copy creator invoke commands not present in `lib.rs`. Check the handler list before debugging an `invoke` failure.
- UI styling is mostly Mantine + CSS Modules. `src/index.css` has Tailwind 4/shadcn setup but is not imported by `main.tsx`, so Tailwind utility classes alone will not apply.
