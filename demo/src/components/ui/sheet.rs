use std::sync::Arc;
use leptos::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum SheetSide {
    Top,
    Bottom,
    Left,
    #[default]
    Right,
}

impl std::fmt::Display for SheetSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SheetSide::Top => write!(f, "top"),
            SheetSide::Bottom => write!(f, "bottom"),
            SheetSide::Left => write!(f, "left"),
            SheetSide::Right => write!(f, "right"),
        }
    }
}

impl std::str::FromStr for SheetSide {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "top" => Ok(SheetSide::Top),
            "bottom" => Ok(SheetSide::Bottom),
            "left" => Ok(SheetSide::Left),
            "right" => Ok(SheetSide::Right),
            _ => Err(format!("Invalid sheet side: {}", s)),
        }
    }
}

fn get_side_classes(side: &SheetSide) -> &'static str {
    match side {
        SheetSide::Top => "inset-x-0 top-0 border-b data-[state=closed]:slide-out-to-top data-[state=open]:slide-in-from-top",
        SheetSide::Bottom => "inset-x-0 bottom-0 border-t data-[state=closed]:slide-out-to-bottom data-[state=open]:slide-in-from-bottom",
        SheetSide::Left => "inset-y-0 left-0 h-full w-3/4 border-r data-[state=closed]:slide-out-to-left data-[state=open]:slide-in-from-left sm:max-w-sm",
        SheetSide::Right => "inset-y-0 right-0 h-full w-3/4 border-l data-[state=closed]:slide-out-to-right data-[state=open]:slide-in-from-right sm:max-w-sm",
    }
}

const BASE_SHEET_CLASSES: &str = "fixed z-50 gap-4 bg-background p-6 shadow-lg transition ease-in-out data-[state=closed]:duration-300 data-[state=open]:duration-500 data-[state=open]:animate-in data-[state=closed]:animate-out";

#[derive(Clone)]
pub struct SheetContext {
    pub is_open: RwSignal<bool>,
    pub on_close: Callback<()>,
}

#[component]
#[allow(non_snake_case)]
pub fn Sheet(
    #[prop(optional)] open: Option<RwSignal<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    children: Children,
) -> impl IntoView {
    let internal_open = RwSignal::new(default_open.unwrap_or(false));
    let is_open = open.unwrap_or(internal_open);

    // Ensure on_open_change is properly cloned and thread-safe
    let on_open_change = on_open_change.clone();

    let on_close = Callback::new(move |_| {
        is_open.set(false);
        if let Some(callback) = on_open_change.as_ref() {
            callback.run(false);
        }
    });

    let context = SheetContext {
        is_open: is_open.clone(),
        on_close: on_close.clone(),
    };

    provide_context(context);

    view! {
        <div class="sheet-root" data-state=move || if is_open.get() { "open" } else { "closed" }>
            {children()}
        </div>
    }
}

use leptos::*;
use leptos::prelude::*;
use crate::cn;

#[component]
#[allow(non_snake_case)]
pub fn SheetTrigger(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let context = use_context::<SheetContext>().expect("SheetTrigger must be used within Sheet");
    // let children = children()
    //     .nodes
    //     .into_iter()
    //     .map(|child| view! { <li>{child.as_element().}</li> })
    //     .collect_view();

    // view! {
    //     <ul>{children}</ul>
    // }


    view! {
        <div class=class role="button" on:click=move |_| context.is_open.set(true)>
            {children()}
        </div>
    }
}

#[component]
#[allow(non_snake_case)]
pub fn SheetOverlay(
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let context = use_context::<SheetContext>().expect("SheetOverlay must be used within Sheet");

    view! {
        <div
            class=cn!(
                "fixed inset-0 z-50 bg-black/80 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0", &class
            )
            data-state=move || if context.is_open.get() { "open" } else { "closed" }
            on:click=move |_| context.on_close.run(())
        />
    }
}
use derive_more::Display;

#[component]
#[allow(non_snake_case)]
pub fn SheetContent(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] side: SheetSide,
    children: Children,
) -> impl IntoView {
    let context = use_context::<SheetContext>().expect("SheetContent must be used within Sheet");
    let variant_class = get_side_classes(&side);

    let is_open = context.is_open;
    let fragment = children();

    view! {
        <div class="sheet-portal" class:hidden=move || !is_open.get()>
            <SheetOverlay />
            <div
                class=cn!(BASE_SHEET_CLASSES, variant_class, &class)
                data-state=move || if is_open.get() { "open" } else { "closed" }
            >
                <SheetClose />
                {fragment}
            </div>
        </div>
    }
}

#[component]
pub fn SheetClose() -> impl IntoView {
    let context = use_context::<SheetContext>().expect("SheetClose must be used within Sheet");

    view! {
        <button
            class="absolute top-4 right-4 rounded-sm opacity-70 transition-opacity hover:opacity-100 focus:ring-2 focus:ring-offset-2 focus:outline-none disabled:pointer-events-none ring-offset-background data-[state=open]:bg-secondary focus:ring-ring"
            on:click=move |_| context.on_close.run(())
        >
            <svg
                class="w-4 h-4"
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <line x1="18" y1="6" x2="6" y2="18"></line>
                <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
            <span class="sr-only">"Close"</span>
        </button>
    }
}

// Other components remain the same...

// Other components remain the same...

// Rest of the components remain the same...

#[component]
#[allow(non_snake_case)]
pub fn SheetHeader(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=cn!(
            "flex flex-col space-y-2 text-center sm:text-left", &class
        )>{children()}</div>
    }
}

#[component]
#[allow(non_snake_case)]
pub fn SheetFooter(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=cn!(
            "flex flex-col-reverse sm:flex-row sm:justify-end sm:space-x-2", &class
        )>{children()}</div>
    }
}

#[component]
#[allow(non_snake_case)]
pub fn SheetTitle(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! { <h2 class=cn!("text-lg font-semibold text-foreground", &class)>{children()}</h2> }
}

#[component]
#[allow(non_snake_case)]
pub fn SheetDescription(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! { <p class=cn!("text-sm text-muted-foreground", &class)>{children()}</p> }
}
