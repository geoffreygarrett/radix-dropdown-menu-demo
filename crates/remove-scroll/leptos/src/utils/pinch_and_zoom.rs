use web_sys::TouchEvent;
use crate::types::TouchAction;
use crate::utils::touch::TouchTracker;

/// Determines the action based on touch movements.
pub fn pinch_or_zoom(event: &TouchEvent, cache: &mut TouchTracker) -> TouchAction {
    cache.track_touch(event)
}
