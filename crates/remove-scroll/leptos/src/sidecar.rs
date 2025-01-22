use std::rc::Rc;
use leptos::*;
use leptos::prelude::WriteSignal;
use crate::types::{GapMode, RemoveScrollEffectProps};
use web_sys::{Event, HtmlElement, WheelEvent, TouchEvent};
use leptos_node_ref::AnyNodeRef;
use crate::utils::{handle_scroll, TouchTracker, TouchAction};
use crate::utils::aggressive_capture::non_passive;
use crate::utils::touch::TouchTracker as TouchTrackerUtil;
use crate::types::Axis;

pub struct SideCarComponent {
    medium: Rc<dyn Fn() -> RemoveScrollEffectProps>,
    props: RemoveScrollEffectProps,
}

impl SideCarComponent {
    pub fn new(
        medium: Rc<dyn Fn() -> RemoveScrollEffectProps>,
        props: RemoveScrollEffectProps,
    ) -> Self {
        Self { medium, props }
    }
}

pub fn effect_car() -> Rc<dyn Fn() -> RemoveScrollEffectProps> {
    Rc::new(|| RemoveScrollEffectProps {
        no_isolation: false,
        remove_scroll_bar: true,
        allow_pinch_zoom: false,
        inert: false,
        shards: Vec::new(),
        lock_ref: AnyNodeRef::new(),
        gap_mode: GapMode::Padding,
        set_callbacks: None,
    })
}
