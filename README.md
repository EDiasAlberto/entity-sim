 # Entity Sim

 A small simulation project with a Rust backend for state processing and a Python/Pygame frontend for rendering.

 This repository contains two main components:

 - `state-processor` — a Rust crate (PyO3 extension) that generates terrain, entities, and game state. It is built into a Python extension wheel using `maturin`.
 - `game-renderer` — a Python frontend that imports the compiled `state_processor` extension and renders the terrain and entities using `pygame`.

 ## Repository layout

 - `state-processor/` — Rust crate, exposes Python-callable functions via PyO3. Build artifacts (wheels) are placed in `state-processor/target/wheels/`.
	 - `src/` — Rust source (includes `core` module for terrain, entity management, and time management).
 - `game-renderer/` — Python code that renders the simulation using `pygame`.
 - `compile_lib_and_install.sh` — helper script that automates building the Rust crate and installing the generated wheel into the Python venv used by the renderer.

 ## High level overview

 The Rust crate exposes these Python-callable functions:

 - `run_terrain_gen(width: u16, height: u16, depth: u8) -> Terrain` — quickly generate a terrain object.
 - `generate_game_state(map_size: (u16, u16, u8), spawn_zone: (u16, u16, u16, u16)) -> GameState` — creates a `GameState` that contains a `terrain_map`, `entity_mgmt`, and `time_mgmt`.

 The `Terrain` type provides a method `get_map_data()` that returns (materials, heights) as NumPy arrays suitable for rendering. `EntityMgmt` exposes entity locations so the renderer can draw them.

 ## Requirements

 - Rust toolchain (stable; edition 2024 is used in the crate)
 - Python 3.13 
 - maturin 
 - Python packages for the frontend: `numpy`, `pygame` (install into `game-renderer/venv` or your chosen venv)

 Note: The wheel you build for `state-processor` must be built with the same Python interpreter version that you plan to run `game-renderer` with.

 ## Useful file pointers

 - `state-processor/src/lib.rs` — PyO3 glue (exports `run_terrain_gen` and `generate_game_state`).
 - `state-processor/src/core/terrain.rs` — terrain generation and NumPy interop.
 - `state-processor/src/core/entity_management.rs` — entity structures and management API.
 - `game-renderer/main.py` — minimal pygame renderer example.

 ## License

MIT
