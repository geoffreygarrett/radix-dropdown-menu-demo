use leptos::{ev, html, logging};
use leptos::ev::on;
use leptos::prelude::*;
use leptos::tachys::html::class::class as cls;
use radix_leptos_primitive::{Primitive};
use radix_leptos_id::use_id;
// use crate::primitive;
// use leptix_primitives
// use leptix_primitives::primitive::Primitive;

#[derive(Clone)]
pub struct CollapsibleContext {
    pub open: ReadSignal<bool>,
    pub set_open: WriteSignal<bool>,
    pub content_id: String,
    pub disabled: bool,
    pub on_open_toggle: Callback<()>,
}

#[component]
pub fn CollapsibleProvider(
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] open: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let controlled = open.is_none();

    // Create the main signal with the correct initial state
    let (is_open, set_is_open) = signal(open.unwrap_or_else(|| default_open.unwrap_or(false)));

    // Store values in StoredValue to avoid FnOnce issues
    let class = StoredValue::new(class);
    let disabled = StoredValue::new(disabled);
    let on_open_change = StoredValue::new(on_open_change);

    let on_open_toggle = Callback::new(move |_| {
        let new_state = !is_open.get();
        if let Some(on_change) = on_open_change.with_value(|c| c.clone()) {
            on_change.run(new_state);
        }
        if !controlled {
            set_is_open.set(new_state);
        }
    });

    let context = CollapsibleContext {
        open: is_open,
        set_open: set_is_open,
        content_id: use_id().get(),
        disabled: disabled.with_value(|d| d.unwrap_or(false)),
        on_open_toggle,
    };

    provide_context(context);

    view! {
        <div
            class=move || class.with_value(|c| c.get())
            data-state=move || if is_open.get() { "open" } else { "closed" }
            data-disabled=move || disabled.with_value(|d| d.unwrap_or(false).to_string())
        >
            {children()}
        </div>
    }
}
pub fn use_collapsible() -> Option<CollapsibleContext> {
    use_context::<CollapsibleContext>()
}


#[component]
pub fn CollapsibleTrigger(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] aria_label: Option<String>,
    #[prop(optional, into)] as_child: MaybeProp<bool>,
    #[prop(into, default=Callback::new(|_| logging::log!("collapsible")))]
    on_click: Callback<ev::MouseEvent>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let CollapsibleContext {
        disabled,
        content_id,
        open,
        on_open_toggle,
        ..
    } = use_collapsible()
        .expect("CollapsibleTrigger must be used within CollapsibleProvider");

    // Combine click handlers
    let handle_click = move |ev: ev::MouseEvent| {
        on_click.run(ev);
        on_open_toggle.run(());
    };

    view! {
        <Primitive
            element=html::button
            as_child=as_child
            on:click=handle_click
            attr:disabled=disabled
            attr:aria-expanded=move || if open.get() { "true" } else { "false" }
            attr:aria-controls=content_id
            attr:aria-label=aria_label
            attr:data-disabled=move || if disabled { "true" } else { "false" }
            attr:data-state=move || if open.get() { "open" } else { "closed" }
            {..}
            class=("collapsible-trigger", true)
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}

#[component]
pub fn CollapsibleContent(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] force_mount: Option<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let context = use_context::<CollapsibleContext>()
        .expect("CollapsibleContent must be used within CollapsibleProvider");

    let class = StoredValue::new(class);
    let force_mount = StoredValue::new(force_mount);
    let content_id = StoredValue::new(context.content_id.clone());

    let show_content = create_memo(move |_| {
        context.open.get() || force_mount.with_value(|f| f.unwrap_or(false))
    });

    view! {
        <Show when=move || show_content.get() fallback=|| view! { <div /> }>
            <div
                id=move || content_id.with_value(|id| id.clone())
                class=move || class.with_value(|c| c.clone().unwrap_or_default())
                data-state=move || if context.open.get() { "open" } else { "closed" }
                hidden=move || {
                    !context.open.get() && !force_mount.with_value(|f| f.unwrap_or(false))
                }
            >
                {children()}
            </div>
        </Show>
    }
}

#[component]
pub fn Collapsible(
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] open: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(default=Callback::new(|_|{}), into)] on_open_change: Callback<bool>,
    // #[prop(default=Callback::new(|_|{}), into)] on_open_change: Callback<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <CollapsibleProvider
            default_open=default_open.unwrap_or(false)
            open=open.unwrap_or(false)
            disabled=disabled.unwrap_or(false)
            // on_open_change=on_open_change
            class=class
        >
            // <Primitive
            // element=html::div
            // as_child=as_child
            // on:click=handle_click
            // attr:disabled=disabled
            // class=("collapsible-trigger", true)
            // >
            // {children.with_value(|children| children())}
            // </Primitive>
            // </Primitive>
            // <Primitive.div
            // data-state={getState(open)}
            // data-disabled={disabled ? '' : undefined}
            // {...collapsibleProps}
            // ref={forwardedRef}
            // />
            {children()}
        </CollapsibleProvider>
    }
}