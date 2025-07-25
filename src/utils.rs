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
                std::thread::spawn(move || {
                    std::thread::sleep(interval);
                    let _ = sender.send(());
                });
                let _ = receiver.await;
            }
            #[cfg(target_arch = "wasm32")]
            Self::Javascript => {
                use std::convert::TryInto;
                use wasm_bindgen_futures::JsFuture;

                JsFuture::from(web_sys::js_sys::Promise::new(&mut |yes, _| {
                    web_sys::window()
                        .unwrap()
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            &yes,
                            interval.as_millis().try_into().unwrap(),
                        )
                        .unwrap();
                }))
                .await
                .unwrap();
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
