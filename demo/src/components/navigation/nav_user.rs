use leptos::prelude::*;
use lucide_leptos::{Sparkles, BadgeCheck, CreditCard, Bell, LogOut, ChevronsUpDown};
use crate::components::hooks::use_is_mobile;
use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::components::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuGroup,
    DropdownMenuItem, DropdownMenuLabel, DropdownMenuSeparator,
};
use crate::components::ui::sidebar::{SidebarMenu, SidebarMenuButton, SidebarMenuItem, SidebarMenuButtonSize};

// Basic user info
#[derive(Clone, Debug)]
pub struct User {
    pub name: String,
    pub email: String,
    pub avatar: String,
}

#[component]
#[allow(non_snake_case)]
pub fn NavUser(user: User) -> impl IntoView {
    // Example: if you have a `use_is_mobile` hook
    let is_mobile = use_is_mobile();
    let user = StoredValue::new(user);

    // Memo for avatar initials
    let initials = Memo::new(move |_| {
        user.with_value(|v| v
            .name
            .split_whitespace()
            .filter(|w| !w.is_empty())
            .take(2)
            .filter_map(|w| w.chars().next())
            .map(|c| c.to_uppercase().next().unwrap_or_default())
            .collect::<String>())
    });

    // Handler for menu item selection
    let on_select = move |key: String| {
        log::warn!("Selected key: {}", key);
    };

    view! {
        <SidebarMenu>
            <SidebarMenuItem>
                <DropdownMenu>
                    <DropdownMenuTrigger as_child=true>
                        <SidebarMenuButton
                            size=SidebarMenuButtonSize::Lg
                            class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
                        >
                            <Avatar class="w-8 h-8 rounded-lg mr-2">
                                <AvatarImage src=user.with_value(|v| v.avatar.clone()) alt=user.with_value(|v| v.name.clone()) />
                                <AvatarFallback class="rounded-lg">{initials}</AvatarFallback>
                            </Avatar>
                            <div class="grid flex-1 text-sm leading-tight text-left">
                                <span class="font-semibold truncate">{user.with_value(|v| v.name.clone())}</span>
                                <span class="text-xs truncate">{user.with_value(|v| v.email.clone())}</span>
                            </div>
                            <ChevronsUpDown attr:class="ml-auto w-4 h-4" />
                        </SidebarMenuButton>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent
                        side_offset=4
                        attr:align=move || if is_mobile.get() { "bottom" } else { "right" }
                        class="rounded-lg min-w-56 w-[var(--radix-dropdown-menu-trigger-width)]"
                    >
                        <DropdownMenuLabel class="p-0 font-normal">
                            <div class="flex gap-2 items-center py-1.5 px-1 text-sm text-left">
                                <Avatar class="w-8 h-8 rounded-lg">
                                    <AvatarImage src=user.with_value(|v| v.avatar.clone()) alt=user.with_value(|v| v.name.clone()) />
                                    <AvatarFallback class="rounded-lg">{initials}</AvatarFallback>
                                </Avatar>
                                <div class="grid flex-1 text-sm leading-tight text-left">
                                    <span class="font-semibold truncate">{user.with_value(|v| v.name.clone())}</span>
                                    <span class="text-xs truncate">{user.with_value(|v| v.email.clone())}</span>
                                </div>
                            </div>
                        </DropdownMenuLabel>
                        <DropdownMenuSeparator />
                        <DropdownMenuGroup>
                            <DropdownMenuItem on:click=move |_| on_select("upgrade".into())>
                                <Sparkles />
                                "Upgrade to Pro"
                            </DropdownMenuItem>
                        </DropdownMenuGroup>
                        <DropdownMenuSeparator />
                        <DropdownMenuGroup>
                            <DropdownMenuItem on:click=move |_| on_select("account".into())>
                                <BadgeCheck />
                                "Account"
                            </DropdownMenuItem>
                            <DropdownMenuItem on:click=move |_| on_select("billing".into())>
                                <CreditCard />
                                "Billing"
                            </DropdownMenuItem>
                            <DropdownMenuItem on:click=move |_| on_select("notifications".into())>
                                <Bell />
                                "Notifications"
                            </DropdownMenuItem>
                        </DropdownMenuGroup>
                        <DropdownMenuSeparator />
                        <DropdownMenuItem on:click=move |_| on_select("logout".into())>
                            <LogOut />
                            "Log out"
                        </DropdownMenuItem>
                    </DropdownMenuContent>
                </DropdownMenu>
            </SidebarMenuItem>
        </SidebarMenu>
    }
}
