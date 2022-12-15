# tauri-leptos-example

- [Tauri][tauri_web]
- [Leptos][leptos_repo]

See [Prerequisites](#prerequisites) section.

Requires Rust nightly. See Leptos [`nightly` note][leptos_nightly_note].

```sh
# Build and develop for desktop
cargo tauri dev

# Build and release for desktop
cargo tauri build
```

## Prerequisites

```sh
# Tauri CLI
cargo install --locked tauri-cli

# Rust nightly (required by Leptos)
rustup toolchain install nightly --allow-downgrade

# WASM target
rustup target add wasm32-unknown-unknown

# Trunk WASM bundler
cargo install --locked trunk

# `wasm-bindgen` for Apple M1 chips (required by Trunk)
cargo install --locked wasm-bindgen-cli

# `esbuild` as dependency of `tauri-sys` crate (used in UI)
npm install --global --save-exact esbuild
```

## Credits

All credit for the counter example in [`./src-ui/src/lib.rs`](src-ui/src/lib.rs) goes to 
authors and contributors of [gbj/leptos][leptos_repo] GitHub repository, 
[MIT License][leptos_license], Copyright 2022 Greg Johnston.

[tauri_web]: https://tauri.app/
[leptos_repo]: https://github.com/gbj/leptos
[leptos_nightly_note]: https://github.com/gbj/leptos#nightly-note
[leptos_license]: https://github.com/gbj/leptos/blob/e465867b30db8fccce7493f9fc913359246ac4bd/LICENSE