/*!
It provides simple `async_sleep()` that work well in both web browsers and native applications.
This can be used in the `dioxus` application.

## Howto use
Just call `async_sleep()` on the frontend or backend.

```rust
# use async_sleep_aki::async_sleep;
# async fn func() {
async_sleep(100).await;
# }
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

```rust
use dioxus::prelude::*;
use async_sleep_aki::postponed_call;

#[component]
fn func() -> Element {
    let mut postponed = use_signal(|| postponed_call(10, move || {}));
    let mut is_loading = use_signal(|| false);
    let rsrc = use_resource(move || async move {
        let t = postponed_call(2000, move || {
            if *is_loading.read() {
                is_loading.set(false);
            }
        });
        let _ = postponed.replace(t);
    });
    rsx!{ div{} }
}
```

## Implementation
If `target` is `wasm32-unknown-unknown`, calls `gloo_timers::future::sleep()`, otherwise calls `tokio::time::sleep()`.
*/

/// Stops processing for `delay` milliseconds.
///
/// Other asynchronous functions will be processed while it is stopped.
///
/// If `delay` is a negative number, it is treated as `0` seconds.
///
/// In a web browser, this is implemented using the `setTimeout()` function of `javascript`, so it is subject to [the same restrictions].
///
/// [the same restrictions]: https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout#maximum_delay_value
pub async fn async_sleep(delay: i32) {
    let delay = if delay >= 0 { delay as u64 } else { 0 };
    let dur = std::time::Duration::from_millis(delay);

    // for non browser
    #[cfg(not(all(
        target_arch = "wasm32",
        target_vendor = "unknown",
        target_os = "unknown"
    )))]
    tokio::time::sleep(dur).await;

    // for browser
    #[cfg(all(
        target_arch = "wasm32",
        target_vendor = "unknown",
        target_os = "unknown"
    ))]
    gloo_timers::future::sleep(dur).await;
}

/// Pause processing for `delay` milliseconds and then call the argument async function.
pub async fn delayed_call<F>(delay: i32, f: F)
where
    F: std::future::Future<Output = ()> + 'static,
{
    async_sleep(delay).await;
    f.await;
}

#[cfg(feature = "dx")]
pub struct PostponedCall {
    // for non browser
    #[cfg(not(all(
        target_arch = "wasm32",
        target_vendor = "unknown",
        target_os = "unknown"
    )))]
    _a: dioxus_core::Task,

    // for browser
    #[cfg(all(
        target_arch = "wasm32",
        target_vendor = "unknown",
        target_os = "unknown"
    ))]
    _a: gloo_timers::callback::Timeout,
}

#[cfg(feature = "dx")]
// for non browser
#[cfg(not(all(
    target_arch = "wasm32",
    target_vendor = "unknown",
    target_os = "unknown"
)))]
impl Drop for PostponedCall {
    fn drop(&mut self) {
        let r = self._a;
        self._a = dioxus_core::spawn(async move {});
        r.cancel();
    }
}

/// Create a Task that will call the `callback` after `millis` milliseconds.
///
/// When dropped, the task is canceled.
///
/// If `millis` is a negative number, it is treated as `0` seconds.
///
/// In a web browser, this is implemented using the `setTimeout()` function of `javascript`, so it is subject to [the same restrictions].
///
/// [the same restrictions]: https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout#maximum_delay_value
#[cfg(feature = "dx")]
pub fn postponed_call<F>(millis: i32, callback: F) -> PostponedCall
where
    F: 'static + FnOnce(),
{
    let delay = if millis >= 0 { millis as u64 } else { 0 };

    // for non browser
    #[cfg(not(all(
        target_arch = "wasm32",
        target_vendor = "unknown",
        target_os = "unknown"
    )))]
    {
        let dur = std::time::Duration::from_millis(delay);
        let a = dioxus_core::spawn(async move {
            tokio::time::sleep(dur).await;
            callback();
        });
        PostponedCall { _a: a }
    }

    // for browser
    #[cfg(all(
        target_arch = "wasm32",
        target_vendor = "unknown",
        target_os = "unknown"
    ))]
    {
        let a = gloo_timers::callback::Timeout::new(millis as u32, callback);
        PostponedCall { _a: a }
    }
}
