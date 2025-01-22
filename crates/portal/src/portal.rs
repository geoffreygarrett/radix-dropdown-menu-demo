use std::cell::RefCell;
use std::rc::Rc;
use leptos::{html, prelude::*, portal::Portal as LeptosPortal};
use leptos::attr::Attribute;
use leptos::attribute_interceptor::AttributeInterceptor;
use web_sys::Element;
use leptos_node_ref::AnyNodeRef;

#[component]
pub fn Portal(
    mount: Option<Element>,                 // Use Option instead of MaybeProp
    #[prop(optional)] use_shadow: bool,                   // Optional props as per Leptos docs
    #[prop(optional)] is_svg: bool,
    #[prop(into, optional)] as_child: MaybeProp<bool>,    // Keep MaybeProp<bool> if it's necessary and safe
    #[prop(optional)] node_ref: AnyNodeRef,               // NodeRef is fine as it doesn't require Send/Sync
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    // Create a mount point if none provided
    let mount_point = mount.unwrap_or_else(|| {
        let document = web_sys::window()
            .unwrap()
            .document()
            .unwrap();

        // First check if portal root already exists
        if let Some(existing) = document.get_element_by_id("portal-root") {
        // if let Some(existing) = document.get_element_by_id(use_id(Some("portal-root"))) {
            return existing;
        }

        // Create new portal root
        let portal_root = document.create_element("div").unwrap();
        portal_root.set_id("portal-root");
        // portal_root.set_attribute("style", "position: fixed; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; z-index: 1000;").unwrap();
        document.body().unwrap().append_child(&portal_root).unwrap();
        portal_root
    });
    // let mount: Element = node_ref.element();
    view! {
        // <LeptosPortal
        // mount=mount.unwrap_or_else(|| { web_sys::window().unwrap().document().unwrap().create_element("div").unwrap() })
        // use_shadow=use_shadow
        // is_svg=is_svgdd
        // >
        // <AttributeInterceptor let:attr>
        // {let attr = StoredValue::new(attr.into_cloneable_owned())}
        // TODO: Figure out attributes across the portal.

        <leptos_portal::Portal use_shadow=false mount=mount_point>
            {children.with_value(|children| children())}
        </leptos_portal::Portal>
    }
}
mod leptos_portal {
    use leptos::prelude::*;
    use leptos_dom::helpers::document;
    use std::sync::Arc;
    use leptos::mount;
    use leptos::wasm_bindgen::JsCast;

    /// Renders components directly into another DOM element without a wrapper.
    ///
    /// Useful for inserting modals and tooltips outside of a cropping layout.
    /// If no mount point is given, the portal is inserted in `document.body`.
    /// Setting `use_shadow` to `true` places the element in a shadow root to isolate styles.
    #[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", skip_all))]
    #[component]
    pub fn Portal<V>(
        /// Target element where the children will be appended
        #[prop(into, optional)]
        mount: Option<web_sys::Element>,
        /// Whether to use a shadow DOM inside `mount`. Defaults to `false`.
        #[prop(optional)]
        use_shadow: bool,
        /// The children to teleport into the `mount` element
        children: TypedChildrenFn<V>,
    ) -> impl IntoView
    where
        V: IntoView + 'static,
    {
        if cfg!(target_arch = "wasm32")
            && Owner::current_shared_context()
            .map(|sc| sc.is_browser())
            .unwrap_or(true)
        {
            use send_wrapper::SendWrapper;
            use wasm_bindgen::JsCast;

            let mount = mount.unwrap_or_else(|| {
                document().body().expect("body to exist").unchecked_into()
            });
            let children = children.into_inner();

            Effect::new(move |_| {
                let render_root = if use_shadow {
                    mount
                        .attach_shadow(&web_sys::ShadowRootInit::new(
                            web_sys::ShadowRootMode::Open,
                        ))
                        .map(|root| root.unchecked_into())
                        .unwrap_or(mount.clone())
                } else {
                    mount.clone()
                };

                // Mount directly to the target element or its shadow root
                let handle = SendWrapper::new((
                    mount::mount_to(render_root.unchecked_into(), {
                        let children = Arc::clone(&children);
                        move || untrack(|| children())
                    }),
                    mount.clone(),
                ));

                Owner::on_cleanup({
                    move || {
                        let (handle, _mount) = handle.take();
                        drop(handle);
                        // Cleanup happens automatically when the mount handle is dropped
                    }
                })
            });
        }
    }

}

// /// Based on [`leptos::Portal`].
// mod leptos_portal {
//     use cfg_if::cfg_if;
//     use leptos::{component, html::AnyElement, ChildrenFn, MaybeProp, NodeRef};
//     use leptos_dom::IntoView;
//
//     /// Renders components somewhere else in the DOM.
//     ///
//     /// Useful for inserting modals and tooltips outside of a cropping layout.
//     /// If no mount point is given, the portal is inserted in `document.body`;
//     #[cfg_attr(
//         any(debug_assertions, feature = "ssr"),
//         tracing::instrument(level = "trace", skip_all)
//     )]
//     #[component]
//     pub fn LeptosPortal(
//         /// Target element where the children will be appended
//         #[prop(into, optional)]
//         mount: MaybeProp<web_sys::Element>,
//         #[prop(optional)] mount_ref: NodeRef<AnyElement>,
//         /// The children to teleport into the `mount` element
//         children: ChildrenFn,
//     ) -> impl IntoView {
//         cfg_if! { if #[cfg(all(target_arch = "wasm32", any(feature = "hydrate", feature = "csr")))] {
//             use leptos::{on_cleanup, Effect, RwSignal, Signal, SignalGet, SignalSet, StoredValue};
//             use leptos_dom::{document, Mountable};
//             use web_sys::wasm_bindgen::JsCast;
//
//             let children = StoredValue::new(children);
//
//             let mount = Signal::derive(move || {
//                 mount_ref
//                     .get()
//                     .map(|mount| {
//                         let element: &web_sys::HtmlElement = &mount;
//                         element.clone().unchecked_into::<web_sys::Element>()
//                     })
//                     .or_else(|| mount.get())
//                     .unwrap_or_else(|| document().body().expect("body to exist").into())
//             });
//
//             let current_mount: RwSignal<Option<web_sys::Element>> = RwSignal::new(None);
//             let current_nodes: RwSignal<Option<Vec<web_sys::Node>>> = RwSignal::new(None);
//
//             let remove_nodes = move |current_mount: &web_sys::Element | {
//                 if let Some(current_nodes) = current_nodes.get() {
//                     for current_node in current_nodes {
//                         current_mount.remove_child(&current_node).expect("child to be removed");
//                     }
//                 }
//             };
//
//             Effect::new(move |_| {
//                 let mount = mount.get();
//                 if current_mount.get().as_ref() != Some(&mount) {
//                     if let Some(current_mount) = current_mount.get() {
//                         remove_nodes(&current_mount);
//                     }
//                     current_mount.set(Some(mount));
//                 }
//             });
//
//             Effect::new(move |_| {
//                 if let Some(current_mount) = current_mount.get() {
//                     remove_nodes(&current_mount);
//
//                     let node = children.with_value(|children| children().into_view().get_mountable_node());
//                     if let Some(fragment) = node.dyn_ref::<web_sys::DocumentFragment>() {
//                         let mut nodes: Vec<web_sys::Node> = vec![];
//                         for index in 0..fragment.children().length() {
//                             nodes.push(fragment.children().item(index).expect("child to exist").into());
//                         }
//
//                         current_mount.append_child(&node).expect("child to be appended");
//                         current_nodes.set(Some(nodes));
//                     } else {
//                         current_nodes.set(Some(vec![current_mount.append_child(&node).expect("child to be appended")]));
//                     }
//                 }
//             });
//
//             on_cleanup(move || {
//                 if let Some(current_mount) = current_mount.get() {
//                     remove_nodes(&current_mount);
//                 }
//             });
//         } else {
//             let _ = mount;
//             let _ = mount_ref;
//             let _ = children;
//         }}
//     }
// }
