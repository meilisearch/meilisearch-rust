use std::time::Duration;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) async fn async_sleep(interval: Duration) {
    let (sender, receiver) = futures::channel::oneshot::channel::<()>();
    std::thread::spawn(move || {
        std::thread::sleep(interval);
        let _ = sender.send(());
    });
    let _ = receiver.await;
}

#[cfg(target_arch = "wasm32")]
pub(crate) async fn async_sleep(interval: Duration) {
    use std::convert::TryInto;
    use wasm_bindgen_futures::JsFuture;

    JsFuture::from(js_sys::Promise::new(&mut |yes, _| {
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

#[cfg(test)]
mod test {
    use super::*;
    use meilisearch_test_macro::meilisearch_test;

    #[meilisearch_test]
    async fn test_async_sleep() {
        let sleep_duration = std::time::Duration::from_millis(10);
        let now = time::Instant::now();

        async_sleep(sleep_duration).await;

        assert!(now.elapsed() >= sleep_duration);
    }
}
