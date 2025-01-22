use leptos::{ev, ev::{animationcancel, animationend, animationstart}, html, logging, prelude::*};
use leptos::wasm_bindgen::JsCast;
use leptos_use::use_event_listener;
use web_sys::HtmlDivElement;
use leptos_node_ref::prelude::*;
use leptos_typed_fallback_show::TypedFallbackShow;

/// States representing the mounting cycle of a component
#[derive(Debug, Clone, PartialEq)]
enum PresenceState {
    Mounted,
    UnmountSuspended,
    Unmounted,
}

#[derive(Debug, Clone, Copy)]
enum PresenceEvent {
    Mount,
    Unmount,
    AnimationOut,
    AnimationEnd,
}

impl PresenceState {
    fn transition(&self, event: PresenceEvent) -> Option<Self> {
        #[cfg(debug_assertions)]
        logging::log!("TRANSITION - From {:?} with event {:?}", self, event);

        let result = match (self, event) {
            (Self::Mounted, PresenceEvent::Unmount) => Some(Self::Unmounted),
            (Self::Mounted, PresenceEvent::AnimationOut) => Some(Self::UnmountSuspended),
            (Self::Unmounted, PresenceEvent::Mount) => Some(Self::Mounted),
            (Self::UnmountSuspended, PresenceEvent::AnimationEnd) => Some(Self::Unmounted),
            (Self::UnmountSuspended, PresenceEvent::Mount) => Some(Self::Mounted),
            _ => None,
        };

        #[cfg(debug_assertions)]
        if let Some(ref new_state) = result {
            logging::log!("TRANSITION - To {:?}", new_state);
        }

        result
    }
}

fn get_animation_name_recursive(element: &web_sys::Element) -> Option<String> {
    if let Some(name) = get_animation_name(element) {
        return Some(name);
    }

    let children = element.children();
    for i in 0..children.length() {
        if let Some(child) = children.item(i) {
            if let Some(name) = get_animation_name_recursive(&child) {
                return Some(name);
            }
        }
    }
    None
}

fn get_animation_name(element: &web_sys::Element) -> Option<String> {
    window()
        .get_computed_style(element)
        .ok()
        .flatten()
        .and_then(|s| s.get_property_value("animation-name").ok())
        .and_then(|name| {
            let name = name.trim();
            if name.is_empty() || name == "none" {
                None
            } else {
                Some(name.to_string())
            }
        })
}

/// A component that handles mounting/unmounting of children with animation support.
/// Tracks animation state to ensure proper cleanup after exit animations complete.
#[component]
pub fn Presence<C: IntoView + 'static>(
    #[prop(into)]
    present: Signal<bool>,
    children: TypedChildrenFn<C>,
) -> impl IntoView {
    let node_ref = NodeRef::<html::Div>::new();
    // let is_present = create_presence(present, node_ref);
    let children = StoredValue::new(children.into_inner());

    let (prev_present, set_prev_present) = signal(present.get_untracked());
    let (state, set_state) = signal(if present.get_untracked() {
        PresenceState::Mounted
    } else {
        PresenceState::Unmounted
    });
    let (active_animations, set_active_animations) = signal(0);
    let (is_unmounting, set_is_unmounting) = signal(false);
    let (exit_animation_started, set_exit_animation_started) = signal(false);

    let send = move |event: PresenceEvent| {
        if let Some(new_state) = state.get_untracked().transition(event) {
            set_state.set(new_state);
        }
    };

    Effect::new(move |_| {
        let Some(el) = node_ref.get().and_then(|n: HtmlDivElement| n.dyn_into::<web_sys::HtmlElement>().ok()) else {
            return;
        };

        let was_present = prev_present.get();
        let now_present = present.get();

        if was_present == now_present {
            return;
        }

        if now_present {
            set_is_unmounting.set(false);
            set_exit_animation_started.set(false);
            let _ = el.set_attribute("data-state", "enter");
            send(PresenceEvent::Mount);
        } else {
            set_is_unmounting.set(true);
            set_exit_animation_started.set(false);

            let _ = el.set_attribute("data-state", "exit");
            let _ = el.offset_height();

            let has_animation = get_animation_name_recursive(&el).is_some();

            if has_animation {
                send(PresenceEvent::AnimationOut);
            } else {
                send(PresenceEvent::Unmount);
            }
        }

        set_prev_present.set(now_present);
    });

    Effect::new(move |_| {
        let handle_start = move |ev: ev::AnimationEvent| {
            if is_unmounting.get_untracked() && ev.animation_name().contains("exit") {
                set_exit_animation_started.set(true);
            }

            let current = active_animations.get_untracked();
            set_active_animations.set(current + 1);
        };

        let handle_end = move |ev: ev::AnimationEvent| {
            let current = active_animations.get_untracked();
            let new_count = (current - 1).max(0);
            set_active_animations.set(new_count);

            if new_count == 0 &&
                is_unmounting.get_untracked() &&
                exit_animation_started.get_untracked() {
                send(PresenceEvent::AnimationEnd);
            }
        };

        _ = use_event_listener(node_ref, animationstart, handle_start);
        _ = use_event_listener(node_ref, animationend, handle_end.clone());
        _ = use_event_listener(node_ref, animationcancel, handle_end);
    });

    view! {
        <TypedFallbackShow when=move || present.get() fallback=|| ()>
            {children.with_value(|children| children()).add_any_attr(any_node_ref(node_ref))}
        </TypedFallbackShow>
    }
}
// use leptos::{
//     ev,
//     ev::{animationcancel, animationend, animationstart},
//     html, logging, prelude::*,
// };
// use leptos::wasm_bindgen::JsCast;
// use leptos_node_ref::prelude::*;
// use leptos_use::use_event_listener;
// use radix_leptos_primitive::TypedFallbackShow;
// use web_sys::HtmlDivElement;
//
// /* -------------------------------------------------------------------------------------------------
//  * PresenceState, PresenceEvent
//  * -----------------------------------------------------------------------------------------------*/
//
// /// States representing the lifecycle of a component's presence in the DOM.
// ///
// /// - [`Mounted`]: The element is fully visible and rendered.
// /// - [`UnmountSuspended`]: An exit animation is playing, waiting to finish before removing the element.
// /// - [`Unmounted`]: The element is no longer rendered.
// #[derive(Debug, Clone, PartialEq)]
// enum PresenceState {
//     /// The child is fully visible in the DOM.
//     Mounted,
//     /// An exit animation is in progress, preventing immediate removal.
//     UnmountSuspended,
//     /// The child is removed from the DOM.
//     Unmounted,
// }
//
// /// Events that can trigger transitions between presence states.
// ///
// /// - [`Mount`]: Request to mount (or re-mount) the element.
// /// - [`Unmount`]: Request to unmount the element (potentially playing an exit animation).
// /// - [`AnimationOut`]: Confirmation that an exit animation is starting.
// /// - [`AnimationEnd`]: Completion of the exit animation, allowing the element to be fully removed.
// #[derive(Debug, Clone, Copy)]
// enum PresenceEvent {
//     /// Begin or continue showing the child in the DOM.
//     Mount,
//     /// Begin removing the child from the DOM, with an optional exit animation.
//     Unmount,
//     /// Signal that an exit animation is detected, pausing removal until it finishes.
//     AnimationOut,
//     /// Signal that the exit animation has completed, allowing final unmount.
//     AnimationEnd,
// }
//
// impl PresenceState {
//     /// Applies an event to the current [`PresenceState`], returning the next state if it matches
//     /// a valid transition.
//     fn transition(&self, event: PresenceEvent) -> Option<Self> {
//         #[cfg(debug_assertions)]
//         logging::log!("TRANSITION - From {:?} with event {:?}", self, event);
//
//         let result = match (self, event) {
//             // From "mounted", we can either unmount directly or detect an exit animation:
//             (Self::Mounted, PresenceEvent::Unmount) => Some(Self::UnmountSuspended),
//             (Self::Mounted, PresenceEvent::AnimationOut) => Some(Self::UnmountSuspended),
//
//             // While unmount-suspended (exit animation in progress), we can finalize unmount:
//             (Self::UnmountSuspended, PresenceEvent::AnimationEnd) => Some(Self::Unmounted),
//             // or forcibly skip the animation if another unmount request arrives:
//             (Self::UnmountSuspended, PresenceEvent::Unmount) => Some(Self::Unmounted),
//             // We intentionally omit `(Self::UnmountSuspended, PresenceEvent::Mount)` to allow the exit animation to complete.
//
//             // Fully unmounted can mount again.
//             (Self::Unmounted, PresenceEvent::Mount) => Some(Self::Mounted),
//
//             // No other transitions apply.
//             _ => None,
//         };
//
//         #[cfg(debug_assertions)]
//         if let Some(ref new_state) = result {
//             logging::log!("TRANSITION - To {:?}", new_state);
//         }
//
//         result
//     }
// }
//
// /* -------------------------------------------------------------------------------------------------
//  * Presence
//  * -----------------------------------------------------------------------------------------------*/
//
// /// A Leptos component that controls a child's presence in the DOM with optional enter/exit animations.
// ///
// /// # How It Works
// ///
// /// 1. When [`present`](Presence#structfield.present) changes from `false` to `true`, the child is
// ///    instantly mounted and optionally given a data attribute (e.g. `"enter"`) so CSS can trigger
// ///    an **enter animation**.
// /// 2. When `present` changes from `true` to `false`, we check for a named animation and, if found,
// ///    switch to [`PresenceState::UnmountSuspended`]. We remain in that state until the animation
// ///    ends, then unmount fully.
// /// 3. If no exit animation is found, unmount immediately.
// /// 4. If `present` toggles again during the exit animation, the code **waits** until that animation
// ///    finishes, avoiding an interrupt in mid-animation.
// ///
// /// # Example
// ///
// /// ```rust
// /// use leptos::prelude::*;
// /// use radix_leptos_presence::Presence;
// ///
// /// #[component]
// /// fn Demo() -> impl IntoView {
// ///
// ///     let (show, set_show) = signal(true);
// ///
// ///     view! {
// ///         <button on:click=move |_| set_show.update(|b| *b = !*b)>
// ///             {move || if show.get() { "Hide" } else { "Show" }}
// ///         </button>
// ///
// ///         <Presence present=show>
// ///             <div class="animate-in fade-in duration-300">
// ///                 "Hello, I will appear and disappear with animations!"
// ///             </div>
// ///         </Presence>
// ///     }
// /// }
// /// ```
// ///
// /// This code checks a child's computed `animation-name` to decide if an exit animation is playing.
// /// If you rely on transitions or an animation that reports `"none"` at computed style time, the
// /// component will unmount instantly.
// ///
// /// # Note
// ///
// /// This component wraps a `<div>` with a [`NodeRef`](leptos_node_ref::NodeRef). If you need more
// /// complex node mapping, see [`TypedFallbackShow`] or manual usage of `NodeRef`.
// #[component]
// pub fn Presence<C>(
//     /// Whether to show (mount) the child or hide (unmount) it.
//     ///
//     /// Switching from `false` to `true` triggers an optional enter animation;
//     /// switching from `true` to `false` triggers an optional exit animation.
//     #[prop(into)]
//     present: Signal<bool>,
//
//     /// Child content to render. Typically, a single `<div>`-like element
//     /// that has animation classes applied.
//     children: TypedChildrenFn<C>,
// ) -> impl IntoView
// where
//     C: IntoView + 'static,
// {
//     // NodeRef for the wrapper `<div>` so we can measure styles and attach animation listeners.
//     let node_ref = NodeRef::<html::Div>::new();
//
//     // We store the children in a `StoredValue` so they can be reused.
//     let children = StoredValue::new(children.into_inner());
//
//     // Current presence state (`Mounted`, `UnmountSuspended`, or `Unmounted`).
//     let (state, set_state) = signal(if present.get_untracked() {
//         PresenceState::Mounted
//     } else {
//         PresenceState::Unmounted
//     });
//
//     // Track the previous `present` so we detect toggles.
//     let (prev_present, set_prev_present) = signal(present.get_untracked());
//
//     // Count how many animations are currently active on our node (including nested children).
//     let (active_animations, set_active_animations) = signal(0);
//
//     // Indicates we’re in the process of unmounting (so any starting animation is an “exit”).
//     let (is_unmounting, set_is_unmounting) = signal(false);
//
//     // Tracks whether an exit animation has started at least once.
//     let (exit_animation_started, set_exit_animation_started) = signal(false);
//
//     // Function to send events to our presence state machine.
//     let send = move |event: PresenceEvent| {
//         if let Some(next_state) = state.get_untracked().transition(event) {
//             set_state.set(next_state);
//         }
//     };
//
//     /* ---------------------------------------------------------------------------------------------
//      * Synchronize `present` => state machine
//      * -------------------------------------------------------------------------------------------*/
//     Effect::new(move |_| {
//         let Some(el) = node_ref
//             .get()
//             .and_then(|node: HtmlDivElement| node.dyn_into::<web_sys::HtmlElement>().ok())
//         else {
//             // If the node is missing, we can’t animate yet.
//             return;
//         };
//
//         let was_present = prev_present.get();
//         let now_present = present.get();
//
//         if was_present == now_present {
//             return;
//         }
//
//         if now_present {
//             // Starting to show the element
//             set_is_unmounting.set(false);
//             set_exit_animation_started.set(false);
//
//             // e.g. `data-state="enter"` => CSS can pick that up
//             _ = el.set_attribute("data-state", "enter");
//             // Force layout so the newly-set attribute is recognized by the browser
//             let _ = el.offset_height();
//
//             send(PresenceEvent::Mount);
//         } else {
//             // Starting to hide the element (exit)
//             set_is_unmounting.set(true);
//             set_exit_animation_started.set(false);
//
//             _ = el.set_attribute("data-state", "exit");
//             let _ = el.offset_height(); // Force reflow again
//
//             // Check if there's a named animation
//             if get_animation_name_recursive(&el).is_some() {
//                 send(PresenceEvent::AnimationOut);
//             } else {
//                 // No recognized animation => unmount immediately
//                 send(PresenceEvent::Unmount);
//             }
//         }
//
//         set_prev_present.set(now_present);
//     });
//
//     /* ---------------------------------------------------------------------------------------------
//      * Animation event listeners
//      * -------------------------------------------------------------------------------------------*/
//     Effect::new(move |_| {
//         // Called for each animationstart event on node_ref or its child elements.
//         let handle_start = move |_ev: ev::AnimationEvent| {
//             // If we’re unmounting, treat it as an exit animation
//             if is_unmounting.get_untracked() {
//                 set_exit_animation_started.set(true);
//             }
//
//             let current = active_animations.get_untracked();
//             set_active_animations.set(current + 1);
//         };
//
//         // Called for each animationend/cancel event on node_ref or its child elements.
//         let handle_end = move |_ev: ev::AnimationEvent| {
//             let current = active_animations.get_untracked();
//             let new_count = (current - 1).max(0);
//             set_active_animations.set(new_count);
//
//             // If no active animations remain, finalize unmount.
//             if new_count == 0
//                 && is_unmounting.get_untracked()
//                 && exit_animation_started.get_untracked()
//             {
//                 send(PresenceEvent::AnimationEnd);
//             }
//         };
//
//         // Attach event listeners with `leptos_use::use_event_listener`
//         _ = use_event_listener(node_ref, animationstart, handle_start);
//         _ = use_event_listener(node_ref, animationend, handle_end.clone());
//         _ = use_event_listener(node_ref, animationcancel, handle_end);
//     });
//
//     /* ---------------------------------------------------------------------------------------------
//      * Rendering
//      * -------------------------------------------------------------------------------------------*/
//
//     // Renders the child if `Mounted` or `UnmountSuspended`, hides it when truly `Unmounted`.
//     view! {
//         <TypedFallbackShow
//             when=move || state.get() != PresenceState::Unmounted
//             fallback=|| ()
//         >
//             {
//                 children.with_value(|child_fn| {
//                     child_fn()
//                         // Ensure we attach the node_ref to the top-level child
//                         .add_any_attr(any_node_ref(node_ref))
//                 })
//             }
//         </TypedFallbackShow>
//     }
// }
//
// /* -------------------------------------------------------------------------------------------------
//  * Helpers
//  * -----------------------------------------------------------------------------------------------*/
//
// /// Checks for a non-empty `animation-name` on the given element (or any nested child).
// /// Returns `Some("fade-out")` etc. if at least one named keyframe is found, or `None` if none found.
// fn get_animation_name_recursive(element: &web_sys::Element) -> Option<String> {
//     if let Some(name) = get_animation_name(element) {
//         return Some(name);
//     }
//     let children = element.children();
//     for i in 0..children.length() {
//         if let Some(child) = children.item(i) {
//             if let Some(name) = get_animation_name_recursive(&child) {
//                 return Some(name);
//             }
//         }
//     }
//     None
// }
//
// /// Returns the `animation-name` from computed styles if it’s not `"none"` or empty.
// fn get_animation_name(element: &web_sys::Element) -> Option<String> {
//     window()
//         .get_computed_style(element)
//         .ok()
//         .flatten()
//         .and_then(|style| style.get_property_value("animation-name").ok())
//         .and_then(|anim| {
//             let trimmed = anim.trim();
//             if trimmed.is_empty() || trimmed == "none" {
//                 None
//             } else {
//                 Some(trimmed.to_string())
//             }
//         })
// }
