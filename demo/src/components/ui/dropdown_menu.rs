use std::sync::Arc;
use leptos::{
    attribute_interceptor::AttributeInterceptor,
    component,
    html,
    prelude::*,
    reactive::unwrap_signal,
};
use leptos_node_ref::prelude::{AnyNodeRef, IntoAnyNodeRef};
use lucide_leptos::{Check, ChevronRight, Circle};
// use radix_leptos_checkbox::{CheckedState, ToChecked};
use radix_leptos_dropdown_menu::primitive as DropdownMenuPrimitive;

pub use DropdownMenuPrimitive::Root as DropdownMenu;

pub use DropdownMenuPrimitive::Trigger as DropdownMenuTrigger;

pub use DropdownMenuPrimitive::Group as DropdownMenuGroup;

pub use DropdownMenuPrimitive::Portal as DropdownMenuPortal;

pub use DropdownMenuPrimitive::Sub as DropdownMenuSub;

pub use DropdownMenuPrimitive::RadioGroup as DropdownMenuRadioGroup;

use crate::cn;

#[component(transparent)]
#[allow(non_snake_case)]
pub fn DropdownMenuSubTrigger(
    children: ChildrenFn,
    #[prop(into, optional)] inset: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let children = StoredValue::new(children);
    view! {
        <DropdownMenuPrimitive::SubTrigger
            node_ref=node_ref
            attr:class=move || {
                cn!(
                    "flex cursor-default gap-2 select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:bg-accent data-[state=open]:bg-accent [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0", (inset.get().unwrap_or_default(), "pl-8")
                )
            }
        >
            {children.with_value(|children| children())}
            <ChevronRight class:ml-auto=true />
        </DropdownMenuPrimitive::SubTrigger>
    }
}

#[component(transparent)]
#[allow(non_snake_case)]
pub fn DropdownMenuSubContent(
    children: ChildrenFn,
    #[prop(optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let children = StoredValue::new(children);
    view! {
        <DropdownMenuPrimitive::SubContent
            node_ref=node_ref
            attr:class=move || {
                cn!(
                    "z-50 min-w-[8rem] overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-lg data-[state=open]:animate-in data-[state=closed]:animate-out data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2"
                )
            }
            attr:role="dialog"
        >
            {children.with_value(|children| children())}
        </DropdownMenuPrimitive::SubContent>
    }
}

#[component(transparent)]
#[allow(non_snake_case)]
pub fn DropdownMenuContent(
    children: ChildrenFn,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional, default = 4.into())] side_offset: MaybeProp<i32>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let children = StoredValue::new(children);
    view! {
        <DropdownMenuPrimitive::Portal>
            <DropdownMenuPrimitive::Content
                attr:class=move || {
                    cn!(
                        "z-50 min-w-[8rem] overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-md data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2", class.get()
                    )
                }
                node_ref=node_ref
            >
                {children.with_value(|children| children())}
            </DropdownMenuPrimitive::Content>
        </DropdownMenuPrimitive::Portal>
    }
}

#[component(transparent)]
#[allow(non_snake_case)]
pub fn DropdownMenuItem(
    children: ChildrenFn,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] inset: MaybeProp<bool>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let children = StoredValue::new(children);
    view! {
        <DropdownMenuPrimitive::Item
            disabled=disabled
            node_ref=node_ref
            attr:class=move || {
                cn!(
                    "relative flex cursor-default select-none items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-none transition-colors focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50 [&>svg]:size-4 [&>svg]:shrink-0", (inset.get().unwrap_or_default(), "pl-8"), class.get()
                )
            }
        >
            {children.with_value(|children| children())}
        </DropdownMenuPrimitive::Item>
    }
}

// #[component(transparent)]
// #[allow(non_snake_case)]
// pub fn DropdownMenuCheckboxItem(
//     children: ChildrenFn,
//     #[prop(into, optional)] checked: MaybeProp<CheckedState>,
//     #[prop(into, optional)] node_ref: AnyNodeRef,
// ) -> impl IntoView {
//     let children = StoredValue::new(children);
//     view! {
//         <DropdownMenuPrimitive::CheckboxItem
//             node_ref=node_ref
//             checked={checked}
//             attr:class=move || cn!("relative flex cursor-default select-none items-center rounded-sm py-1.5 pl-8 pr-2 text-sm outline-none transition-colors focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50")
//         >
//             <span class="absolute left-2 flex justify-center items-center w-3.5 h-3.5">
//                 <DropdownMenuPrimitive::ItemIndicator>
//                     <Check class:w-4=true class:h-4=true />
//                 </DropdownMenuPrimitive::ItemIndicator>
//             </span>
//             {children.with_value(|children| children())}
//         </DropdownMenuPrimitive::CheckboxItem>
//     }
// }

// #[component(transparent)]
// #[allow(non_snake_case)]
// pub fn DropdownMenuRadioItem<T: 'static + Send + Sync>(
//     children: ChildrenFn,
//     #[prop(into, optional)] value: MaybeProp<T>,
//     #[prop(into, optional)] node_ref: AnyNodeRef,
// ) -> impl IntoView {
//     let children = StoredValue::new(children);
//     view! {
//         <DropdownMenuPrimitive::RadioItem
//             node_ref={node_ref}
//             attr:class=move || cn!("relative flex cursor-default select-none items-center rounded-sm py-1.5 pl-8 pr-2 text-sm outline-none transition-colors focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50")
//         >
//             <span class="absolute left-2 flex justify-center items-center w-3.5 h-3.5">
//                 <DropdownMenuPrimitive::ItemIndicator>
//                     <Circle attr:class="w-2 h-2 fill-current" />
//                 </DropdownMenuPrimitive::ItemIndicator>
//             </span>
//             {children.with_value(|children| children())}
//         </DropdownMenuPrimitive::RadioItem>
//     }
// }

#[component(transparent)]
#[allow(non_snake_case)]
pub fn DropdownMenuLabel(
    children: ChildrenFn,
    #[prop(into, optional)] inset: MaybeProp<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let children = StoredValue::new(children);
    view! {
        <DropdownMenuPrimitive::Label
            node_ref=node_ref
            attr:class=cn!(
                "px-2 py-1.5 text-sm font-semibold", (inset.get().unwrap_or_default(), "pl-8"), class.get()
            )
        >
            {children.with_value(|children| children())}
        </DropdownMenuPrimitive::Label>
    }
}

#[component(transparent)]
#[allow(non_snake_case)]
pub fn DropdownMenuSeparator(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    view! {
        <DropdownMenuPrimitive::Separator
            node_ref=node_ref
            attr:class=move || cn!("-mx-1 my-1 h-px bg-muted", class.get())
        />
    }
}

#[component(transparent)]
#[allow(non_snake_case)]
pub fn DropdownMenuShortcut(
    children: Children,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] node_ref: NodeRef<html::Span>,
) -> impl IntoView {
    view! {
        <span
            node_ref=node_ref
            class=move || cn!("ml-auto text-xs tracking-widest opacity-60", class.get())
        >
            {children()}
        </span>
    }
}