use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::{ev, html};
use radix_leptos_id::use_id;
use radix_leptos_compose_refs::use_composed_refs;
use radix_leptos_primitive::Primitive;
use radix_leptos_menu::primitive as MenuPrimitive;
use radix_leptos_use_controllable_state::{use_controllable_state, UseControllableStateParams};
use radix_leptos_context::create_context;
use leptos_maybe_callback::MaybeCallback;
// pub use radix_leptos_checkbox::CheckedState;
pub use radix_leptos_direction::Direction;
pub use leptos_node_ref::AnyNodeRef;
use leptos::context::Provider;

/* -------------------------------------------------------------------------------------------------
 * DropdownMenu
 * -----------------------------------------------------------------------------------------------*/

const DROPDOWN_MENU_NAME: &str = "DropdownMenu";

#[derive(Clone)]
struct DropdownMenuContextValue {
    trigger_id: Signal<String>,
    trigger_ref: AnyNodeRef,
    content_id: Signal<String>,
    open: Signal<bool>,
    on_open_change: Callback<bool>,
    on_open_toggle: Callback<()>,
    modal: Signal<bool>,
}

create_context!(
    context_type: DropdownMenuContextValue,
    provider: DropdownMenuProvider,
    hook: use_dropdown_menu_context,
    root: DROPDOWN_MENU_NAME
);

#[component]
#[allow(non_snake_case)]
pub fn DropdownMenu(
    children: TypedChildrenFn<impl IntoView + 'static>,
    #[prop(optional, into)] dir: Direction,
    #[prop(optional, into)] open: MaybeProp<bool>,
    #[prop(optional, into)] default_open: MaybeProp<bool>,
    #[prop(optional, into)] on_open_change: MaybeCallback<bool>,
    #[prop(optional, into)] modal: MaybeProp<bool>,
) -> impl IntoView {
    let (open, set_open) = use_controllable_state(UseControllableStateParams {
        prop: open,
        default_prop: default_open,
        on_change: on_open_change.into(),
    });

    let toggle_open = Callback::new(move |_| {
        let current = open.get();
        leptos::logging::log!("DropdownMenu toggle_open called, current state: {}", current);
        set_open.run(!current);
    });

    let context_value = DropdownMenuContextValue {
        trigger_id: use_id().into(),
        trigger_ref: AnyNodeRef::new(),
        content_id: use_id().into(),
        open,
        on_open_change: set_open,
        on_open_toggle: toggle_open,
        modal: Signal::derive(move || modal.get().unwrap_or_default()),
    };

    view! {
        <DropdownMenuProvider value=context_value>
            <MenuPrimitive::Root
                children=children
                open=open
                on_open_change=set_open
                dir=dir
                modal=modal
            />
        </DropdownMenuProvider>
    }
}


/* -------------------------------------------------------------------------------------------------
 * DropdownMenuTrigger
 * -----------------------------------------------------------------------------------------------*/

const DROPDOWN_MENU_TRIGGER_NAME: &str = "DropdownMenuTrigger";

#[component]
#[allow(non_snake_case)]
pub fn DropdownMenuTrigger(
    children: TypedChildrenFn<impl IntoView + 'static>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] node_ref: AnyNodeRef,
    #[prop(optional, into)] as_child: MaybeProp<bool>,
) -> impl IntoView
{
    let children = StoredValue::new(children.into_inner());
    let context = use_dropdown_menu_context(DROPDOWN_MENU_TRIGGER_NAME);

    let on_pointer_down = move |event: ev::PointerEvent| {
        if !disabled.get().unwrap_or_default() && event.button() == 0 && !event.ctrl_key() {
            context.on_open_toggle.run(());
            if !context.open.get() {
                event.prevent_default();
            }
        }
    };

    let on_key_down = move |event: ev::KeyboardEvent| {
        if disabled.get().unwrap_or_default() {
            return;
        }
        if ["Enter", " "].contains(&event.key().as_str()) {
            context.on_open_toggle.run(());
            event.prevent_default();
        }
        if event.key() == "ArrowDown" {
            context.on_open_change.run(true);
            event.prevent_default();
        }
    };

    view! {
        <MenuPrimitive::Anchor as_child=true>
            <Primitive
                element=html::button
                as_child=as_child
                node_ref=use_composed_refs((node_ref, context.trigger_ref))
                attr:id=context.trigger_id
                attr:r#type="button"
                attr:aria-haspopup="menu"
                attr:aria-expanded=move || context.open.get()
                attr:aria-controls=move || {
                    if context.open.get() { Some(context.content_id.clone()) } else { None }
                }
                attr:data-state=move || if context.open.get() { "open" } else { "closed" }
                attr:data-disabled=move || {
                    if disabled.get().unwrap_or_default() { Some("") } else { None }
                }
                attr:disabled=move || disabled.get().unwrap_or_default()
                attr:class=move || class.get()
                {..}
                on:pointerdown=on_pointer_down
                on:keydown=on_key_down
            >
                {children.with_value(|children| children())}
            </Primitive>
        </MenuPrimitive::Anchor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * DropdownMenuPortal
 * -----------------------------------------------------------------------------------------------*/

pub use MenuPrimitive::Portal as DropdownMenuPortal;

/* -------------------------------------------------------------------------------------------------
 * DropdownMenuContent
 * -----------------------------------------------------------------------------------------------*/

const DROPDOWN_MENU_CONTENT_NAME: &str = "DropdownMenuContent";

#[component(transparent)]
#[allow(non_snake_case)]
pub fn DropdownMenuContent(
    children: ChildrenFn, // NOTE: No passthrough needed
    #[prop(optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context = use_dropdown_menu_context(DROPDOWN_MENU_CONTENT_NAME);

    let has_interacted_outside = RwSignal::new(false);

    let on_close_auto_focus = move |event: ev::Event| {
        if !has_interacted_outside.get() {
            if let Some(trigger) = context.trigger_ref.get() {
                let trigger_element = trigger.dyn_into::<web_sys::HtmlElement>()
                    .expect("Trigger should be an HTML element");
                trigger_element.focus().unwrap_or_default();
            }
        }
        has_interacted_outside.set(false);
        event.prevent_default();
    };

    let on_interact_outside = move |_event: ev::Event| {
        if !context.modal.get() {
            has_interacted_outside.set(true);
        }
    };
    view! {
        <MenuPrimitive::Content
            node_ref={node_ref}
            attr:aria-labelledby=context.trigger_id
            {..}
            id=context.content_id
            on:closeautofocus=on_close_auto_focus
            on:interactoutside=on_interact_outside
            style:--radix-dropdown-menu-content-transform-origin="var(--radix-popper-transform-origin)"
            style:--radix-dropdown-menu-content-available-width="var(--radix-popper-available-width)"
            style:--radix-dropdown-menu-content-available-height="var(--radix-popper-available-height)"
            style:--radix-dropdown-menu-trigger-width="var(--radix-popper-anchor-width)"
            style:--radix-dropdown-menu-trigger-height="var(--radix-popper-anchor-height)"
        >
            {children.with_value(|children| children())}
        </MenuPrimitive::Content>
    }
}

/* -------------------------------------------------------------------------------------------------
 * DropdownMenuGroup
 * -----------------------------------------------------------------------------------------------*/

pub use MenuPrimitive::Group as DropdownMenuGroup;

/* -------------------------------------------------------------------------------------------------
 * DropdownMenuLabel
 * -----------------------------------------------------------------------------------------------*/

pub use MenuPrimitive::Label as DropdownMenuLabel;

/* -------------------------------------------------------------------------------------------------
 * DropdownMenuItem
 * -----------------------------------------------------------------------------------------------*/

pub use MenuPrimitive::Item as DropdownMenuItem;

/* -------------------------------------------------------------------------------------------------
 * DropdownMenuCheckboxItem
 * -----------------------------------------------------------------------------------------------*/

// pub use MenuPrimitive::CheckboxItem as DropdownMenuCheckboxItem;

/* -------------------------------------------------------------------------------------------------
 * DropdownMenuRadioGroup
 * -----------------------------------------------------------------------------------------------*/

pub use MenuPrimitive::RadioGroup as DropdownMenuRadioGroup;

/* -------------------------------------------------------------------------------------------------
 * DropdownMenuRadioItem
 * -----------------------------------------------------------------------------------------------*/

// pub use MenuPrimitive::RadioItem as DropdownMenuRadioItem;

/* -------------------------------------------------------------------------------------------------
 * DropdownMenuItemIndicator
 * -----------------------------------------------------------------------------------------------*/

pub use MenuPrimitive::ItemIndicator as DropdownMenuItemIndicator;

/* -------------------------------------------------------------------------------------------------
 * DropdownMenuSeparator
 * -----------------------------------------------------------------------------------------------*/

pub use MenuPrimitive::Separator as DropdownMenuSeparator;

/* -------------------------------------------------------------------------------------------------
 * DropdownMenuArrow
 * -----------------------------------------------------------------------------------------------*/

pub use MenuPrimitive::Arrow as DropdownMenuArrow;

/* -------------------------------------------------------------------------------------------------
 * DropdownMenuSub
 * -----------------------------------------------------------------------------------------------*/

pub use MenuPrimitive::Sub as DropdownMenuSub;

/* -------------------------------------------------------------------------------------------------
 * DropdownMenuSubTrigger
 * -----------------------------------------------------------------------------------------------*/

pub use MenuPrimitive::SubTrigger as DropdownMenuSubTrigger;

/* -------------------------------------------------------------------------------------------------
 * DropdownMenuSubContent
 * -----------------------------------------------------------------------------------------------*/

#[component]
#[allow(non_snake_case)]
pub fn DropdownMenuSubContent(
    children: ChildrenFn, // NOTE: No passthrough needed
    #[prop(optional, into)] node_ref: AnyNodeRef,
) -> impl IntoView {
    view! {
        <MenuPrimitive::SubContent
            children={children}
            node_ref={node_ref}
            {..}
            style:--radix-dropdown-menu-content-transform-origin="var(--radix-popper-transform-origin)"
            style:--radix-dropdown-menu-content-available-width="var(--radix-popper-available-width)"
            style:--radix-dropdown-menu-content-available-height="var(--radix-popper-available-height)"
            style:--radix-dropdown-menu-trigger-width="var(--radix-popper-anchor-width)"
            style:--radix-dropdown-menu-trigger-height="var(--radix-popper-anchor-height)"
        />
    }
}

/* -------------------------------------------------------------------------------------------------
 * Primitive re-exports
 * -----------------------------------------------------------------------------------------------*/

pub mod primitive {
    pub use super::*;
    pub use DropdownMenu as Root;
    pub use DropdownMenuArrow as Arrow;
    // pub use DropdownMenuCheckboxItem as CheckboxItem;
    pub use DropdownMenuContent as Content;
    pub use DropdownMenuGroup as Group;
    pub use DropdownMenuItem as Item;
    pub use DropdownMenuItemIndicator as ItemIndicator;
    pub use DropdownMenuLabel as Label;
    pub use DropdownMenuPortal as Portal;
    pub use DropdownMenuRadioGroup as RadioGroup;
    // pub use DropdownMenuRadioItem as RadioItem;
    pub use DropdownMenuSeparator as Separator;
    pub use DropdownMenuSub as Sub;
    pub use DropdownMenuSubContent as SubContent;
    pub use DropdownMenuSubTrigger as SubTrigger;
    pub use DropdownMenuTrigger as Trigger;
}
