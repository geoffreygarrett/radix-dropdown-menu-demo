use leptos::*;
use web_sys::{window, HtmlElement, Event};
use crate::types::{Axis};
use std::cell::RefCell;
use std::rc::Rc;
use leptos::wasm_bindgen::JsCast;

/// Handles scroll events and determines if the event should be canceled.
///
/// # Arguments
///
/// * `axis` - The axis on which the scroll is occurring (`Axis::V` for vertical, `Axis::H` for horizontal).
/// * `end_target` - The HTML element that is the target of the scroll lock.
/// * `event` - The scroll event.
/// * `source_delta` - The delta value from the event.
/// * `no_overscroll` - Whether overscroll should be prevented.
///
/// # Returns
///
/// * `bool` - `true` if the scroll should be canceled, `false` otherwise.
pub fn handle_scroll(
    axis: Axis,
    end_target: &HtmlElement,
    event: &Event,
    source_delta: f64,
    no_overscroll: bool,
) -> bool {
    let window = window().unwrap();
    let computed_style = window.get_computed_style(end_target).unwrap().unwrap();

    let is_rtl = computed_style.get_property_value("direction").unwrap() == "rtl";
    let direction_factor = match axis {
        Axis::H => if is_rtl { -1.0 } else { 1.0 },
        Axis::V => 1.0,
    };

    let delta = direction_factor * source_delta;
    let mut current = Some(end_target.clone());
    let mut should_cancel = false;

    while let Some(element) = current {
        let styles = window.get_computed_style(&element).unwrap().unwrap();
        let overflow = match axis {
            Axis::V => styles.get_property_value("overflow-y").unwrap_or_default(),
            Axis::H => styles.get_property_value("overflow-x").unwrap_or_default(),
        };

        if overflow != "hidden" && overflow != "visible" {
            let (scroll_pos, scroll_size, client_size) = match axis {
                Axis::V => (
                    element.scroll_top() as f64,
                    element.scroll_height() as f64,
                    element.client_height() as f64,
                ),
                Axis::H => (
                    element.scroll_left() as f64,
                    element.scroll_width() as f64,
                    element.client_width() as f64,
                ),
            };

            if scroll_size > client_size {
                if (delta > 0.0 && scroll_pos + client_size >= scroll_size) ||
                    (delta < 0.0 && scroll_pos <= 0.0) {
                    should_cancel = true;
                }
                break;
            }
        }

        current = element.parent_element()
            .and_then(|e| e.dyn_into::<HtmlElement>().ok());
    }

    should_cancel
}

/// Calculates the width of the scrollbar to adjust padding accordingly.
///
/// # Errors
///
/// Returns a static string slice on failure.
pub fn calculate_scrollbar_width() -> Result<i32, &'static str> {
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
