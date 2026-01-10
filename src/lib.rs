/*!
It provides simple `async_sleep()` that work well in both web browsers and native applications.
This can be used in the `dioxus` application.

## Howto use
Just call `async_sleep()` on the frontend or backend.

```rust
# async fn func() {
use async_sleep_aki::{async_sleep, Duration};
async_sleep(Duration::from_millis(100)).await;
# }
```

## Implementation
If `target` is `wasm32-unknown-unknown`, calls `gloo_timers::future::sleep()`, otherwise calls `tokio::time::sleep()`.
*/
pub use std::time::Duration;

#[cfg(not(all(
    target_arch = "wasm32",
    target_vendor = "unknown",
    target_os = "unknown"
)))]
pub async fn async_sleep(dur: Duration) {
    tokio::time::sleep(dur).await;
}

#[cfg(all(
    target_arch = "wasm32",
    target_vendor = "unknown",
    target_os = "unknown"
))]
pub async fn async_sleep(dur: Duration) {
    gloo_timers::future::sleep(dur).await;
}
