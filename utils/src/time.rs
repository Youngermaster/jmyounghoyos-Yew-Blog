use chrono::prelude::*;
use std::time::{Duration, UNIX_EPOCH};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

pub fn format_timestamp(timestamp: u64, format_str: &'static str) -> String {
    let date = UNIX_EPOCH + Duration::from_millis(timestamp);
    let datetime = DateTime::<Utc>::from(date);

    datetime.format(format_str).to_string()
}

pub fn format_time_string(timestring: &str, format_str: &'static str) -> String {
    let datetime = DateTime::parse_from_rfc3339(timestring).unwrap();

    datetime.format(format_str).to_string()
}

pub fn set_timeout<F>(callback: F, timeout: i32)
where
    F: Fn() + 'static,
{
    let window = web_sys::window().unwrap();
    let callback = Closure::wrap(Box::new(callback) as Box<dyn Fn()>);

    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            timeout,
        )
        .unwrap();
}
