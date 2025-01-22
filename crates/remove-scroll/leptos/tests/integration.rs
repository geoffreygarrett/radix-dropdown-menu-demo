use js_sys::wasm_bindgen::JsCast;
use leptos::{mount::mount_to, prelude::*};
use leptos_remove_scroll::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

async fn tick() {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 0)
            .unwrap();
    });
    JsFuture::from(promise).await.unwrap();
}

fn setup_test() -> (web_sys::Document, web_sys::Element) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let test_container = document.create_element("div").unwrap();
    test_container.set_id("test-container");

    if document.get_element_by_id("test-container").is_none() {
        if let Some(body) = document.body() {
            let _ = body.append_child(&test_container);
        }
    }

    (document, test_container)
}

fn cleanup_test(document: &web_sys::Document) {
    if let Some(container) = document.get_element_by_id("test-container") {
        if let Some(parent) = container.parent_node() {
            let _ = parent.remove_child(&container);
        }
    }
}

#[wasm_bindgen_test]
async fn test_remove_scroll_basic_rendering() {
    let (document, test_container) = setup_test();

    let _dispose = mount_to(
        test_container.clone().unchecked_into(),
        || view! {
            <RemoveScroll>
                <div class="test-content">"Test Content"</div>
            </RemoveScroll>
        },
    );

    tick().await;

    let wrapper = test_container.first_element_child().unwrap();
    assert!(wrapper.class_list().contains(class_names::WRAPPER));
    assert!(wrapper.inner_html().contains("Test Content"));

    cleanup_test(&document);
}

#[wasm_bindgen_test]
async fn test_scroll_restoration() {
    let (document, test_container) = setup_test();
    let body = document.body().unwrap();

    body.style().set_property("overflow", "auto").unwrap();
    body.style().set_property("padding-right", "20px").unwrap();

    let _dispose = mount_to(
        test_container.clone().unchecked_into(),
        || view! {
            <RemoveScroll enabled=true>
                <div>"Test Content"</div>
            </RemoveScroll>
        },
    );

    tick().await;
    assert_eq!(body.style().get_property_value("overflow").unwrap(), "hidden");

    drop(_dispose);
    tick().await;

    assert_eq!(body.style().get_property_value("overflow").unwrap(), "auto");
    assert_eq!(body.style().get_property_value("padding-right").unwrap(), "20px");

    cleanup_test(&document);
}

#[wasm_bindgen_test]
async fn test_event_handling() {
    let (document, test_container) = setup_test();

    let _dispose = mount_to(
        test_container.clone().unchecked_into(),
        || view! {
            <RemoveScroll enabled=true no_isolation=false>
                <div id="test-content">"Test Content"</div>
            </RemoveScroll>
        },
    );

    tick().await;

    let wrapper = test_container.first_element_child().unwrap();

    let event_init = web_sys::EventInit::new();
    event_init.set_bubbles(true);
    event_init.set_cancelable(true);
    let wheel_event = web_sys::Event::new_with_event_init_dict("wheel", &event_init).unwrap();

    let result = wrapper.dispatch_event(&wheel_event).unwrap();
    assert!(!result, "Wheel event should be prevented");

    let touch_event = web_sys::Event::new_with_event_init_dict("touchmove", &event_init).unwrap();
    let result = wrapper.dispatch_event(&touch_event).unwrap();
    assert!(!result, "Touch event should be prevented");

    cleanup_test(&document);
}