use leptos::prelude::*;
use leptos_router::components::A;
use lucide_leptos::ChevronRight;

use crate::components::ui::collapsible::{
    Collapsible, CollapsibleContent, CollapsibleTrigger,
};
use crate::components::ui::sidebar::{
    SidebarGroup, SidebarGroupLabel, SidebarMenu, SidebarMenuButton,
    SidebarMenuItem, SidebarMenuSub, SidebarMenuSubButton, SidebarMenuSubItem,
};

/// If you want an icon, define it as `Option<ViewFn>` so you can call it like `icon.map(|f| f(...))`.
pub type IconViewFn = fn(Option<String>) -> AnyView;

/// A sub-item for your nav (collapsible items).
#[derive(Clone)]
pub struct NavSubItem {
    pub title: String,
    pub url: String,
}

/// A main nav item that can hold an optional icon and optional sub-items.
#[derive(Clone)]
pub struct NavMainItem {
    pub title: String,
    pub url: String,
    pub icon: Option<IconViewFn>,   // None if no icon available
    pub is_active: Option<bool>,    // Whether the collapsible is open by default
    pub items: Option<Vec<NavSubItem>>,
}

#[component]
pub fn NavMain(items: Vec<NavMainItem>) -> impl IntoView {
    let items = StoredValue::new(items);
    view! {
        <SidebarGroup>
            <SidebarGroupLabel>"Platform"</SidebarGroupLabel>
            <SidebarMenu>
                <For
                    each=move || items.with_value(|v| v.clone())
                    key=|item| item.title.clone()
                    children=move |item| {
                        let is_open = item.is_active.unwrap_or(false);
                        let item = StoredValue::new(item);

                        view! {
                            <Collapsible
                                as_child=true
                                default_open=is_open
                                class="group/collapsible"
                            >
                                <SidebarMenuItem>
                                    <CollapsibleTrigger as_child=true>
                                        <SidebarMenuButton tooltip=item.with_value(|v| v
                                            .title
                                            .clone())>
                                            {// If the item has an icon, render it
                                            item.with_value(|v| v
                                                .icon
                                                .map(|icon_fn| {
                                                    icon_fn(Some("mr-2 w-4 h-4".to_string()))
                                                }))} <span>{item.with_value(|v| v.title.clone())}</span>
                                            <ChevronRight attr:class="ml-auto transition-transform duration-200 group-data-[state=open]/collapsible:rotate-90" />
                                        </SidebarMenuButton>
                                    </CollapsibleTrigger>
                                    <CollapsibleContent>
                                        <SidebarMenuSub>
                                            <For
                                                each=move || item.with_value(|v| v.items.clone().unwrap_or_default())
                                                key=|sub| sub.title.clone()
                                                children=move |sub| {
                                                    let sub = StoredValue::new(sub);
                                                    view! {
                                                        <SidebarMenuSubItem>
                                                            <SidebarMenuSubButton as_child=true>
                                                                <A href=sub.with_value(|v| v.url.clone())>
                                                                    <span>{sub.with_value(|v| v.title.clone())}</span>
                                                                </A>
                                                            </SidebarMenuSubButton>
                                                        </SidebarMenuSubItem>
                                                    }
                                                }
                                            />
                                        </SidebarMenuSub>
                                    </CollapsibleContent>
                                </SidebarMenuItem>
                            </Collapsible>
                        }
                    }
                />
            </SidebarMenu>
        </SidebarGroup>
    }
}
