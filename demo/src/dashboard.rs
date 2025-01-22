use leptos::*;
use leptos_router::*;
use leptos::prelude::*;

// components imports
use crate::components::navigation::app_sidebar::AppSidebar;
use crate::components::ui::breadcrumb::{Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator};
use crate::components::ui::separator::{Separator, SeparatorOrientation};
use crate::components::ui::sidebar::{SidebarInset, SidebarProvider, SidebarTrigger};
use crate::demo::DropdownMenuDemo;

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <SidebarProvider>
            <AppSidebar />
            <SidebarInset>
                <header class="flex h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-[[data-collapsible=icon]]/sidebar-wrapper:h-12">
                    <div class="flex items-center gap-2 px-4">
                        <SidebarTrigger class="-ml-1" />
                        <Separator orientation="vertical" class="mr-2 h-4" />
                        <Breadcrumb>
                            <BreadcrumbList>
                                <BreadcrumbItem class="hidden md:block">
                                    <BreadcrumbLink href="#">
                                        "Building Your Application"
                                    </BreadcrumbLink>
                                </BreadcrumbItem>
                                <BreadcrumbSeparator class="hidden md:block" />
                                <BreadcrumbItem>
                                    <BreadcrumbPage>"Data Fetching"</BreadcrumbPage>
                                </BreadcrumbItem>
                            </BreadcrumbList>
                        </Breadcrumb>
                    </div>
                </header>
                <div class="flex flex-1 flex-col gap-4 p-4 pt-0">
                    <DropdownMenuDemo/>
                    // <div class="grid auto-rows-min gap-4 md:grid-cols-3">
                    //     <div class="aspect-video rounded-xl bg-muted/50" />
                    //     <div class="aspect-video rounded-xl bg-muted/50" />
                    //     <div class="aspect-video rounded-xl bg-muted/50" />
                    // </div>
                    // <div class="min-h-[100vh] flex-1 rounded-xl bg-muted/50 md:min-h-min" />
                </div>
            </SidebarInset>
        </SidebarProvider>
    }
}