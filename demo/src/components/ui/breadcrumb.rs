use leptos::prelude::*;
use leptos_router::components::{ToHref, A};
use lucide_leptos::{Ellipsis, ChevronRight};
use crate::cn;

#[component]
pub fn Breadcrumb(
    children: Children,
    #[prop(optional)] class: Option<&'static str>,
    #[prop(optional)] separator: Option<Children>,
) -> impl IntoView {
    view! {
        <nav aria-label="breadcrumb" class=class>
            {children()}
        </nav>
    }
}

#[component]
pub fn BreadcrumbList(
    children: Children,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    view! {
        <ol class=(
            [
                "flex",
                "flex-wrap",
                "items-center",
                "gap-1.5",
                "break-words",
                "text-sm",
                "text-muted-foreground",
                "sm:gap-2.5",
            ],
            true,
        )>{children()}</ol>
    }
}

#[component]
pub fn BreadcrumbItem(
    children: Children,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    view! { <li class=(["inline-flex", "items-center", "gap-1.5"], true)>{children()}</li> }
}

#[component]
pub fn BreadcrumbLink<H: ToHref + Send + Sync + 'static>(
    children: Children,
    href: H,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] on_click: Option<Callback<()>>,
) -> impl IntoView {
    view! {
        <A
            href=href
            attr:class=cn!("transition-colors hover:text-foreground", class.get())
            on:click=move |e| on_click.map(|c| c.run(())).unwrap_or(())
        >
            {children()}
        </A>
    }
}

#[component]
pub fn BreadcrumbPage(
    children: Children,
) -> impl IntoView {
    view! {
        <span
            role="link"
            aria-disabled="true"
            aria-current="page"
            class=(["font-normal", "text-foreground"], true)
        >
            {children()}
        </span>
    }
}

#[component]
pub fn BreadcrumbSeparator(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, optional)] class: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <li role="presentation" aria-hidden="true" class=cn!("[&>svg]:size-3.5", class.get())>
            {children.map(|c| c()).unwrap_or_else(|| view! { <ChevronRight /> }.into_any())}
        </li>
    }
}

#[component]
pub fn BreadcrumbEllipsis(
) -> impl IntoView {
    view! {
        <span
            role="presentation"
            aria-hidden="true"
            class=(["flex", "h-9", "w-9", "items-center", "justify-center"], true)
        >
            <Ellipsis class:w-4=true class:h-4=true />
            <span class="sr-only">"More"</span>
        </span>
    }
}
