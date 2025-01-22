//! # RemoveScroll
//!
//! `RemoveScroll` is a Leptos component designed to disable scrolling on the `<body>` element.
//! It compensates for scrollbar removal to prevent layout shifts and manages nested scroll locks.
//! This component serves as a Rust-based replacement for the `react-remove-scroll` library.
//!
//! ## Features
//! - **Scroll State Management:** Captures and restores the original scroll state (`overflow`, `padding-right`, and scroll positions).
//! - **Event Handling:** Prevents `wheel` and `touchmove` events to disable scrolling.
//! - **Scrollbar Compensation:** Calculates scrollbar width to adjust padding and prevent layout shifts.
//! - **Context Provision:** Allows nested components to share scroll lock state.
//! - **Cleanup Mechanism:** Ensures scroll is restored when no more locks are present.
//!
//! ## Usage
//! ```rust
//! use leptos::prelude::*;
//! use leptos_remove_scroll::RemoveScroll;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <RemoveScroll enabled=true allow_pinch_zoom=false>
//!             {/* Your content here */}
//!         </RemoveScroll>
//!     }
//! }
//! ```

mod types;
mod utils;
mod sidecar;
mod components;

use leptos::{prelude::*, ev::{TouchEvent, WheelEvent}, wasm_bindgen::JsCast, ev};
use leptos::ev::on;
use web_sys::{window, HtmlElement};
use leptos_typed_fallback_show::TypedFallbackShow;

/// Represents the state of the scroll before it was disabled.
#[derive(Clone)]
struct ScrollState {
    original_overflow: String,
    original_padding_right: String,
    scroll_position: (f64, f64),
}

/// Context to manage scroll locking across nested components.
#[derive(Clone)]
struct RemoveScrollContext {
    enabled: RwSignal<bool>,
    locks: RwSignal<i32>,
}

#[component]
#[allow(non_snake_case)]
pub fn RemoveScroll<C: IntoView + 'static>(
    children: TypedChildrenFn<C>,
    /// Determines if scroll removal is enabled
    #[prop(optional, into, default = MaybeProp::from(true))]
    enabled: MaybeProp<bool>,
    /// Allows pinch-to-zoom when scroll is removed
    #[prop(optional, into, default = MaybeProp::from(false))]
    allow_pinch_zoom: MaybeProp<bool>,
    /// Prevents event isolation when scroll is removed
    #[prop(optional, into, default = MaybeProp::from(false))]
    no_isolation: MaybeProp<bool>,
    /// Forwards properties to children without wrapping
    #[prop(optional, into, default = MaybeProp::from(false))]
    forward_props: MaybeProp<bool>,
) -> impl IntoView {
    // Initialize signals for managing locks and enabled state
    let children = StoredValue::new(children.into_inner());
    let locks = RwSignal::new(0);
    let is_enabled = RwSignal::new(enabled.get().unwrap_or(true));

    // Reactive signal to store the scroll state
    let scroll_state = RwSignal::new(None::<ScrollState>);

    // Provide context to allow nested components to share scroll lock state
    provide_context(RemoveScrollContext {
        enabled: is_enabled.clone(),
        locks: locks.clone(),
    });

    // Effect to handle enabling scroll removal
    Effect::new(move |_| {
        if is_enabled.get() {
            locks.update(|count| *count += 1);
            if locks.get() == 1 {
                if let Err(e) = disable_scroll(scroll_state.clone(), allow_pinch_zoom.get().unwrap_or(false)) {
                    leptos::logging::error!("Failed to disable scroll: {:?}", e);
                }
            }
        }
    });

    // Register cleanup to be called when the component is unmounted
    Owner::on_cleanup(move || {
        locks.update(|count| *count -= 1);
        if locks.get() <= 0 {
            if let Err(e) = restore_scroll(scroll_state.clone()) {
                leptos::logging::error!("Failed to restore scroll: {:?}", e);
            }
        }
    });

    // Event handler to prevent wheel events
    let prevent_wheel = move |e: WheelEvent| {
        if is_enabled.get() && !no_isolation.get().unwrap_or(false) {
            e.prevent_default();
            e.stop_propagation();
        }
    };

    // Event handler to prevent touchmove events
    let prevent_touch = move |e: TouchEvent| {
        if is_enabled.get() && !no_isolation.get().unwrap_or(false) {
            e.prevent_default();
            e.stop_propagation();
        }
    };

    view! {
        <TypedFallbackShow
            on:wheel=prevent_wheel
            on:touchmove=prevent_touch
            class:remove-scroll-wrapper=true
            when=move || forward_props.get().unwrap_or_default()
            fallback=move || children.with_value(|children| children())
        >
            <div>{children.with_value(|children| children())}</div>
        </TypedFallbackShow>
    }
}

/// Disables scrolling by modifying the document's styles.
///
/// # Arguments
///
/// * `state` - Reactive signal to store the original scroll configuration.
/// * `allow_pinch_zoom` - Flag to allow pinch-to-zoom gestures.
///
/// # Errors
///
/// Returns a static string slice on failure.
fn disable_scroll(
    state: RwSignal<Option<ScrollState>>,
    allow_pinch_zoom: bool,
) -> Result<(), &'static str> {
    let window = window().ok_or("No global `window` exists")?;
    let document = window.document().ok_or("No document found")?;
    let body = document.body().ok_or("No body element found")?;
    let document_element = document.document_element().ok_or("No document element found")?;

    // Retrieve and store current scroll state
    let computed_style = window
        .get_computed_style(&body)
        .map_err(|_| "Failed to get computed style")?
        .ok_or("Computed style is none")?;

    let scroll_state = ScrollState {
        original_overflow: computed_style
            .get_property_value("overflow")
            .map_err(|_| "Failed to get overflow property")?,
        original_padding_right: computed_style
            .get_property_value("padding-right")
            .map_err(|_| "Failed to get padding-right property")?,
        scroll_position: (
            window.scroll_x().unwrap_or(0.0),
            window.scroll_y().unwrap_or(0.0),
        ),
    };

    // Apply styles to disable scrolling
    body.style()
        .set_property("overflow", "hidden")
        .map_err(|_| "Failed to set overflow")?;

    // Calculate scrollbar width to prevent layout shift
    let scrollbar_width = calculate_scrollbar_width()?;
    body.style()
        .set_property("padding-right", &format!("{}px", scrollbar_width))
        .map_err(|_| "Failed to set padding-right")?;

    // Optionally disable pinch-to-zoom
    if !allow_pinch_zoom {
        if let Some(element) = document_element.dyn_ref::<HtmlElement>() {
            element
                .style()
                .set_property("touch-action", "none")
                .map_err(|_| "Failed to set touch-action")?;
        }
    }

    // Store the scroll state for restoration
    state.set(Some(scroll_state));
    Ok(())
}

/// Restores the scroll state to its original configuration.
fn restore_scroll(state: RwSignal<Option<ScrollState>>) -> Result<(), &'static str> {
    let window = window().ok_or("No global `window` exists")?;
    let document = window.document().ok_or("No document found")?;
    let body = document.body().ok_or("No body element found")?;
    let document_element = document.document_element().ok_or("No document element found")?;

    if let Some(scroll_state) = state.get() {
        // Restore original overflow and padding
        body.style()
            .set_property("overflow", &scroll_state.original_overflow)
            .map_err(|_| "Failed to restore overflow")?;
        body.style()
            .set_property("padding-right", &scroll_state.original_padding_right)
            .map_err(|_| "Failed to restore padding-right")?;

        // Restore scroll position
        window.scroll_to_with_x_and_y(
            scroll_state.scroll_position.0,
            scroll_state.scroll_position.1,
        );

        // Remove touch-action style
        if let Some(element) = document_element.dyn_ref::<HtmlElement>() {
            element
                .style()
                .remove_property("touch-action")
                .map_err(|_| "Failed to remove touch-action")?;
        }

        // Clear the scroll state
        state.set(None);
    }
    Ok(())
}

/// Calculates the width of the scrollbar to adjust padding accordingly.
///
/// # Errors
///
/// Returns a static string slice on failure.
fn calculate_scrollbar_width() -> Result<i32, &'static str> {
    let window = window().ok_or("No global `window` exists")?;
    let document = window.document().ok_or("No document found")?;

    // Create outer div with scrollbar
    let outer = document
        .create_element("div")
        .map_err(|_| "Failed to create outer div")?;
    outer
        .set_attribute(
            "style",
            "visibility: hidden; width: 100px; overflow: scroll; position: absolute; top: -9999px;",
        )
        .map_err(|_| "Failed to set outer div style")?;

    // Create inner div to calculate scrollbar width
    let inner = document
        .create_element("div")
        .map_err(|_| "Failed to create inner div")?;
    inner
        .set_attribute("style", "width: 100%")
        .map_err(|_| "Failed to set inner div style")?;
    outer
        .append_child(&inner)
        .map_err(|_| "Failed to append inner div to outer div")?;

    // Append to body to render and measure
    let body = document.body().ok_or("No body element found")?;
    body.append_child(&outer)
        .map_err(|_| "Failed to append outer div to body")?;

    // Calculate scrollbar width
    let scrollbar_width = outer
        .dyn_ref::<HtmlElement>()
        .and_then(|el| {
            inner
                .dyn_ref::<HtmlElement>()
                .map(|i| el.client_width() - i.client_width())
        })
        .ok_or("Failed to calculate scrollbar width")?;

    // Remove the temporary elements
    body.remove_child(&outer)
        .map_err(|_| "Failed to remove outer div from body")?;

    Ok(scrollbar_width)
}

/// Utility class names for managing layout adjustments.
pub mod class_names {
    /// Class for the wrapper element to manage scroll removal.
    pub const WRAPPER: &str = "remove-scroll-wrapper";

    /// Class for full-width elements to compensate for scrollbar removal.
    pub const FULL_WIDTH: &str = "remove-scroll-full-width";

    /// Class to set zero right padding.
    pub const ZERO_RIGHT: &str = "remove-scroll-zero-right";
}
