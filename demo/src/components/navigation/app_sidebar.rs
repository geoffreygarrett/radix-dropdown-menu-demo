use leptos::prelude::*;
use crate::components::navigation::nav_main::{NavMain, NavMainItem, NavSubItem};
use crate::components::navigation::nav_user::{NavUser, User};
use crate::components::navigation::team_switcher::{Team, TeamSwitcher};
use crate::components::ui::sidebar::{
    Sidebar, SidebarCollapsible, SidebarContent, SidebarFooter, SidebarHeader, SidebarRail,
};

// If you eventually want a real icon system, replace with your actual icon type or ViewFn
#[derive(Clone)]
pub struct IconType;

// A placeholder Project struct, if you need it:
#[derive(Clone)]
pub struct Project {
    pub name: String,
    pub url: String,
    pub icon: IconType,
}

/// Get all data for the sidebar (user, teams, nav items, projects)
fn get_data() -> (User, Vec<Team>, Vec<NavMainItem>, Vec<Project>) {
    let user = User {
        name: "shadcn".to_string(),
        email: "m@example.com".to_string(),
        avatar: "/avatars/shadcn.jpg".to_string(),
    };

    let teams = vec![
        Team {
            name: "Acme Inc".to_string(),
            // For now, we use `None` if you don’t have an actual icon function
            logo: None,
            plan: "Enterprise".to_string(),
        },
        Team {
            name: "Acme Corp.".to_string(),
            logo: None,
            plan: "Startup".to_string(),
        },
        Team {
            name: "Evil Corp.".to_string(),
            logo: None,
            plan: "Free".to_string(),
        },
    ];

    let nav_main = vec![
        NavMainItem {
            title: "Playground".to_string(),
            url: "#".to_string(),
            icon: None, // No icon for now
            is_active: Some(true),
            items: Some(vec![
                NavSubItem {
                    title: "History".to_string(),
                    url: "#".to_string(),
                },
                NavSubItem {
                    title: "Starred".to_string(),
                    url: "#".to_string(),
                },
                NavSubItem {
                    title: "Settings".to_string(),
                    url: "#".to_string(),
                },
            ]),
        },
    ];

    let projects = vec![
        Project {
            name: "Design Engineering".to_string(),
            url: "#".to_string(),
            icon: IconType,
        },
        Project {
            name: "Sales & Marketing".to_string(),
            url: "#".to_string(),
            icon: IconType,
        },
        Project {
            name: "Travel".to_string(),
            url: "#".to_string(),
            icon: IconType,
        },
    ];

    (user, teams, nav_main, projects)
}

#[component]
#[allow(non_snake_case)]
pub fn AppSidebar() -> impl IntoView {
    let (user, teams, nav_main, projects) = get_data();

    let user = StoredValue::new(user);
    let teams = StoredValue::new(teams);
    let nav_main = StoredValue::new(nav_main);
    let projects = StoredValue::new(projects);

    view! {
        <Sidebar collapsible=SidebarCollapsible::Icon>
            <SidebarHeader>
                <TeamSwitcher teams=teams.with_value(|v| v.clone()) />
            </SidebarHeader>

            <SidebarContent>
                // Your main navigation
                <NavMain items=nav_main.with_value(|v| v.clone()) />

            // If you want to show projects, you'd do:
            // <NavProjects projects=projects />
            </SidebarContent>

            <SidebarFooter>
                <NavUser user=user.with_value(|v| v.clone()) />
            </SidebarFooter>

            <SidebarRail />
        </Sidebar>
    }
}
