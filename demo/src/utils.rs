//! Utility for composing Tailwind CSS classes with conditional logic.
//!
//! This module provides the [`cn!`] macro which combines the conditional class composition
//! capabilities of [`clsx`](https://crates.io/crates/clsx) with Tailwind's class conflict resolution via
//! [`tw_merge`](https://crates.io/crates/tailwind-fuse). It's particularly
//! useful in Leptos or other UI frameworks where class names need to be dynamically composed.
//!
//! # Overview
//!
//! - **Single-pass** – We rely on the `clsx!` macro under the hood for minimal allocations.
//! - **Flexible** – Accepts strings, booleans, arrays, hashmaps, numeric types, closures, etc.
//! - **Tailwind merging** – Later classes override earlier ones when conflicts exist (e.g., `p-2` vs `p-4`).
//!
//! # Examples
//!
//! ## Basic usage
//! ```rust
//! use leptos_shadcn::cn;
//!
//! let classes = cn!("flex", "items-center");
//! assert_eq!(classes, "flex items-center");
//! ```
//!
//! ## Conditional classes
//! ```rust
//! use leptos_shadcn::cn;
//!
//! let is_active = true;
//! let classes = cn!(
//!     "btn",
//!     (is_active, "btn-active"),
//!     {"hover:bg-blue-500": true}
//! );
//! // => "btn btn-active hover:bg-blue-500"
//! ```
//!
//! ## Numeric & Option usage
//! ```rust
//! use leptos_shadcn::cn;
//! // Suppose we have a numeric "z-index" or some param
//! let z_index = 10;
//! let classes = cn!("absolute", z_index, Some("top-0"));
//! assert_eq!(classes, "absolute 10 top-0");
//! ```
//!
//! ## With Leptos components
//! ```rust,ignore
//! use leptos::*;
//! use leptos_shadcn::cn;
//!
//! #[component]
//! pub fn Button(
//!     #[prop(default = false)] active: bool,
//!     #[prop(default = "")] class: &'static str,
//! ) -> impl IntoView {
//!     view! {
//!         <button
//!             class=cn!(
//!                 "btn",
//!                 (active, "btn-active"),
//!                 class
//!             )>
//!             // ...
//!         </button>
//!     }
//! }
//! ```
//!
//! ## Tailwind merging examples
//! ```rust
//! use leptos_shadcn::cn;
//!
//! // Later classes override earlier ones in conflicts
//! assert_eq!(cn!("p-2", "p-4"), "p-4");
//! assert_eq!(cn!("p-2", "px-4"), "p-2 px-4");
//! assert_eq!(cn!("grid-cols-2", "grid-cols-4"), "grid-cols-4");
//! ```
//!

use leptos::{ev, logging, web_sys};
use leptos::wasm_bindgen::JsCast;

// Re-export or reference your updated `clsx` crate under a known name.
// Replace `your_crate` with the actual path if it's local or from crates.io:

/// Combines conditional class composition (`clsx!`) with Tailwind class merging (`tw_merge!`).
///
/// The macro supports multiple input types for flexible class composition:
///
/// - **Strings**: `cn!("flex", "p-2")`
/// - **Conditional tuples**: `cn!((is_active, "active"))`
/// - **HashMaps / objects**: `cn!({"class1": true, "class2": false})`
/// - **Arrays / Vectors**: `cn!(["class1", "class2"])`
/// - **Option types**: `cn!(Some("class"), None::<&str>)`
/// - **Numeric types**: appended as raw strings
/// - **Closures** returning something that implements `ClsxArg` (e.g., `Option<&str>`)
///
/// After assembling all classes via [`clsx`][clsx_dep::clsx!],
/// it calls [`tw_merge`](https://crates.io/crates/tailwind-fuse) to resolve any Tailwind conflicts.
///
/// # Examples
///
/// ```rust,ignore
/// use leptos_shadcn::cn;
/// let is_active = true;
/// let classes = cn!("btn", (is_active, "btn-active"), "p-4");
/// assert_eq!(classes, "btn btn-active p-4");
/// ```
///
/// ```rust,ignore
/// use crate::cn;
/// // Tailwind conflict merging
/// assert_eq!(cn!("p-2", "p-4"), "p-4");
/// assert_eq!(cn!("p-2", "px-4"), "p-2 px-4");
/// ```
#[macro_export]
macro_rules! cn {
    ($($args:tt)*) => {{
        let composed = clsx::clsx!($($args)*);
        tailwind_fuse::tw_merge!(&composed)
    }};
}

// Optional debug-only click handler for demonstration
#[cfg(debug_assertions)]
pub const DEFAULT_CLICK_HANDLER: fn(ev::MouseEvent) = |ev| {
    let target_element = ev.target()
        .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
        .map(|el| el.tag_name().to_lowercase())
        .unwrap_or_else(|| "unknown".to_string());

    let target_classes = ev.target()
        .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
        .and_then(|el| el.get_attribute("class"))
        .unwrap_or_else(|| "no classes".to_string());

    logging::log!(
        "Click: {{\n  element: {},\n  classes: {},\n  x: {},\n  y: {}\n}}",
        target_element,
        target_classes,
        ev.client_x(),
        ev.client_y()
    );
};

#[cfg(not(debug_assertions))]
pub const DEFAULT_CLICK_HANDLER: fn(ev::MouseEvent) = |_| ();

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::cn;

    #[test]
    fn test_empty() {
        assert_eq!(cn!(), "");
    }

    #[test]
    fn test_basic_strings() {
        assert_eq!(cn!("flex", "p-2"), "flex p-2");
        assert_eq!(cn!(""), "");
        // Leading/trailing spaces get trimmed by tw_merge
        assert_eq!(cn!("  p-2  "), "p-2");
    }

    #[test]
    fn test_whitespace_handling() {
        // tw_merge also condenses multiple spaces
        assert_eq!(cn!("flex    p-2"), "p-2");
        assert_eq!(cn!("   "), "");
    }

    #[test]
    fn test_conditional_tuples() {
        assert_eq!(cn!((true, "active"), (false, "inactive")), "active");
        assert_eq!(cn!("base", (true, "active")), "base active");
        assert_eq!(cn!("base", (false, "active")), "base");
    }

    #[test]
    fn test_arrays_and_vecs() {
        assert_eq!(cn!(["flex", "p-2"]), "flex p-2");
        assert_eq!(cn!(vec!["flex", "p-2"]), "flex p-2");
        assert_eq!(cn!("base", ["flex", "p-2"]), "base flex p-2");
    }

    #[test]
    fn test_hashmap() {
        let mut classes = HashMap::new();
        classes.insert("flex".to_string(), true);
        classes.insert("hidden".to_string(), false);
        assert_eq!(cn!(classes), "flex");
    }

    #[test]
    fn test_option_types() {
        assert_eq!(cn!(Some("flex")), "flex");
        assert_eq!(cn!(None::<&str>), "");
        assert_eq!(cn!(Some("flex"), None::<&str>, "p-2"), "flex p-2");
    }

    #[test]
    fn test_numeric_types() {
        let z_index = 50;
        let spacing = 0;
        // numeric -> appended as "50" or "0"
        assert_eq!(cn!("absolute", z_index, spacing), "absolute 50 0");
        // tw_merge won't merge numeric strings with tailwind classes,
        // so each stands as a separate "class".
    }

    #[test]
    fn test_closures() {
        let is_active = true;
        assert_eq!(
            cn!("base", || if is_active { "active" } else { "" }),
            "base active"
        );
        let not_active = false;
        assert_eq!(
            cn!("base", || if not_active { "active" } else { "" }),
            "base"
        );
    }

    #[test]
    fn test_complex_composition() {
        let is_active = true;
        let size = Some("lg");
        let mut map = HashMap::new();
        map.insert("bg-red-500".to_string(), false);
        map.insert("shadow".to_string(), true);

        let out = cn!(
            "btn",
            (is_active, "btn-active"),
            map,
            size.map(|s| format!("btn-{}", s)),
            ["p-2", "rounded"],
        );
        // tw_merge merges "p-2" with no conflict, "btn-active" not conflicting with "btn-lg"
        // final string:
        assert_eq!(out, "btn btn-active shadow btn-lg p-2 rounded");
    }

    #[test]
    fn test_tailwind_merge_basics() {
        // "p-4" overrides "p-2"
        assert_eq!(cn!("p-2", "p-4"), "p-4");
        // "p-2" + "px-4" are distinct
        assert_eq!(cn!("p-2", "px-4"), "p-2 px-4");
    }

    #[test]
    fn test_tailwind_merge_with_modifiers() {
        // "hover:p-4" overrides "hover:p-2"
        assert_eq!(cn!("hover:p-2", "hover:p-4"), "hover:p-4");
        // "sm:p-4" overrides "sm:p-2"
        assert_eq!(cn!("sm:p-2", "sm:p-4", "md:p-6"), "sm:p-4 md:p-6");
    }

    #[test]
    fn test_tailwind_arbitrary_values() {
        // "p-[4px]" overrides "p-[2px]"
        assert_eq!(cn!("p-[2px]", "p-[4px]"), "p-[4px]");
        assert_eq!(
            cn!("grid-cols-[1fr,2fr]", "grid-cols-[1fr,1fr]"),
            "grid-cols-[1fr,1fr]"
        );
    }

    #[test]
    fn test_complex_tailwind_scenarios() {
        let is_active = true;
        let is_large = true;
        assert_eq!(
            cn!(
                "p-2",
                (is_active, "hover:bg-blue-500"),
                ["rounded-lg", "shadow-sm"],
                (is_large, "lg:p-4"),
                "dark:bg-gray-800"
            ),
            "p-2 hover:bg-blue-500 rounded-lg shadow-sm lg:p-4 dark:bg-gray-800"
        );
    }

    #[test]
    fn test_edge_cases() {
        // Empty values
        assert_eq!(cn!("", "p-2"), "p-2");
        assert_eq!(cn!("p-2", ""), "p-2");

        // Whitespace handling
        assert_eq!(cn!("  p-2  ", "  p-4  "), "p-4");
        assert_eq!(cn!("p-2    p-4"), "p-4");

        // Duplicate classes => last wins in tailwind_fuse
        assert_eq!(cn!("p-2 p-2", "p-2"), "p-2");
    }
}
