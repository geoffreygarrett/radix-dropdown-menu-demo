use leptos::prelude::*;
use lucide_leptos::{ChevronUp, Plus};
use crate::components::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuLabel,
    DropdownMenuSeparator, DropdownMenuShortcut, DropdownMenuTrigger,
};
use crate::components::ui::sidebar::{
    SidebarMenu, SidebarMenuButton, SidebarMenuButtonSize, SidebarMenuItem, use_sidebar,
};
use leptos::control_flow::ForEnumerate;
use tailwind_fuse::tw_merge;

use crate::components::navigation::nav_main::IconViewFn;

/// Each “team” has a name, a plan, and an optional icon function
#[derive(Clone)]
pub struct Team {
    pub name: String,
    pub logo: Option<IconViewFn>, // We'll call it if present
    pub plan: String,
}

#[component]
#[allow(non_snake_case)]
pub fn TeamSwitcher(teams: Vec<Team>) -> impl IntoView {
    // Active team selection
    let (active_team, set_active_team) = signal(teams[0].clone());
    let sidebar_state = use_sidebar();
    let teams = StoredValue::new(teams);

    // Render the “logo” if it exists
    let render_logo = move |team: &Team| {
        match &team.logo {
            Some(logo_fn) => logo_fn(Some("w-4 h-4".to_string())),
            None => view! { <div class="w-4 h-4" /> }.into_any(),
        }
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
                            // Team logo
                            <div class=tw_merge!(
                                "flex aspect-square w-8 h-8 items-center justify-center rounded-lg bg-sidebar-primary text-sidebar-primary-foreground mr-2"
                            )>{render_logo(&active_team.get())}</div>

                            <div class="grid flex-1 text-left text-sm leading-tight">
                                <span class="truncate font-semibold">
                                    {move || active_team.get().name.clone()}
                                </span>
                                <span class="truncate text-xs">
                                    {move || active_team.get().plan.clone()}
                                </span>
                            </div>
                            <ChevronUp attr:class="ml-auto w-4 h-4" />
                        </SidebarMenuButton>
                    </DropdownMenuTrigger>

                    <DropdownMenuContent
                        class="w-[--radix-dropdown-menu-trigger-width] min-w-56 rounded-lg"
                        side_offset=4
                    >
                        // If you want dynamic alignment:
                        // side=move || if sidebar_state.is_mobile.get() { "bottom" } else { "right" }
                        <DropdownMenuLabel class="text-xs text-muted-foreground">
                            "Teams"
                        </DropdownMenuLabel>

                        <ForEnumerate
                            each=move || teams.with_value(|v| v.clone())
                            key=|team| team.name.clone()
                            children=move |index, team| {
                                let team = StoredValue::new(team.clone());
                                view! {
                                    <DropdownMenuItem
                                        class="gap-2 p-2"
                                        on:click=move |_| set_active_team.set(team.with_value(|v| v.clone()))
                                    >
                                        <div class="flex w-6 h-6 items-center justify-center rounded-sm border">
                                            {render_logo(&team.with_value(|v| v.clone()))}
                                        </div>
                                        {team.with_value(|v| v.clone()).name.clone()}
                                        <DropdownMenuShortcut>
                                            {"⌘"}{move || index.get() + 1}
                                        </DropdownMenuShortcut>
                                    </DropdownMenuItem>
                                }
                            }
                        />

                        <DropdownMenuSeparator />

                        <DropdownMenuItem class="gap-2 p-2">
                            <div class="flex w-6 h-6 items-center justify-center rounded-md border bg-background">
                                <Plus attr:class="w-4 h-4" />
                            </div>
                            <div class="font-medium text-muted-foreground">"Add team"</div>
                        </DropdownMenuItem>
                    </DropdownMenuContent>
                </DropdownMenu>
            </SidebarMenuItem>
        </SidebarMenu>
    }
}
