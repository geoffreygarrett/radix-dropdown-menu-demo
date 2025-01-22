use js_sys::Object;
use leptos::prelude::window;
use leptos::wasm_bindgen::closure::Closure;
use leptos::wasm_bindgen::JsValue;
use leptos::wasm_bindgen::prelude::*;
use leptos::wasm_bindgen::JsCast;

/// Returns the appropriate options for non-passive event listeners.
pub fn non_passive() -> web_sys::AddEventListenerOptions {
    let mut options = web_sys::AddEventListenerOptions::new();
    if passive_supported() {
        options.set_passive(false);
    }
    options
}

/// Checks if passive event listeners are supported.
fn passive_supported() -> bool {
    static PASSIVE_SUPPORTED: once_cell::sync::OnceCell<bool> = once_cell::sync::OnceCell::new();
    PASSIVE_SUPPORTED.get_or_init(|| {
        let window = window();
        let mut supported = false;
        let closure = Closure::wrap(Box::new(move || {}) as Box<dyn FnMut()>);

        let options = Object::new();
        js_sys::Reflect::set(&options, &JsValue::from_str("passive"), &JsValue::TRUE).unwrap();

        let result = window.add_event_listener_with_callback_and_bool(
            "test",
            closure.as_ref().unchecked_ref(),
            true,
        );

        if result.is_ok() {
            supported = true;
        }

        window.add_event_listener_with_callback_and_bool(
            "test",
            closure.as_ref().unchecked_ref(),
            true,
        ).unwrap_or(());

        closure.forget(); // Prevent garbage collection

        supported
    }).clone()
}
