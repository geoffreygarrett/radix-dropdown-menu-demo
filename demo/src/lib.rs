#![allow(unused)]
#![allow(unused_imports)]
pub(crate) mod dashboard;
pub(crate) mod components;
pub(crate) mod demo;
pub(crate) mod utils;

use leptos::prelude::*;
use leptos_meta::{Html, Meta, Title};
use leptos_routable::prelude::{MaybeParam, Routable};
use leptos_router::components::{Router, A};
use crate::dashboard::{Page as DashboardPage};

#[derive(Routable)]
#[routes(
    view_prefix = "",
    view_suffix = "Page",
    transition = false
)]
pub enum AppRoutes {
    #[route(path = "/")]
    Dashboard,
    #[fallback]
    #[route(path = "/404")]
    NotFound,
}

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <div class="p-4 text-center">
            <h1 class="text-2xl font-bold">"404: Not Found"</h1>
            <p>"Sorry, we can't find that page."</p>
            <A
                href=AppRoutes::Dashboard
                attr:class="inline-block px-4 py-2 bg-green-500 text-white rounded mt-4"
            >
                "Go Home"
            </A>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    leptos_meta::provide_meta_context();
    view! {
        <Html attr:lang="en" attr:dir="ltr" />
        <Title text="Welcome to Leptos CSR" />
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <main class="min-h-screen">
            <Router>{move || AppRoutes::routes()}</Router>
        </main>
    }
}