use leptos::html;
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use leptos_maybe_callback::MaybeCallback;
use radix_leptos_avatar::{primitive as AvatarPrimitive, ImageLoadingStatus};
use crate::cn;

#[component]
#[allow(non_snake_case)]
pub fn Avatar(
    children: TypedChildrenFn<impl IntoView + 'static>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    view! {
        <AvatarPrimitive::Root
            node_ref=node_ref
            children=children
            as_child=as_child
            attr:class=cn!(
                "relative flex h-10 w-10 shrink-0 overflow-hidden rounded-full", class.get()
            )
        />
    }
}

#[component]
#[allow(non_snake_case)]
pub fn AvatarImage(
    #[prop(into)] src: MaybeProp<String>,
    #[prop(into, optional)] alt: MaybeProp<String>,
    #[prop(into, optional)] referrer_policy: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] node_ref: NodeRef<html::Img>,
    #[prop(into, optional)] on_loading_status_change: MaybeCallback<ImageLoadingStatus>,
) -> impl IntoView {
    view! {
        <AvatarPrimitive::Image
            src=src
            node_ref=node_ref
            on_loading_status_change=on_loading_status_change
            referrer_policy=referrer_policy
            attr:class=move || cn!("aspect-square h-full w-full", class.get())
            attr:alt=move || alt.get().unwrap_or_default()
        />
    }
}

#[component]
#[allow(non_snake_case)]
pub fn AvatarFallback(
    children: TypedChildrenFn<impl IntoView + 'static>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] delay_ms: MaybeProp<i32>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    view! {
        <AvatarPrimitive::Fallback
            children=children
            as_child=as_child
            delay_ms=delay_ms
            node_ref=node_ref
            attr:class=move || {
                cn!(
                    "flex h-full w-full items-center justify-center rounded-full bg-muted", class.get()
                )
            }
        />
    }
}