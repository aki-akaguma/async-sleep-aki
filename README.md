# async-sleep-aki

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Rust Version][rustc-image]
![Apache2/MIT licensed][license-image]
[![Test ubu][test-ubuntu-image]][test-ubuntu-link]
[![Test mac][test-windows-image]][test-windows-link]
[![Test win][test-macos-image]][test-macos-link]

It provides simple `async_sleep()` that work well in both web browsers and native applications.
This can be used in the `dioxus` application.

### Howto use
Just call `async_sleep()` on the frontend or backend.

```rust
async_sleep(100).await;
```

In dioxus component:

```rust
use dioxus::prelude::*;
use async_sleep_aki::delayed_call;

#[component]
fn func() -> Element {
    let mut is_loading = use_signal(|| false);
    use_effect(move ||{
        spawn(delayed_call(2000, async move {
            if *is_loading.read() {
                is_loading.set(false);
            }
        }));
    });
    rsx!{ div{} }
}
```

### Implementation
If `target` is `wasm32-unknown-unknown`, calls `gloo_timers::future::sleep()`, otherwise calls `tokio::time::sleep()`.

# Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/async-sleep-aki/blob/main/CHANGELOG.md)

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/async-sleep-aki.svg
[crate-link]: https://crates.io/crates/async-sleep-aki
[docs-image]: https://docs.rs/async-sleep-aki/badge.svg
[docs-link]: https://docs.rs/async-sleep-aki/
[rustc-image]: https://img.shields.io/badge/rustc-1.90+-blue.svg
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[test-ubuntu-image]: https://github.com/aki-akaguma/async-sleep-aki/actions/workflows/test-ubuntu.yml/badge.svg
[test-ubuntu-link]: https://github.com/aki-akaguma/async-sleep-aki/actions/workflows/test-ubuntu.yml
[test-macos-image]: https://github.com/aki-akaguma/async-sleep-aki/actions/workflows/test-macos.yml/badge.svg
[test-macos-link]: https://github.com/aki-akaguma/async-sleep-aki/actions/workflows/test-macos.yml
[test-windows-image]: https://github.com/aki-akaguma/async-sleep-aki/actions/workflows/test-windows.yml/badge.svg
[test-windows-link]: https://github.com/aki-akaguma/async-sleep-aki/actions/workflows/test-windows.yml
