use std::time::Duration;

#[derive(Debug, Copy, Clone)]
pub(crate) enum SleepBackend {
    #[cfg(all(not(target_arch = "wasm32"), feature = "reqwest"))]
    Tokio,
    #[cfg(not(target_arch = "wasm32"))]
    Thread,
    #[cfg(target_arch = "wasm32")]
    Javascript,
}

impl SleepBackend {
    pub(crate) fn infer(is_tokio: bool) -> Self {
        #[cfg(all(not(target_arch = "wasm32"), feature = "reqwest"))]
        if is_tokio {
            return Self::Tokio;
        }
        #[cfg(any(target_arch = "wasm32", not(feature = "reqwest")))]
        let _ = is_tokio;

        #[cfg(not(target_arch = "wasm32"))]
        return Self::Thread;

        #[cfg(target_arch = "wasm32")]
        return Self::Javascript;
    }

    pub(crate) async fn sleep(self, interval: Duration) {
        match self {
            #[cfg(all(not(target_arch = "wasm32"), feature = "reqwest"))]
            Self::Tokio => {
                tokio::time::sleep(interval).await;
            }
            #[cfg(not(target_arch = "wasm32"))]
            Self::Thread => {
                let (sender, receiver) = futures_channel::oneshot::channel::<()>();
                let (cancel, cancellation) = std::sync::mpsc::channel::<()>();
                std::thread::spawn(move || {
                    if cancellation.recv_timeout(interval)
                        == Err(std::sync::mpsc::RecvTimeoutError::Timeout)
                    {
                        let _ = sender.send(());
                    }
                });
                // Keep the sender alive while waiting. Dropping it wakes the
                // timer thread if the sleep future is canceled.
                let _cancel = cancel;
                let _ = receiver.await;
            }
            #[cfg(target_arch = "wasm32")]
            Self::Javascript => {
                use wasm_bindgen_futures::JsFuture;
                use web_sys::Window;

                struct Timer<'a> {
                    window: &'a Window,
                    id: i32,
                }
                impl Drop for Timer<'_> {
                    fn drop(&mut self) {
                        self.window.clear_timeout_with_handle(self.id);
                    }
                }

                let window = web_sys::window().unwrap();
                let mut remaining = interval;
                loop {
                    let delay = remaining.min(Duration::from_millis(i32::MAX as u64));
                    let mut timer = None;
                    let promise = web_sys::js_sys::Promise::new(&mut |resolve, _| {
                        let id = window
                            .set_timeout_with_callback_and_timeout_and_arguments_0(
                                &resolve,
                                delay.as_millis() as i32,
                            )
                            .unwrap();
                        timer = Some(Timer {
                            window: &window,
                            id,
                        });
                    });
                    let _timer = timer;
                    JsFuture::from(promise).await.unwrap();
                    remaining -= delay;
                    if remaining.is_zero() {
                        break;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use meilisearch_test_macro::meilisearch_test;

    #[cfg(all(not(target_arch = "wasm32"), feature = "reqwest"))]
    #[meilisearch_test]
    async fn sleep_tokio() {
        let sleep_duration = Duration::from_millis(10);
        let now = std::time::Instant::now();

        SleepBackend::Tokio.sleep(sleep_duration).await;

        assert!(now.elapsed() >= sleep_duration);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[meilisearch_test]
    async fn sleep_thread() {
        let sleep_duration = Duration::from_millis(10);
        let now = std::time::Instant::now();

        SleepBackend::Thread.sleep(sleep_duration).await;

        assert!(now.elapsed() >= sleep_duration);
    }

    #[cfg(target_arch = "wasm32")]
    #[meilisearch_test]
    async fn sleep_javascript() {
        let sleep_duration = Duration::from_millis(10);
        let now = std::time::Instant::now();

        SleepBackend::Javascript.sleep(sleep_duration).await;

        assert!(now.elapsed() >= sleep_duration);
    }
}
