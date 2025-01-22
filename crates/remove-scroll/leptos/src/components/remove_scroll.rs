use crate::{
    sidecar::{SideCarComponent, effect_car},
    types::*,
};
use leptos::prelude::*;
use web_sys::Event;
use leptos::ev;
use leptos_node_ref::AnyNodeRef;

#[component]
#[allow(non_snake_case)]
pub fn RemoveScroll(
    /// Whether scroll removal is enabled
    #[prop(optional, into)]
    enabled: MaybeProp<bool>,
    /// Allow pinch-to-zoom gestures
    #[prop(optional, into)]
    allow_pinch_zoom: MaybeProp<bool>,
    /// Disable event isolation
    #[prop(optional, into)]
    no_isolation: MaybeProp<bool>,
    /// Forward props to children
    #[prop(optional, into)]
    forward_props: MaybeProp<bool>,
    /// Remove scrollbar
    #[prop(optional, into)]
    remove_scroll_bar: MaybeProp<bool>,
    /// Enable inert mode
    #[prop(optional, into)]
    inert: MaybeProp<bool>,
    /// Gap mode for scrollbar compensation
    #[prop(optional)]
    gap_mode: Option<GapMode>,
    /// Additional shards
    #[prop(optional)]
    shards: Option<Vec<AnyNodeRef>>,
    /// Child content
    children: TypedChildren<impl IntoView + 'static>,
    /// Optional className when forward_props is false
    #[prop(optional, into)]
    class_name: MaybeProp<String>,
) -> impl IntoView {
    // Set default values similar to React's defaultProps
    let is_enabled = RwSignal::new(enabled.get().unwrap_or(true));
    let remove_scroll_bar = remove_scroll_bar.get().unwrap_or(true);
    let inert = inert.get().unwrap_or(false);

    let lock_ref = AnyNodeRef::new();
    let locks = RwSignal::new(0);
    let callbacks = RwSignal::new(RemoveScrollEffectCallbacks::default());

    // Provide context
    provide_context(RemoveScrollContext {
        enabled: is_enabled.clone(),
        locks: locks.clone(),
    });

    // Prepare effect properties
    let effect_props = RemoveScrollEffectProps {
        no_isolation: no_isolation.get().unwrap_or(false),
        remove_scroll_bar,
        allow_pinch_zoom: allow_pinch_zoom.get().unwrap_or(false),
        inert,
        shards: shards.unwrap_or_default(),
        lock_ref: lock_ref.clone(),
        gap_mode: gap_mode.unwrap_or_default(),
        set_callbacks: Some(callbacks.write_only()),
    };

    // Conditionally create sidecar effect based on `enabled`
    let sidecar_view = move || {
        if is_enabled.get() {
            SideCarComponent::new(effect_car(), effect_props.clone()).into_view()
        } else {
            ().into_view()
        }
    };

    let prevent_event = move |e: Event| {
        if is_enabled.get() && !no_isolation.get() {
            e.prevent_default();
            e.stop_propagation();
        }
    };

    // Handle forward_props logic
    let content_view = move || {
        if forward_props.get().unwrap_or(false) {
            // let mut children_vec = children().nodes.clone();
            //
            // if children_vec.len() != 1 {
            //     panic!("RemoveScroll with forward_props requires exactly one child");
            // }
            //
            // let child = children_vec.pop().unwrap();

            children.into_inner()
                .attr("ref", lock_ref.clone())
                .on(ev::scroll, move |evt| {
                    if let Some(callback) = callbacks.get().on_scroll_capture {
                        callback(evt)
                    }
                })
                .on(ev::wheel, move |evt| {
                    if let Some(callback) = callbacks.get().on_wheel_capture {
                        callback(evt)
                    }
                })
                .on(ev::touchmove, move |evt| {
                    if let Some(callback) = callbacks.get().on_touch_move_capture {
                        callback(evt)
                    }
                })
                .into_view()
        } else {
            let class_name = class_name.get().unwrap_or_default();
            view! {
                <div on:touchmove=prevent_even>
                    // node_ref=lock_ref
                    // on:wheel=prevent_event
                    // on:touchmove=prevent_event
                    {(children.into_inner())()}
                </div>
            }
        }
    };

    view! { <>{sidecar_view()} {content_view()}</> }
}

// Expose class names for compatibility
pub mod class_names {
    pub const WRAPPER: &str = "remove-scroll-wrapper";
    pub const FULL_WIDTH: &str = "remove-scroll-full-width";
    pub const ZERO_RIGHT: &str = "remove-scroll-zero-right";
}
