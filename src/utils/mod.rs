#[cfg(feature = "web")]
pub fn get_now() -> u128 {
    web_sys::js_sys::Date::now() as u128
}

#[cfg(feature = "desktop")]
pub fn get_now() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

#[cfg(feature = "web")]
pub fn nanoid() -> String {
    nanoid_wasm::nanoid!(21)
}

#[cfg(feature = "desktop")]
pub fn nanoid() -> String {
    nanoid::nanoid!()
}

#[cfg(feature = "desktop")]
pub async fn sleep(millis: i32) {
    use std::time::Duration;

    let Ok(millis) = u64::try_from(millis) else {
        return;
    };

    tokio::time::sleep(Duration::from_millis(millis)).await;
}

#[cfg(feature = "web")]
pub async fn sleep(millis: i32) {
    use web_sys::js_sys::Promise;
    use web_sys::js_sys::futures::JsFuture;

    let promise = Promise::new(&mut |resolve, _reject| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, millis)
            .unwrap();
    });

    JsFuture::from(promise).await.unwrap();
}
