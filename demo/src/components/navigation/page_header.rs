use leptos::prelude::*;
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use leptos::ev::MouseEvent;

#[derive(Clone)]
pub struct PageHeaderAction {
    pub label: String,
    pub icon: Option<String>,
    pub variant: ButtonVariant,
    pub on_click: Callback<MouseEvent>,
}

#[component]
#[allow(non_snake_case)]
fn PageHeaderButton(action: PageHeaderAction) -> impl IntoView {
    let action = StoredValue::new(action);
    let on_click = action.with_value(|x| x.clone()).on_click.clone();
    view! {
        <Button
            variant=action.with_value(|x| x.clone()).variant
            size=ButtonSize::Sm
            on:click=move |ev| {
                on_click.run(ev);
            }
            class="inline-flex items-center"
        >
            {action
                .with_value(|x| x.clone())
                .icon
                .map(|icon| view! { <span class="mr-2 leading-none">{icon}</span> })}
            <span class="leading-none">{action.with_value(|x| x.clone()).label}</span>
        </Button>
    }
}

#[component]
#[allow(non_snake_case)]
pub fn PageHeader(
    #[prop(into)] title: String,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] actions: Option<Vec<PageHeaderAction>>,
    children: ChildrenFn,
) -> impl IntoView {
    let actions_store = StoredValue::new(actions.unwrap_or_default());

    view! {
        <div class="sticky top-0 z-10 w-full border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
            <div class="py-5 px-6">
                <div class="mx-auto max-w-7xl">
                    <div class="flex flex-col gap-4 sm:flex-row sm:justify-between sm:items-center">
                        <div class="flex flex-col gap-1.5 min-w-0">
                            <h1 class="text-2xl font-semibold tracking-tight text-foreground">
                                {title}
                            </h1>
                            {description
                                .map(|desc| {
                                    view! {
                                        <p class="text-sm leading-relaxed text-muted-foreground line-clamp-1">
                                            {desc}
                                        </p>
                                    }
                                })}
                        </div>

                        <div class="flex gap-3 items-center md:gap-4">
                            {actions_store
                                .with_value(|actions| {
                                    actions
                                        .iter()
                                        .cloned()
                                        .map(|action| {
                                            view! { <PageHeaderButton action=action /> }
                                        })
                                        .collect_view()
                                })} {children()}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}