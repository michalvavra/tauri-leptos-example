# tauri-leptos-example

- [Tauri][tauri_web]
- [Leptos][leptos_repo]

Requires Rust Nightly. See Leptos [`nightly` Note][leptos_nightly_note].

```sh
# Install Tauri CLI
cargo install tauri-cli

# Build and develop for desktop
cargo tauri dev

# Build and release for desktop
cargo tauri build

```

## Credits

All credit for the counter example in [`./src-ui/src/lib.rs`](src-ui/src/lib.rs) goes to 
authors and contributors of [gbj/leptos][leptos_repo] GitHub repository, 
[MIT License][leptos_license], Copyright 2022 Greg Johnston.

[tauri_web]: https://tauri.app/
[leptos_repo]: https://github.com/gbj/leptos
[leptos_nightly_note]: https://github.com/gbj/leptos#nightly-note
[leptos_license]: https://github.com/gbj/leptos/blob/e465867b30db8fccce7493f9fc913359246ac4bd/LICENSE