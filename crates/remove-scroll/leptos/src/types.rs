use leptos::prelude::*;
use std::rc::Rc;
use leptos::ev;
use leptos_node_ref::AnyNodeRef;

#[derive(Clone, Debug)]
pub struct ScrollState {
    pub original_overflow: String,
    pub original_padding_right: String,
    pub scroll_position: (f64, f64),
}

#[derive(Clone)]
pub struct RemoveScrollContext {
    pub enabled: RwSignal<bool>,
    pub locks: RwSignal<i32>,
}

#[derive(Clone, Default)]
pub struct RemoveScrollEffectCallbacks {
    pub on_scroll_capture: Option<Rc<dyn Fn(ev::Event)>>,
    pub on_wheel_capture: Option<Rc<dyn Fn(ev::WheelEvent)>>,
    pub on_touch_move_capture: Option<Rc<dyn Fn(ev::TouchEvent)>>,
}

#[derive(Clone)]
pub struct RemoveScrollEffectProps {
    pub no_isolation: bool,
    pub remove_scroll_bar: bool,
    pub allow_pinch_zoom: bool,
    pub inert: bool,
    pub shards: Vec<AnyNodeRef>,
    pub lock_ref: AnyNodeRef,
    pub gap_mode: GapMode,
    pub set_callbacks: Option<WriteSignal<RemoveScrollEffectCallbacks>>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum GapMode {
    Margin,
    Padding,
}

impl Default for GapMode {
    fn default() -> Self {
        Self::Margin
    }
}

#[derive(Clone, PartialEq)]
pub enum Axis {
    V,
    H,
}

// #[derive(Clone)]
// pub struct RemoveScrollProps {
//     pub enabled: MaybeSignal<bool>,
//     pub allow_pinch_zoom: MaybeSignal<bool>,
//     pub no_isolation: MaybeSignal<bool>,
//     pub forward_props: MaybeSignal<bool>,
//     pub remove_scroll_bar: MaybeSignal<bool>,
//     pub inert: MaybeSignal<bool>,
//     pub gap_mode: Option<GapMode>,
//     pub shards: Option<Vec<NodeRef>>,
//     pub class: MaybeSignal<String>,
//     pub children: Children,
// }

// #[derive(Clone)]
// pub struct RemoveScrollUIProps {
//     pub props: RemoveScrollProps,
//     pub side_car: SideCarComponent,
// }

#[derive(Clone)]
pub enum TouchAction {
    Move,
    Pinch { delta_x: f64, delta_y: f64 },
    Zoom,
}
