// TODO: remove
#![expect(dead_code, unused_variables)]

use std::rc::Rc;
use std::{marker::PhantomData};

use leptos::context::Provider;
use leptos::tachys::html::node_ref::NodeRefContainer;
use leptos::{ev, ev::{CustomEvent, Event, FocusEvent, KeyboardEvent, MouseEvent, PointerEvent}, html, prelude::*};
use leptos_node_ref::prelude::*;
use leptos_remove_scroll::RemoveScroll;
// use radix_leptos_checkbox::{get_state as get_checked_state, is_indeterminate, CheckedState};
// use radix_leptos_collection::primitive as Collection;
use radix_leptos_compose_refs::{use_composed_refs};
use radix_leptos_direction::{use_direction, Direction};
// use radix_leptos_dismissable_layer::{
//     FocusOutsideEvent, InteractOutsideEvent, PointerDownOutsideEvent,
// };
use radix_leptos_id::use_id;
use radix_leptos_presence::Presence;

// use radix_leptos_focus_guards::use_focus_guards;
// use radix_leptos_focus_scope::FocusScope;
use radix_leptos_popper::{Popper, PopperAnchor, PopperArrow, PopperContent};
use radix_leptos_portal::Portal as PortalPrimitive;
use radix_leptos_primitive::{compose_callbacks, Primitive, VoidPrimitive};

// use radix_leptos_roving_focus::{Orientation, RovingFocusGroup, RovingFocusGroupItem};
use web_sys::{
    wasm_bindgen::{closure::Closure, JsCast},
    AddEventListenerOptions, CustomEventInit,
};
use leptos_typed_fallback_show::TypedFallbackShow;
// use radix_leptos_focus_guards::use_focus_guards;

const ENTER: &str = "Enter";
const SPACE: &str = " ";
const ARROW_DOWN: &str = "ArrowDown";
const PAGE_UP: &str = "PageUp";
const HOME: &str = "Home";
const ARROW_UP: &str = "ArrowUp";
const PAGE_DOWN: &str = "PageDown";
const END: &str = "End";
const ARROW_LEFT: &str = "ArrowLeft";
const ARROW_RIGHT: &str = "ArrowRight";

const SELECTION_KEYS: &[&str] = &[ENTER, SPACE];
const FIRST_KEYS: &[&str] = &[ARROW_DOWN, PAGE_UP, HOME];
const LAST_KEYS: &[&str] = &[ARROW_UP, PAGE_DOWN, END];
const FIRST_LAST_KEYS: &[&str] = &[ARROW_DOWN, PAGE_UP, HOME, ARROW_UP, PAGE_DOWN, END];

const SUB_OPEN_KEYS: &[(Direction, &[&str])] = &[
    (Direction::Ltr, &[ENTER, SPACE, ARROW_RIGHT]),
    (Direction::Rtl, &[ENTER, SPACE, ARROW_LEFT]),
];

const SUB_CLOSE_KEYS: &[(Direction, &[&str])] = &[
    (Direction::Ltr, &[ARROW_LEFT]),
    (Direction::Rtl, &[ARROW_RIGHT]),
];

/* -------------------------------------------------------------------------------------------------
 * Menu
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Debug)]
struct ItemData {
    disabled: bool,
    text_value: String,
}

const ITEM_DATA_PHANTOM: PhantomData<ItemData> = PhantomData;

#[derive(Clone)]
struct MenuContextValue {
    open: Signal<bool>,
    content_ref: NodeRef<html::Div>,
    on_open_change: Callback<bool>,
}

#[derive(Clone)]
struct MenuRootContextValue {
    is_using_keyboard: Signal<bool>,
    dir: Signal<Direction>,
    modal: Signal<bool>,
    on_close: Callback<()>,
}

#[component]
#[allow(non_snake_case)]
pub fn Menu<C: IntoView + 'static>(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] modal: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    children: TypedChildrenFn<C>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    let open = Signal::derive(move || open.get().unwrap_or(false));
    let modal = Signal::derive(move || modal.get().unwrap_or(true));
    let on_open_change = on_open_change.unwrap_or(Callback::new(|_| {}));

    let content_ref: NodeRef<html::Div> = NodeRef::new();
    let is_using_keyboard = RwSignal::new(false);
    let direction = use_direction(dir);

    let context_value = StoredValue::new(MenuContextValue {
        open,
        content_ref,
        on_open_change,
    });
    let root_context_value = StoredValue::new(MenuRootContextValue {
        is_using_keyboard: is_using_keyboard.into(),
        dir: direction,
        modal,
        on_close: Callback::new(move |_| on_open_change.run(false)),
    });

    let handle_pointer: Rc<Closure<dyn Fn(PointerEvent)>> = Rc::new(Closure::new(move |_| {
        is_using_keyboard.set(false);
    }));
    let cleanup_handle_pointer = handle_pointer.clone();

    let handle_key_down: Rc<Closure<dyn Fn(KeyboardEvent)>> = Rc::new(Closure::new(move |_| {
        is_using_keyboard.set(true);

        let options = AddEventListenerOptions::new();
        options.set_capture(true);
        options.set_once(true);

        document()
            .add_event_listener_with_callback_and_add_event_listener_options(
                "pointerdown",
                (*handle_pointer).as_ref().unchecked_ref(),
                &options,
            )
            .expect("Pointer down event listener should be added.");
        document()
            .add_event_listener_with_callback_and_add_event_listener_options(
                "pointermove",
                (*handle_pointer).as_ref().unchecked_ref(),
                &options,
            )
            .expect("Pointer move event listener should be added.");
    }));
    let cleanup_handle_key_down = handle_key_down.clone();

    Effect::new(move |_| {
        let options = AddEventListenerOptions::new();
        options.set_capture(true);

        // Capture phase ensures we set the boolean before any side effects execute
        // in response to the key or pointer event as they might depend on this value.
        document()
            .add_event_listener_with_callback_and_add_event_listener_options(
                "keydown",
                (*handle_key_down).as_ref().unchecked_ref(),
                &options,
            )
            .expect("Key down event listener should be added.");
    });

    // on_cleanup(move || {
    //     let options = EventListenerOptions::new();
    //     options.set_capture(true);
    //
    //     document()
    //         .remove_event_listener_with_callback_and_event_listener_options(
    //             "keydown",
    //             (*cleanup_handle_key_down).as_ref().unchecked_ref(),
    //             &options,
    //         )
    //         .expect("Key down event listener should be removed.");
    //
    //     document()
    //         .remove_event_listener_with_callback_and_event_listener_options(
    //             "pointerdown",
    //             (*cleanup_handle_pointer).as_ref().unchecked_ref(),
    //             &options,
    //         )
    //         .expect("Pointer down event listener should be removed.");
    //
    //     document()
    //         .remove_event_listener_with_callback_and_event_listener_options(
    //             "pointermove",
    //             (*cleanup_handle_pointer).as_ref().unchecked_ref(),
    //             &options,
    //         )
    //         .expect("Pointer move event listener should be removed.");
    // });

    view! {
        <Popper>
            <Provider value=context_value.get_value()>
                <Provider value=root_context_value
                    .get_value()>{children.with_value(|children| children())}</Provider>
            </Provider>
        </Popper>
    }
}

/* -------------------------------------------------------------------------------------------------
 * MenuAnchor
 * -----------------------------------------------------------------------------------------------*/

#[component]
#[allow(non_snake_case)]
pub fn MenuAnchor<C: IntoView + 'static>(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional, into)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<C>,
) -> impl IntoView
{
    let children = StoredValue::new(children.into_inner());
    view! {
        <PopperAnchor as_child=as_child node_ref=node_ref>
            {children.with_value(|children| children())}
        </PopperAnchor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * MenuPortal
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn MenuPortal<C: IntoView + 'static>(
    children: TypedChildrenFn<C>,
    #[prop(optional, into)] container: AnyNodeRef,
    #[prop(optional, into, default=MaybeProp::from(true))] force_mount: MaybeProp<bool>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<MenuContextValue>();
    let container = StoredValue::new(container);

    view! {
        <Presence present=Memo::new(move |_| {
            force_mount.get().unwrap_or(true) || context.open.get()
        })>
            <PortalPrimitive as_child=true mount=container.with_value(|container| container.get())>
                {children.with_value(|children| children())}
            </PortalPrimitive>
        </Presence>
    }
}

/* -------------------------------------------------------------------------------------------------
 * MenuContent
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone)]
struct MenuContentContextValue {
    on_item_enter: Callback<PointerEvent>,
    on_item_leave: Callback<PointerEvent>,
    on_trigger_leave: Callback<PointerEvent>,
    search: RwSignal<String>,
    pointer_grace_timer: RwSignal<u64>,
    on_pointer_grace_intent_change: Callback<Option<GraceIntent>>,
}

#[component]
#[allow(non_snake_case)]
pub fn MenuContent<C: IntoView + 'static>(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<C>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let root_context = expect_context::<MenuRootContextValue>();
    let context = expect_context::<MenuContextValue>();
    view! {
        <Presence present=context.open>
            // <Collection::Provider item_data_type=ITEM_DATA_PHANTOM>
            // <Collection::Slot item_data_type=ITEM_DATA_PHANTOM>
            <TypedFallbackShow
                when=move || root_context.modal.get()
                fallback=move || {
                    view! {
                        <MenuRootContentNonModal>
                            {children.with_value(|children| children())}
                        </MenuRootContentNonModal>
                    }
                }
            >
                <MenuRootContentModal as_child=as_child node_ref=node_ref>
                    {children.with_value(|children| children())}
                </MenuRootContentModal>
            </TypedFallbackShow>
        // </Collection::Slot>
        // </Collection::Provider>
        </Presence>
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[component]
#[allow(non_snake_case)]
fn MenuRootContentModal<C: IntoView + 'static>(
    // #[prop(into, optional)] on_focus_outside: Option<Callback<FocusOutsideEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<C>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<MenuContextValue>();
    let content_ref: AnyNodeRef = AnyNodeRef::new();
    let composed_refs = use_composed_refs([node_ref, content_ref]);

    // Hide everything from ARIA except the `MenuContent`.
    Effect::new(move |_| {
        if let Some(content) = content_ref.get() {
            // TODO: imported from `aria-hidden` in JS.
            // hide_others(content);
        }
    });

    view! {
        <MenuContentImpl
            // We make sure we're not trapping once it's been closed (closed != unmounted when animating out).
            trap_focus=context.open
            // Make sure to only disable pointer events when open. This avoids blocking interactions while animating out.
            disable_outside_pointer_events=context.open
            disable_outside_scroll=true
            // When focus is trapped, a `focusout` event may still happen. We make sure we don't trigger our `on_dismiss` in such case.
            // on_focus_outside=compose_callbacks(on_focus_outside, Some(Callback::new(move |event: FocusOutsideEvent| {
            // event.prevent_default();
            // })), Some(false))
            on_dismiss=Callback::new(move |_| context.on_open_change.run(false))
            as_child=as_child
            node_ref=composed_refs
        >
            {children.with_value(|children| children())}
        </MenuContentImpl>
    }
}

#[component]
#[allow(non_snake_case)]
fn MenuRootContentNonModal<C: IntoView + 'static>(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<C>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<MenuContextValue>();
    view! {
        <MenuContentImpl
            trap_focus=false
            disable_outside_pointer_events=false
            disable_outside_scroll=false
            on_dismiss=Callback::new(move |_| context.on_open_change.run(false))
            as_child=as_child
            node_ref=node_ref
        >
            {children.with_value(|children| children())}
        </MenuContentImpl>
    }
}


/* ---------------------------------------------------------------------------------------------- */

#[component]
#[allow(non_snake_case)]
fn ScrollLockWrapper<C: IntoView + 'static>(
    #[prop(into, optional)] disable_outside_scroll: MaybeProp<bool>,
    children: TypedChildrenFn<C>,
) -> impl IntoView
{
    let children = StoredValue::new(children.into_inner());
    view! {
        <TypedFallbackShow
            when=move || disable_outside_scroll.get().unwrap_or(true)
            fallback=move || children.with_value(|children| children()).into_view()
        >
            <RemoveScroll allow_pinch_zoom=true forward_props=false>
                {children.with_value(|children| children())}
            </RemoveScroll>
        </TypedFallbackShow>
    }
}

#[component]
#[allow(non_snake_case)]
fn MenuContentImpl<C: IntoView + 'static>(
    /// Event handler called when autofocusing on open. Can be prevented.
    #[prop(into, optional)]
    on_open_auto_focus: Option<Callback<Event>>,
    /// Event handler called when autofocusing on close. Can be prevented.
    #[prop(into, optional)]
    on_close_auto_focus: Option<Callback<Event>>,
    #[prop(into, optional)] disable_outside_pointer_events: MaybeProp<bool>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<KeyboardEvent>>,
    // #[prop(into, optional)] on_pointer_down_outside: Option<Callback<PointerDownOutsideEvent>>,
    // #[prop(into, optional)] on_focus_outside: Option<Callback<FocusOutsideEvent>>,
    // #[prop(into, optional)] on_interact_outside: Option<Callback<InteractOutsideEvent>>,
    #[prop(into, optional)] on_dismiss: Option<Callback<()>>,
    #[prop(into, optional)] on_key_down: Option<Callback<KeyboardEvent>>,
    #[prop(into, optional)] on_blur: Option<Callback<FocusEvent>>,
    #[prop(into, optional)] on_pointer_move: Option<Callback<PointerEvent>>,
    /// Whether scrolling outside the `MenuContent` should be prevented. Defaults to `false`.
    #[prop(into, optional)]
    disable_outside_scroll: MaybeProp<bool>,
    /// Whether focus should be trapped within the `MenuContent`. Defaults to `false`.
    #[prop(into, optional)]
    trap_focus: MaybeProp<bool>,
    #[prop(into, optional)]
    /// Whether keyboard navigation should loop around. Defaults to `false`.
    r#loop: MaybeProp<bool>,
    #[prop(into, optional)] on_entry_focus: Option<Callback<Event>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<C>,
) -> impl IntoView {
    let r#loop = Signal::derive(move || r#loop.get().unwrap_or(false));
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<MenuContextValue>();
    let root_context = expect_context::<MenuRootContextValue>();
    // let get_items = StoredValue::new(use_collection::<ItemData>());
    let (current_item_id, set_current_item_id) = signal::<Option<String>>(None);
    let content_ref: NodeRef<html::Div> = NodeRef::new();
    let composed_refs = use_composed_refs((node_ref, content_ref));
    let timer = RwSignal::new(0);
    let search = RwSignal::new("".to_string());
    let pointer_grace_timer = RwSignal::new(0);
    let pointer_grace_intent: RwSignal<Option<GraceIntent>> = RwSignal::new(None);
    let pointer_dir = RwSignal::new(Side::Right);
    let last_pointer_x = RwSignal::new(0);

    let clear_search: Closure<dyn Fn()> = Closure::new(move || {
        search.set("".into());
        window().clear_timeout_with_handle(timer.get());
    });

    // let handle_typeahead_search = Callback::new(move |key: String| {
    //     let search_value = search.get() + &key;
    //     // let items = get_items.with_value(|get_items| get_items());
    //     // let items = items
    //     //     .iter()
    //     //     .filter(|item| !item.data.disabled)
    //     //     .collect::<Vec<_>>();
    //     let current_item = document().active_element();
    //     // let current_match = items
    //     //     .iter()
    //     //     .find(|item| {
    //     //         item.r#ref.get().map(|html_element| {
    //     //             let element: &web_sys::Element = html_element.deref();
    //     //             element.clone()
    //     //         }) == current_item
    //     //     })
    //     //     .map(|item| item.data.text_value.clone());
    //     // let values = items
    //     //     .iter()
    //     //     .map(|item| item.data.text_value.clone())
    //     //     .collect::<Vec<_>>();
    //     // let next_match = get_next_match(values, search_value.clone(), current_match);
    //     // let new_item = items
    //     //     .iter()
    //     //     .find(|item| {
    //     //         next_match
    //     //             .as_ref()
    //     //             .is_some_and(|next_match| item.data.text_value == *next_match)
    //     //     })
    //     //     .and_then(|item| item.r#ref.get());
    //
    //     search.set(search_value.clone());
    //     window().clear_timeout_with_handle(timer.get());
    //     if !search_value.is_empty() {
    //         // Reset search 1 second after it was last updated.
    //         timer.set(
    //             window()
    //                 .set_timeout_with_callback_and_timeout_and_arguments_0(
    //                     clear_search.as_ref().unchecked_ref(),
    //                     1000,
    //                 )
    //                 .expect("Timeout should be set"),
    //         );
    //     }
    //
    //     // if let Some(new_item) = new_item {
    //     //     window()
    //     //         .set_timeout_with_callback(
    //     //             Closure::once(move || new_item.deref().focus())
    //     //                 .as_ref()
    //     //                 .unchecked_ref(),
    //     //         )
    //     //         .expect("Timeout should be set.");
    //     // }
    // });

    on_cleanup(move || {
        window().clear_timeout_with_handle(timer.get());
    });

    // Make sure the whole tree has focus guards as our `MenuContent` may be the last element in the DOM (because of the `Portal`).
    // use_focus_guards();

    let is_pointer_moving_to_submenu = move |event: &PointerEvent| -> bool {
        let is_moving_towards = Some(pointer_dir.get())
            == pointer_grace_intent
            .get()
            .map(|pointer_grace_intent| pointer_grace_intent.side);
        is_moving_towards
            && is_pointer_in_grace_area(
            event,
            pointer_grace_intent
                .get()
                .map(|pointer_grace_intent| pointer_grace_intent.area),
        )
    };

    let content_context_value = StoredValue::new(MenuContentContextValue {
        search,
        on_item_enter: Callback::new(move |event| {
            if is_pointer_moving_to_submenu(&event) {
                event.prevent_default();
            }
        }),
        on_item_leave: Callback::new(move |event| {
            if is_pointer_moving_to_submenu(&event) {
                return;
            }
            if let Some(content) = content_ref.get() {
                content.focus().expect("Element should be focused.");
            }
            set_current_item_id.set(None);
        }),
        on_trigger_leave: Callback::new(move |event| {
            if is_pointer_moving_to_submenu(&event) {
                event.prevent_default();
            }
        }),
        pointer_grace_timer,
        on_pointer_grace_intent_change: Callback::new(move |intent| {
            pointer_grace_intent.set(intent);
        }),
    });

    // // let mut attrs = attrs.clone();
    // attrs.extend([
    //     ("role", "menu".into_attribute()),
    //     ("aria-orientation", "vertical".into_attribute()),
    //     (
    //         "data-state",
    //         (move || get_open_state(context.open.get())).into_attribute(),
    //     ),
    //     ("data-radix-menu-content", "".into_attribute()),
    //     ("dir", (move || root_context.dir.get()).into_attribute()),
    //     // TODO: style
    // ]);

    // TODO: ScrollLockWrapper, DismissableLayer
    view! {
        <Provider value=content_context_value.get_value()>
            <ScrollLockWrapper disable_outside_scroll=disable_outside_scroll>
                // <FocusScope
                // as_child=true
                // trapped=trap_focus
                // on_mount_auto_focus=Callback::new(compose_callbacks(
                // on_open_auto_focus,
                // Some(Callback::new(move |event: Event| {
                // // When opening, explicitly focus the content area only and leave `onEntryFocus` in  control of focusing first item.
                // event.prevent_default();
                // 
                // if let Some(content) = content_ref.get_untracked() {
                // // TODO: focus with options doesn't exist in web-sys
                // content.focus().expect("Element should be focused");
                // }
                // })),
                // None,
                // ))
                // on_unmount_auto_focus=on_close_auto_focus
                // >
                // <RovingFocusGroup
                // as_child=true
                // dir=root_context.dir
                // orientation=Orientation::Vertical
                // r#loop=r#loop
                // current_tab_stop_id=current_item_id
                // on_current_tab_stop_id_change=move |value| set_current_item_id.set(value)
                // on_entry_focus=compose_callbacks(on_entry_focus, Some(Callback::new(move |event: Event| {
                // if !root_context.is_using_keyboard.get() {
                // event.prevent_default();
                // }
                // })), None)
                // prevent_scroll_on_entry_focus=true
                // >
                <PopperContent
                    as_child=as_child
                    node_ref=composed_refs
                    on:keydown=compose_callbacks(
                        on_key_down,
                        Some(
                            Callback::new(move |event: KeyboardEvent| {
                                let target = event
                                    .target()
                                    .map(|target| target.unchecked_into::<web_sys::HtmlElement>())
                                    .expect("Event should have target.");
                                let is_key_down_inside = target
                                    .closest("[data-radix-menu-content]")
                                    .expect("Element should be able to query closest.")
                                    == event
                                        .current_target()
                                        .and_then(|current_target| {
                                            current_target.dyn_into::<web_sys::Element>().ok()
                                        });
                                let is_modifier_key = event.ctrl_key() || event.alt_key()
                                    || event.meta_key();
                                let is_character_key = event.key().len() == 1;
                                if is_key_down_inside {
                                    if event.key() == "Tab" {
                                        event.prevent_default();
                                    }
                                    if !is_modifier_key && is_character_key {}
                                }
                                if content_ref.get().is_some_and(|content| *content == target) {
                                    if !FIRST_LAST_KEYS.contains(&event.key().as_str()) {
                                        return;
                                    }
                                    event.prevent_default();
                                }
                            }),
                        ),
                        None,
                    )
                    on:blur=compose_callbacks(
                        on_blur,
                        Some(
                            Callback::new(move |event: FocusEvent| {
                                let target = event
                                    .target()
                                    .map(|target| target.unchecked_into::<web_sys::Node>())
                                    .expect("Event should have target.");
                                let current_target = event
                                    .current_target()
                                    .map(|current_target| {
                                        current_target.unchecked_into::<web_sys::Node>()
                                    })
                                    .expect("Event should have current target.");
                                if !current_target.contains(Some(&target)) {
                                    window().clear_timeout_with_handle(timer.get());
                                    search.set("".into());
                                }
                            }),
                        ),
                        None,
                    )
                    on:pointermove=compose_callbacks(
                        on_pointer_move,
                        Some(
                            when_mouse(move |event: PointerEvent| {
                                let target = event
                                    .target()
                                    .map(|target| target.unchecked_into::<web_sys::HtmlElement>())
                                    .expect("Event should have target.");
                                let current_target = event
                                    .current_target()
                                    .map(|current_target| {
                                        current_target.unchecked_into::<web_sys::Node>()
                                    })
                                    .expect("Event should have current target.");
                                let pointer_x_has_changed = last_pointer_x.get()
                                    != event.client_x();
                                if current_target.contains(Some(&target)) && pointer_x_has_changed {
                                    let new_dir = match event.client_x() > last_pointer_x.get() {
                                        true => Side::Right,
                                        false => Side::Left,
                                    };
                                    pointer_dir.set(new_dir);
                                    last_pointer_x.set(event.client_x());
                                }
                            }),
                        ),
                        None,
                    )
                >
                    {children.with_value(|children| children())}
                </PopperContent>
            // </RovingFocusGroup>
            // </FocusScope>
            </ScrollLockWrapper>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * MenuGroup
 * -----------------------------------------------------------------------------------------------*/

#[component]
#[allow(non_snake_case)]
pub fn MenuGroup<C: IntoView + 'static>(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: NodeRef<html::Div>,
    children: TypedChildrenFn<C>,
) -> impl IntoView
{
    let children = StoredValue::new(children.into_inner());
    view! {
        <Primitive element=html::div as_child=as_child node_ref=node_ref attr:role="group">
            {children.with_value(|children| children())}
        </Primitive>
    }
}

/* -------------------------------------------------------------------------------------------------
 * MenuLabel
 * -----------------------------------------------------------------------------------------------*/
#[component]
#[allow(non_snake_case)]
pub fn MenuLabel<C: IntoView + 'static>(
    children: TypedChildrenFn<C>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
) -> impl IntoView
{
    let children = StoredValue::new(children.into_inner());
    view! {
        <Primitive element=html::div as_child=as_child node_ref=node_ref>
            {children.with_value(|children| children())}
        </Primitive>
    }
}

/* -------------------------------------------------------------------------------------------------
 * MenuItem
 * -----------------------------------------------------------------------------------------------*/

const ITEM_SELECT: &str = "menu.itemSelect";

#[component]
#[allow(non_snake_case)]
pub fn MenuItem<C: IntoView + 'static>(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_select: Option<Callback<Event>>,
    #[prop(into, optional)] on_click: Option<Callback<MouseEvent>>,
    #[prop(into, optional)] on_pointer_down: Option<Callback<PointerEvent>>,
    #[prop(into, optional)] on_pointer_up: Option<Callback<PointerEvent>>,
    #[prop(into, optional)] on_key_down: Option<Callback<KeyboardEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<C>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));

    let item_ref = NodeRef::<html::Div>::new();
    let composed_refs = use_composed_refs(vec![node_ref, item_ref.into_any()]);
    let root_context = expect_context::<MenuRootContextValue>();
    let content_context = expect_context::<MenuContentContextValue>();
    let is_pointer_down = RwSignal::new(false);

    let handle_select = Callback::new(move |_: MouseEvent| {
        if disabled.get() {
            return;
        }

        if let Some(item) = item_ref.get() {
            let on_select_clone = on_select.clone();
            let closure: Closure<dyn Fn(Event)> = Closure::new(move |event: Event| {
                if let Some(handler) = &on_select_clone {
                    handler.run(event);
                }
            });

            // Create and configure CustomEvent
            let mut init = CustomEventInit::new();
            init.bubbles(true);
            init.cancelable(true);

            if let Ok(item_select_event) = CustomEvent::new_with_event_init_dict(ITEM_SELECT, &init) {
                let event = item_select_event.unchecked_into::<Event>();

                // Add one-time event listener
                let _ = item.add_event_listener_with_callback_and_bool(
                    ITEM_SELECT,
                    closure.as_ref().unchecked_ref(),
                    true, // once
                );

                if let Ok(dispatched) = item.dispatch_event(&event) {
                    if !dispatched {
                        is_pointer_down.set(false);
                    } else {
                        root_context.on_close.run(());
                    }
                }
            }

            // Prevent memory leak
            closure.forget();
        }
    });

    view! {
        <MenuItemImpl
            disabled=disabled
            as_child=as_child
            node_ref=composed_refs
            on:click=compose_callbacks(on_click, Some(handle_select.clone()), None)
            on:pointerdown=move |event: PointerEvent| {
                if let Some(handler) = on_pointer_down.as_ref() {
                    handler.run(event);
                }
                is_pointer_down.set(true);
            }
            on:pointerup=compose_callbacks(
                on_pointer_up,
                Some(
                    Callback::new(move |event: PointerEvent| {
                        if is_pointer_down.get() {
                            if let Some(current_target) = event.current_target() {
                                let element = current_target
                                    .unchecked_into::<web_sys::HtmlElement>();
                                let _ = element.click();
                            }
                        }
                    }),
                ),
                None,
            )
            on:keydown=compose_callbacks(
                on_key_down,
                Some(
                    Callback::new(move |event: KeyboardEvent| {
                        let is_typing_ahead = !content_context.search.get().is_empty();
                        if disabled.get() || (is_typing_ahead && event.key() == " ") {
                            return;
                        }
                        if SELECTION_KEYS.contains(&event.key().as_str()) {
                            if let Some(current_target) = event.current_target() {
                                let element = current_target
                                    .unchecked_into::<web_sys::HtmlElement>();
                                let _ = element.click();
                            }
                            event.prevent_default();
                        }
                    }),
                ),
                None,
            )
        >
            {children.with_value(|children| children())}
        </MenuItemImpl>
    }
}

// const ITEM_SELECT: &str = "menu.itemSelect";
//
// #[component]
// #[allow(non_snake_case)]
// pub fn MenuItem<C: IntoView + 'static>(
//     #[prop(into, optional)] disabled: MaybeProp<bool>,
//     #[prop(into, optional)] on_select: Option<Callback<Event>>,
//     #[prop(into, optional)] on_click: Option<Callback<MouseEvent>>,
//     #[prop(into, optional)] on_pointer_down: Option<Callback<PointerEvent>>,
//     #[prop(into, optional)] on_pointer_up: Option<Callback<PointerEvent>>,
//     #[prop(into, optional)] on_key_down: Option<Callback<KeyboardEvent>>,
//     #[prop(into, optional)] as_child: MaybeProp<bool>,
//     #[prop(optional)] node_ref: AnyNodeRef,
//     children: TypedChildrenFn<C>,
// ) -> impl IntoView
// {
//     let children = StoredValue::new(children.into_inner());
//     let disabled = Signal::derive(move || disabled.get().unwrap_or(false));
//
//     let item_ref: AnyNodeRef = AnyNodeRef::new();
//     let composed_refs = use_composed_refs(vec![node_ref, item_ref]);
//     let root_context = expect_context::<MenuRootContextValue>();
//     let content_context = expect_context::<MenuContentContextValue>();
//     let is_pointer_down = RwSignal::new(false);
//
//     let handle_select = Callback::new(move |_: MouseEvent| {
//         if disabled.get() {
//             return;
//         }
//
//         // if let Some(item) = item_ref.get() {
//         //     let closure: Closure<dyn Fn(Event)> = Closure::new(move |event: Event| {
//         //         if let Some(on_select) = on_select {
//         //             on_select.run(event);
//         //         }
//         //     });
//         //
//         //     let init = CustomEventInit::new();
//         //     init.set_bubbles(true);
//         //     init.set_cancelable(true);
//         //
//         //     let item_select_event = CustomEvent::new_with_event_init_dict(ITEM_SELECT, &init)
//         //         .expect("Item select event should be instantiated.");
//         //
//         //     let options = AddEventListenerOptions::new();
//         //     options.set_once(true);
//         //
//         //     item.add_event_listener_with_callback_and_add_event_listener_options(
//         //         ITEM_SELECT,
//         //         closure.as_ref().unchecked_ref(),
//         //         &options,
//         //     )
//         //         .expect("Item select event listener should be added.");
//         //     item.dispatch_event(&item_select_event)
//         //         .expect("Item select event should be dispatched.");
//         //
//         //     if item_select_event.default_prevented() {
//         //         is_pointer_down.set(false);
//         //     } else {
//         //         root_context.on_close.run(());
//         //     }
//         // }
//     });
//
//     view! {
//       <MenuItemImpl
//         disabled=disabled
//         as_child=as_child
//         node_ref=composed_refs
//         on:click=compose_callbacks(on_click, Some(handle_select), None)
//         on:pointerdown=move |event| {
//           if let Some(on_pointer_down) = on_pointer_down {
//             on_pointer_down.run(event);
//           }
//           is_pointer_down.set(true);
//         }
//         on:pointerup=compose_callbacks(
//           on_pointer_up,
//           Some(
//             Callback::new(move |event: PointerEvent| {
//               if is_pointer_down.get() {
//                 if let Some(current_target) = event
//                   .current_target()
//                   .map(|current_target| { current_target.unchecked_into::<web_sys::HtmlElement>() })
//                 {
//                   current_target.click();
//                 }
//               }
//             }),
//           ),
//           None,
//         )
//         on:keydown=compose_callbacks(
//           on_key_down,
//           Some(
//             Callback::new(move |event: KeyboardEvent| {
//               let is_typing_ahead = !content_context.search.get().is_empty();
//               if disabled.get() || (is_typing_ahead && event.key() == " ") {
//                 return;
//               }
//               if SELECTION_KEYS.contains(&event.key().as_str()) {
//                 let current_target = event
//                   .current_target()
//                   .map(|current_target| { current_target.unchecked_into::<web_sys::HtmlElement>() })
//                   .expect("Event should have current target.");
//                 current_target.click();
//                 event.prevent_default();
//               }
//             }),
//           ),
//           None,
//         )
//       >
//         {children.with_value(|children| children())}
//       </MenuItemImpl>
//     }
// }

#[component]
#[allow(non_snake_case)]
fn MenuItemImpl<C: IntoView + 'static>(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] text_value: MaybeProp<String>,
    #[prop(into, optional)] on_pointer_move: Option<Callback<PointerEvent>>,
    #[prop(into, optional)] on_pointer_leave: Option<Callback<PointerEvent>>,
    #[prop(into, optional)] on_focus: Option<Callback<FocusEvent>>,
    #[prop(into, optional)] on_blur: Option<Callback<FocusEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<C>,
) -> impl IntoView
where
{
    let children = StoredValue::new(children.into_inner());
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));

    let content_context = expect_context::<MenuContentContextValue>();
    let item_ref: AnyNodeRef = AnyNodeRef::new();
    let composed_ref = use_composed_refs(vec![node_ref, item_ref]);
    let (is_focused, set_is_focused) = signal(false);

    // Get the item's `.textContent` as default strategy for typeahead `textValue`.
    let (text_content, set_text_content) = signal("".to_string());
    // Effect::new(move |_| {
    //     if let Some(item) = item_ref.get() {
    //         set_text_content.set(item.text_content().unwrap_or("".into()).trim().into());
    //     }
    // });

    let item_data = Signal::derive(move || ItemData {
        disabled: disabled.get(),
        text_value: text_value.get().unwrap_or(text_content.get()),
    });

    view! {
        // <Collection::ItemSlot item_data_type=ITEM_DATA_PHANTOM item_data=item_data>
        // <RovingFocusGroupItem as_child=true focusable=Signal::derive(move || !disabled.get())>
        <Primitive
            element=html::div
            as_child=as_child
            style:outline="none"
            node_ref=composed_ref
            attr:role="menuitem"
            attr:data-highlighted=move || is_focused.get().then_some("").unwrap_or_default()
            attr:aria-disabled=move || disabled.get()
            attr:data-disabled=move || disabled.get()
            on:pointermove=compose_callbacks(
                on_pointer_move,
                Some(
                    when_mouse(move |event| {
                        if disabled.get() {
                            content_context.on_item_leave.run(event);
                        } else {
                            content_context.on_item_enter.run(event.clone());
                            if !event.default_prevented() {
                                let item = event
                                    .current_target()
                                    .map(|target| target.unchecked_into::<web_sys::HtmlElement>())
                                    .expect("Current target should exist.");
                                item.focus().expect("Element should be focused.");
                            }
                        }
                    }),
                ),
                None,
            )
            on:pointerleave=compose_callbacks(
                on_pointer_leave,
                Some(
                    when_mouse(move |event| {
                        content_context.on_item_leave.run(event);
                    }),
                ),
                None,
            )
            on:focus=compose_callbacks(
                on_focus,
                Some(
                    Callback::new(move |_| {
                        set_is_focused.set(true);
                    }),
                ),
                None,
            )
            on:blur=compose_callbacks(
                on_blur,
                Some(
                    Callback::new(move |_| {
                        set_is_focused.set(false);
                    }),
                ),
                None,
            )
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}


/* -------------------------------------------------------------------------------------------------
 * MenuCheckboxItem
 * -----------------------------------------------------------------------------------------------*/
// #[component]
// pub fn MenuCheckboxItem(
//     #[prop(optional, into)] checked: MaybeProp<CheckedState>,
//     #[prop(into, optional)] node_ref: AnyNodeRef,
//     #[prop(optional, into)] on_checked_change: Option<Callback<bool>>,
//     children: Children,
// ) -> impl IntoView {
//     let checked = checked.get().unwrap_or_default();
//     let child_view = children();
//     let onclick = {
//         let on_checked_change = on_checked_change.clone();
//         move |ev: ev::MouseEvent| {
//             if let Some(cb) = on_checked_change.as_ref() {
//                 let new_val = if is_indeterminate(checked) {
//                     true
//                 } else {
//                     !matches!(checked, CheckedState::True)
//                 };
//                 cb.run(new_val);
//             }
//             ev.prevent_default();
//         }
//     };
//
//     view! {
//         <div
//             node_ref=node_ref
//             role="menuitemcheckbox"
//             aria-checked=checked
//             on:click=onclick
//             tabindex=0
//             data-state=get_checked_state(checked)
//         >
//             {child_view}
//         </div>
//     }
// }


// Radio Group Context
#[derive(Clone)]
pub struct RadioGroupContext {
    pub value: Signal<Option<String>>,
    pub on_value_change: Callback<String>,
}

/* -------------------------------------------------------------------------------------------------
 * MenuRadioGroup
 * -----------------------------------------------------------------------------------------------*/
#[component]
pub fn MenuRadioGroup<C: IntoView + 'static>(
    children: TypedChildrenFn<C>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional, default=Callback::new(|_|{}))] on_value_change: Callback<String>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <Provider value=RadioGroupContext {
            value: Signal::derive(move || value.get()),
            on_value_change,
        }>
            <MenuGroup>{children.with_value(|children| children())}</MenuGroup>
        </Provider>
    }
}

// MenuRadioItem component
// #[component]
// pub fn MenuRadioItem(
//     children: Children,
//     #[prop(into, optional)] node_ref: AnyNodeRef,
//     #[prop(into, optional)] value: MaybeProp<String>,
// ) -> impl IntoView {
//     let child_view = children();
//     let radio_ctx = use_context::<RadioGroupContext>().expect("RadioGroupContext not found");
//
//     // Derive checked state
//     let checked = Signal::derive(move || {
//         radio_ctx.value.get().as_ref() == Some(&value.get().unwrap_or_default())
//     });
//
//     // Click handler
//     let onclick = {
//         let cb = radio_ctx.on_value_change.clone();
//         let val_clone = value.get().unwrap_or_default();
//         move |_ev: ev::MouseEvent| {
//             cb.run(val_clone.to_string());
//         }
//     };
//
//     // Derive state for styling
//     let state = Signal::<CheckedState>::derive(move || checked.get().into());
//
//     view! {
//         <div
//             node_ref=node_ref
//             role="menuitemradio"
//             aria-checked=move || checked.get()
//             on:click=onclick
//             tabindex=0
//             data-state=move || get_checked_state(state.get())
//             class="flex relative items-center py-1.5 px-2 text-sm rounded-sm transition-colors cursor-pointer outline-none select-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent focus:bg-accent focus:text-accent-foreground"
//         >
//             {child_view}
//         </div>
//     }
// }


#[component]
#[allow(non_snake_case)]
pub fn MenuItemIndicator<C: IntoView + 'static>(
    children: TypedChildrenFn<C>,
    #[prop(optional)] force_mount: Option<bool>,
    #[prop(optional, into)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <Show when=move || force_mount.unwrap_or(false) fallback=|| ()>
            <span>{children.with_value(|children| children())}</span>
        </Show>
    }
}

// #[component]
// pub fn MenuSeparator(#[prop(optional, into)] node_ref: AnyNodeRef) -> impl IntoView {
//     view! { <div node_ref=node_ref role="separator" aria-orientation="horizontal" /> }
// }

#[component]
#[allow(non_snake_case)]
// NOTE: Using ChildrenFn (AnyView) instead of TypedChildrenFn<C> since we need optional children
//  -> no attribute spreading for now.
pub fn MenuSeparator(
    #[prop(optional, into)] as_child: MaybeProp<bool>,
    #[prop(optional, into)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    if as_child.get().unwrap_or_default() {
        // TODO: as_child support for `MenuSeparator`
        unimplemented!("as_child support for `MenuSeparator`")
    }
    view! {
        <VoidPrimitive
            element=html::div
            as_child=as_child
            node_ref=node_ref
            attr:role="separator"
            attr:aria-orientation="horizontal"
        >
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </VoidPrimitive>
    }
}

#[component]
#[allow(non_snake_case)]
// NOTE: Using ChildrenFn (AnyView) instead of TypedChildrenFn<C> since we need optional children
//  -> no attribute spreading for now.
pub fn MenuArrow(
    #[prop(optional, into)] as_child: MaybeProp<bool>,
    #[prop(optional, into)] node_ref: AnyNodeRef,
    children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    view! {
        <PopperArrow as_child=as_child node_ref=node_ref>
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </PopperArrow>
    }
}

#[derive(Clone)]
pub struct MenuSubContextValue {
    pub content_id: ReadSignal<String>,
    pub trigger_id: ReadSignal<String>,
    pub trigger: RwSignal<Option<NodeRef<html::Div>>>,
    pub on_trigger_change: Callback<Option<NodeRef<html::Div>>>,
}

#[component]
#[allow(non_snake_case)]
pub fn MenuSub<C: IntoView + 'static>(
    children: TypedChildrenFn<C>,
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional, default=Callback::new(|_|{}))] on_open_change: Callback<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let parent_menu_context = expect_context::<MenuContextValue>();

    // State management
    let trigger = RwSignal::new(None::<NodeRef<html::Div>>);
    let content = RwSignal::new(None::<NodeRef<html::Div>>);

    // Create the sub context value
    let sub_context = MenuSubContextValue {
        content_id: use_id(),
        trigger_id: use_id(),
        trigger,
        on_trigger_change: Callback::new(move |new_trigger| {
            trigger.set(new_trigger);
        }),
    };

    // Effect to handle parent menu state changes
    Effect::new(move |_| {
        if !parent_menu_context.open.get() {
            on_open_change.run(false);
        }
    });

    // Cleanup effect
    on_cleanup(move || {
        on_open_change.run(false);
    });

    let menu_context = MenuContextValue {
        open: Signal::derive(move || open.get().unwrap_or_default()),
        content_ref: NodeRef::new(),
        on_open_change,
    };
    let menu_context = StoredValue::new(menu_context);
    let sub_context = StoredValue::new(sub_context);

    view! {
        <Popper>
            <Provider value=menu_context.get_value()>
                <Provider value=sub_context
                    .get_value()>{children.with_value(|children| children())}</Provider>
            </Provider>
        </Popper>
    }
}

// #[component]
// pub fn MenuSubTrigger(
//     #[prop(optional, default = false)] disabled: bool,
//     children: Children,
// ) -> impl IntoView {
//     let child_view = children();
//     // let sub_ctx = use_context::<MenuSubContextValue>().expect("MenuSubContext not found");
//     let menu_ctx = use_context::<MenuContextValue>().expect("MenuContext not found");
//
//     let onclick = {
//         let cb = menu_ctx.on_open_change.clone();
//         move |_ev: ev::MouseEvent| {
//             if !disabled && !menu_ctx.open.get() {
//                 cb.run(true);
//             }
//         }
//     };
//
//     let node_ref = NodeRef::new();
//     Effect::new(move |_| {
//         // sub_ctx.trigger.set(Some(node_ref));
//     });
//
//     view! {
//       <div
//         // id=sub_ctx.trigger_id
//         aria-haspopup="menu"
//         aria-expanded=move || menu_ctx.open.get()
//         on:click=onclick
//         tabindex=0
//         node_ref=node_ref
//       >
//         {child_view}
//       </div>
//     }
// }
pub const SUB_TRIGGER_NAME: &str = "MenuSubTrigger";
#[component]
pub fn MenuSubTrigger<C: IntoView + 'static>(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(optional)] onclick: Option<Callback<MouseEvent>>,
    #[prop(optional)] onpointerleave: Option<Callback<PointerEvent>>,
    #[prop(optional)] onpointermove: Option<Callback<PointerEvent>>,
    #[prop(optional)] onkeydown: Option<Callback<KeyboardEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<C>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));

    let context = expect_context::<MenuContextValue>();
    let root_context = expect_context::<MenuRootContextValue>();
    let sub_context = expect_context::<MenuSubContextValue>();
    let content_context = expect_context::<MenuContentContextValue>();

    let open_timer_ref = RwSignal::new(None::<i32>);

    // Clear timer on cleanup
    on_cleanup(move || {
        if let Some(timer) = open_timer_ref.get() {
            window().clear_timeout_with_handle(timer);
        }
    });

    let trigger_id = StoredValue::new(sub_context.trigger_id);
    let content_id = StoredValue::new(sub_context.content_id);

    view! {
        <MenuAnchor as_child=true>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attr:id=trigger_id.with_value(|id| id.clone())
                attr:aria-haspopup="menu"
                attr:aria-expanded=move || context.open.get()
                attr:aria-controls=content_id.with_value(|id| id.clone())
                attr:data-state=move || get_open_state(context.open.get())
                attr:data-disabled=move || disabled.get().then_some("").unwrap_or_default()
                on:click=move |event: MouseEvent| {
                    if let Some(cb) = onclick.as_ref() {
                        cb.run(event.clone());
                    }
                    if disabled.get() || event.default_prevented() {
                        return;
                    }
                    if let Some(target) = event.current_target() {
                        let element: web_sys::HtmlElement = target.unchecked_into();
                        let _ = element.focus();
                    }
                    if !context.open.get() {
                        context.on_open_change.run(true);
                    }
                }
                on:pointermove=move |event: PointerEvent| {
                    if let Some(cb) = onpointermove.as_ref() {
                        cb.run(event.clone());
                    }
                    if event.pointer_type() != "mouse" {
                        return;
                    }
                    content_context.on_item_enter.run(event.clone());
                    if event.default_prevented() {
                        return;
                    }
                    if !disabled.get() && !context.open.get() {
                        content_context.on_pointer_grace_intent_change.run(None);
                        if let Some(timer) = open_timer_ref.get() {
                            window().clear_timeout_with_handle(timer);
                        }
                        let set_open: Closure<dyn Fn()> = Closure::new(move || {
                            context.on_open_change.run(true);
                            open_timer_ref.set(None);
                        });
                        let timer = window()
                            .set_timeout_with_callback_and_timeout_and_arguments_0(
                                set_open.as_ref().unchecked_ref(),
                                100,
                            )
                            .expect("Failed to set timeout");
                        open_timer_ref.set(Some(timer));
                        set_open.forget();
                    }
                }
                on:pointerleave=move |event: PointerEvent| {
                    if let Some(cb) = onpointerleave.as_ref() {
                        cb.run(event.clone());
                    }
                    if event.pointer_type() != "mouse" {
                        return;
                    }
                    if let Some(timer) = open_timer_ref.get() {
                        window().clear_timeout_with_handle(timer);
                        open_timer_ref.set(None);
                    }
                    if let Some(content) = context.content_ref.get() {
                        let content_rect = content.get_bounding_client_rect();
                        let side = content
                            .dataset()
                            .get("side")
                            .unwrap_or_else(|| "right".to_string());
                        let right_side = side == "right";
                        let bleed = if right_side { -5.0 } else { 5.0 };
                        let area = vec![
                            Point {
                                x: event.client_x() as f64 + bleed,
                                y: event.client_y() as f64,
                            },
                            Point {
                                x: if right_side {
                                    content_rect.left()
                                } else {
                                    content_rect.right()
                                },
                                y: content_rect.top(),
                            },
                            Point {
                                x: if right_side {
                                    content_rect.right()
                                } else {
                                    content_rect.left()
                                },
                                y: content_rect.top(),
                            },
                            Point {
                                x: if right_side {
                                    content_rect.right()
                                } else {
                                    content_rect.left()
                                },
                                y: content_rect.bottom(),
                            },
                            Point {
                                x: if right_side {
                                    content_rect.left()
                                } else {
                                    content_rect.right()
                                },
                                y: content_rect.bottom(),
                            },
                        ];
                        content_context
                            .on_pointer_grace_intent_change
                            .run(
                                Some(GraceIntent {
                                    area,
                                    side: if right_side { Side::Right } else { Side::Left },
                                }),
                            );
                        let clear_intent: Closure<dyn Fn()> = Closure::new(move || {
                            content_context.on_pointer_grace_intent_change.run(None);
                        });
                        let timer = window()
                            .set_timeout_with_callback_and_timeout_and_arguments_0(
                                clear_intent.as_ref().unchecked_ref(),
                                300,
                            )
                            .expect("Failed to set timeout");
                        content_context.pointer_grace_timer.set(timer as u64);
                        clear_intent.forget();
                    } else {
                        content_context.on_trigger_leave.run(event.clone());
                        if event.default_prevented() {
                            return;
                        }
                        content_context.on_pointer_grace_intent_change.run(None);
                    }
                }
                on:keydown=move |event: KeyboardEvent| {
                    if let Some(cb) = onkeydown.as_ref() {
                        cb.run(event.clone());
                    }
                    let is_typing_ahead = !content_context.search.get().is_empty();
                    if disabled.get() || (is_typing_ahead && event.key() == " ") {
                        return;
                    }
                    let open_keys = match root_context.dir.get() {
                        Direction::Ltr => vec!["ArrowRight"],
                        Direction::Rtl => vec!["ArrowLeft"],
                    };
                    if open_keys.contains(&event.key().as_str()) {
                        context.on_open_change.run(true);
                        if let Some(content) = context.content_ref.get() {
                            let _ = content.focus();
                        }
                        event.prevent_default();
                    }
                }
            >
                {children.with_value(|children| children())}
            </Primitive>
        </MenuAnchor>
    }
}
#[component]
pub fn MenuSubContent(
    children: ChildrenFn, // NOTE: No need for attribute spread
    #[prop(optional)] force_mount: Option<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let menu_ctx = expect_context::<MenuContextValue>();
    let children = StoredValue::new(children);
    let is_open = Memo::new(move |_| menu_ctx.open.get());
    view! {
        <Presence present=is_open>
            <MenuContentImpl
                trap_focus=false
                disable_outside_pointer_events=false
                disable_outside_scroll=false
                node_ref=node_ref
            >
                {children.with_value(|children| children())}
            </MenuContentImpl>
        </Presence>
    }
}

fn get_open_state(open: bool) -> String {
    match open {
        true => "open".into(),
        false => "closed".into(),
    }
}

fn focus_first(candidates: Vec<web_sys::HtmlElement>) {
    let previously_focused_element = document().active_element();
    for candidate in candidates {
        // If focus is already where we want to go, we don't want to keep going through the candidates.
        if previously_focused_element.as_ref() == candidate.dyn_ref::<web_sys::Element>() {
            return;
        }

        candidate.focus().expect("Element should be focused.");
        if document().active_element() != previously_focused_element {
            return;
        }
    }
}

/// Wraps an array around itself at a given start index.
fn wrap_array<T: Clone>(array: &mut [T], start_index: usize) -> &[T] {
    array.rotate_right(start_index);
    array
}

/// This is the "meat" of the typeahead matching logic. It takes in all the values,
/// the search and the current match, and returns the next match (or `None`).
///
/// We normalize the search because if a user has repeatedly pressed a character,
/// we want the exact same behavior as if we only had that one character
/// (ie. cycle through options starting with that character)
///
/// We also reorder the values by wrapping the array around the current match.
/// This is so we always look forward from the current match, and picking the first
/// match will always be the correct one.
///
/// Finally, if the normalized search is exactly one character, we exclude the
/// current match from the values because otherwise it would be the first to match always
/// and focus would never move. This is as opposed to the regular case, where we
/// don't want focus to move if the current match still matches.
fn get_next_match(
    values: Vec<String>,
    search: String,
    current_match: Option<String>,
) -> Option<String> {
    let is_repeated =
        search.chars().count() > 1 && search.chars().all(|c| c == search.chars().next().unwrap());
    let normalized_search = if is_repeated {
        search.chars().take(1).collect()
    } else {
        search
    };
    let current_match_index = current_match
        .as_ref()
        .and_then(|current_match| values.iter().position(|value| value == current_match));
    let mut wrapped_values =
        wrap_array(&mut values.clone(), current_match_index.unwrap_or(0)).to_vec();
    let exclude_current_match = normalized_search.chars().count() == 1;
    if exclude_current_match {
        wrapped_values.retain(|v| {
            current_match
                .as_ref()
                .is_none_or(|current_match| v != current_match)
        });
    }
    let next_match = wrapped_values.into_iter().find(|value| {
        value
            .to_lowercase()
            .starts_with(&normalized_search.to_lowercase())
    });

    if next_match != current_match {
        next_match
    } else {
        None
    }
}

#[derive(Clone, Debug)]
struct Point {
    x: f64,
    y: f64,
}

type Polygon = Vec<Point>;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Side {
    Left,
    Right,
}

#[derive(Clone, Debug)]
struct GraceIntent {
    area: Polygon,
    side: Side,
}

/// Determine if a point is inside of a polygon.
fn is_point_in_polygon(point: Point, polygon: Polygon) -> bool {
    let Point { x, y } = point;
    let mut inside = false;

    let mut i = 0;
    let mut j = polygon.len() - 1;
    while i < polygon.len() {
        let xi = polygon[i].x;
        let yi = polygon[i].y;
        let xj = polygon[j].x;
        let yj = polygon[j].y;

        let intersect = ((yi > y) != (yj > y)) && (x < (xj - xi) * (y - yi) / (yj - yi) + xi);
        if intersect {
            inside = !inside;
        }

        j = i;
        i += 1;
    }

    inside
}

fn is_pointer_in_grace_area(event: &PointerEvent, area: Option<Polygon>) -> bool {
    if let Some(area) = area {
        let cursor_pos = Point {
            x: event.client_x() as f64,
            y: event.client_y() as f64,
        };
        is_point_in_polygon(cursor_pos, area)
    } else {
        false
    }
}

fn when_mouse<H: Fn(PointerEvent) + 'static + Sync + Send>(handler: H) -> Callback<PointerEvent> {
    Callback::new(move |event: PointerEvent| {
        if event.pointer_type() == "mouse" {
            handler(event);
        }
    })
}

/* -------------------------------------------------------------------------------------------------
 * Primitive re-exports
 * -----------------------------------------------------------------------------------------------*/
pub mod primitive {
    pub use super::*;
    pub use Menu as Root;
    pub use MenuAnchor as Anchor;
    pub use MenuArrow as Arrow;
    // pub use MenuCheckboxItem as CheckboxItem;
    pub use MenuContent as Content;
    pub use MenuGroup as Group;
    pub use MenuItem as Item;
    pub use MenuItemIndicator as ItemIndicator;
    pub use MenuLabel as Label;
    pub use MenuPortal as Portal;
    pub use MenuRadioGroup as RadioGroup;
    // pub use MenuRadioItem as RadioItem;
    pub use MenuSeparator as Separator;
    pub use MenuSub as Sub;
    pub use MenuSubContent as SubContent;
    pub use MenuSubTrigger as SubTrigger;
}