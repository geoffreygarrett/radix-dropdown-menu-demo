use std::sync::atomic::{AtomicUsize, Ordering};
use leptos::prelude::*;

static COUNT: AtomicUsize = AtomicUsize::new(0);

/// Creates a reactive ID value that can be used in templates and signals.
/// Returns a ReadSignal to allow for reactive updates if needed.
///
/// # Arguments
/// * `deterministic_id` - Optional predefined ID. If None, generates a unique ID.
///
/// # Examples
/// ```
/// use radix_leptos_id::{use_id, use_id_with_deterministic_id};
/// let id = use_id_with_deterministic_id(None);
/// let id = use_id_with_deterministic_id(Some("custom-id".to_string()));
/// ```
pub fn use_id_with_deterministic_id(deterministic_id: Option<String>) -> ReadSignal<String> {
    let prefix = option_env!("RADIX_ID_PREFIX").unwrap_or("radix");

    let (id, _) = signal(
        deterministic_id.unwrap_or_else(|| {
            format!("{}-{}", prefix, COUNT.fetch_add(1, Ordering::Relaxed))
        })
    );

    id
}

/// Shorthand for `use_id_with_deterministic_id(None)`. Generates a unique, stable ID.
pub fn use_id() -> ReadSignal<String> {
    use_id_with_deterministic_id(None)
}