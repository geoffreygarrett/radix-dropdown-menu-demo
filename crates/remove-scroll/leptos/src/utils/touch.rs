use web_sys::{TouchEvent, Touch};
pub use crate::types::TouchAction;
use std::collections::HashMap;

/// Tracks touch events to determine touch actions like move, pinch, or zoom.
pub struct TouchTracker {
    cache: HashMap<i32, Touch>,
}

impl TouchTracker {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn track_touch(&mut self, event: &TouchEvent) -> TouchAction {
        if event.touches().length() == 2 {
            // Handle pinch/zoom gestures
            let touch1 = event.touches().item(0).unwrap();
            let touch2 = event.touches().item(1).unwrap();

            if let (Some(old1), Some(old2)) = (
                self.cache.get(&(touch1.identifier())),
                self.cache.get(&(touch2.identifier())),
            ) {
                let dx1 = touch1.client_x() - old1.client_x();
                let dy1 = touch1.client_y() - old1.client_y();
                let dx2 = touch2.client_x() - old2.client_x();
                let dy2 = touch2.client_y() - old2.client_y();

                if ds(&[dx1 as f64, dx2 as f64]) || ds(&[dy1 as f64, dy2 as f64]) {
                    return TouchAction::Zoom;
                }

                let mx = dx1.abs().max(dx2.abs());
                let my = dy1.abs().max(dy2.abs());

                return TouchAction::Pinch {
                    delta_x: sign(dx1 as f64),
                    delta_y: sign(dy1 as f64),
                };
            }
        }

        // Update cache
        for i in 0..event.changed_touches().length() {
            if let Some(touch) = event.changed_touches().item(i) {
                self.cache.insert(touch.identifier(), touch);
            }
        }

        TouchAction::Move
    }
}

fn ds(ab: &[f64; 2]) -> bool {
    (ab[0] <= 0.0 && ab[1] >= 0.0) || (ab[0] >= 0.0 && ab[1] <= 0.0)
}

fn sign(x: f64) -> f64 {
    if x < 0.0 {
        -1.0
    } else {
        1.0
    }
}
