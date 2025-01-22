// Standard library imports
use std::rc::Rc;

// Main Leptos imports
use leptos::prelude::*;
use leptos::{html::{Li, Ul}};

// Third-party dependencies
use codee::string::FromToStringCodec;
use derive_more::Display;
use leptos_use::{use_cookie_with_options, SameSite, UseCookieOptions};
use radix_leptos_primitive::Primitive;
use serde::{Deserialize, Serialize};
use tailwind_fuse::*;

// Local crate imports
use crate::components::hooks::use_is_mobile;
use crate::components::hooks::use_keypress::{use_keypress, Key};

/// Name of the cookie used to store sidebar state
pub const SIDEBAR_COOKIE_NAME: &str = "sidebar:state";

/// Maximum age of the sidebar state cookie in seconds (7 days)
pub const SIDEBAR_COOKIE_MAX_AGE: u32 = 60 * 60 * 24 * 7;

/// Default sidebar width for desktop view
pub const SIDEBAR_WIDTH: &str = "16rem";

/// Sidebar width for mobile devices
pub const SIDEBAR_WIDTH_MOBILE: &str = "18rem";

/// Width of the sidebar when collapsed to icon-only mode
pub const SIDEBAR_WIDTH_ICON: &str = "3rem";

/// Keyboard shortcut key to toggle the sidebar
pub const SIDEBAR_KEYBOARD_SHORTCUT: char = 'b';



#[derive(Debug, Clone, Copy, PartialEq, Display)]
pub enum SidebarState {
    #[display("expanded")]
    Expanded,
    #[display("collapsed")]
    Collapsed,
}

#[derive(Clone)]
pub struct SidebarContext {
    pub state: Signal<SidebarState>,
    pub open: Signal<bool>,
    pub set_open: Callback<bool>,
    pub open_mobile: RwSignal<bool>,
    pub set_open_mobile: Callback<bool>,
    pub is_mobile: Signal<bool>,
    pub toggle_sidebar: Callback<()>,
}

pub fn use_sidebar() -> SidebarContext {
    use_context::<SidebarContext>().expect("sidebar context not provided")
}

#[component]
pub fn SidebarProvider(
    #[prop(default = true, into)] default_open: bool,
    #[prop(optional)] open: Option<Signal<bool>>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional)] class: Option<&'static str>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let is_mobile = use_is_mobile();
    let (sidebar_cookie, set_sidebar_cookie) = use_cookie_with_options::<bool, FromToStringCodec>(
        SIDEBAR_COOKIE_NAME,
        UseCookieOptions::default()
            .max_age(SIDEBAR_COOKIE_MAX_AGE)
            .same_site(SameSite::Lax),
    );

    // Internal state management, used when internally controlled
    let (internal_open, set_internal_open) = signal(sidebar_cookie.get().unwrap_or(default_open));

    // Default initial mobile is always closed
    let open_mobile = RwSignal::new(false);

    // Create a derived signal that reads from either the provided open signal or internal state
    let effective_open = Signal::derive(move || match open {
        Some(external_open) => external_open.get(),
        None => internal_open.get(),
    });

    // Set up the set callback for external or internal control
    let set_open = Callback::new(move |value: bool| {
        if let Some(callback) = on_open_change.as_ref() {
            callback.run(value);
        } else {
            set_internal_open.set(value);
        }
        set_sidebar_cookie.set(Option::from(value));
    });

    // Toggle sidebar handler
    let toggle_sidebar = Callback::new(move |_| {
        logging::log!("Called toggle");
        if is_mobile.get() {
            logging::log!("is_mobile.get");
            open_mobile.update(|open| *open = !*open);
        } else {
            logging::log!("a");
            set_open.run(!effective_open.get());
        }
    });

    // Ctrl+<shortcut> to toggle sidebar
    use_keypress(
        Key::Character(SIDEBAR_KEYBOARD_SHORTCUT).with_ctrl(),
        move || {
            toggle_sidebar.run(());
        },
    );

    // Create context value
    let state = Signal::derive(move || {
        if effective_open.get() {
            SidebarState::Expanded
        } else {
            SidebarState::Collapsed
        }
    });

    let context = SidebarContext {
        state: state.into(),
        open: effective_open,
        set_open: Callback::from(set_open),
        open_mobile: open_mobile.into(),
        set_open_mobile: Callback::new(move |value| open_mobile.set(value)),
        is_mobile: is_mobile.into(),
        toggle_sidebar: toggle_sidebar.into(),
    };

    provide_context(context);
    let class = cn!(
        "group/sidebar-wrapper flex min-h-svh w-full has-[[data-variant=inset]]:bg-sidebar",
        class
    );
    let style = StoredValue::new(style);
    view! {
        // <TooltipProvider>
        <div
            class=class
            style=move || {
                format!(
                    "--sidebar-width: {}; --sidebar-width-icon: {};{}",
                    SIDEBAR_WIDTH,
                    SIDEBAR_WIDTH_ICON,
                    style.with_value(|x| x.clone().unwrap_or_default()),
                )
            }
        >
            {children()}
        </div>
    }
}

use crate::components::ui::sheet::{Sheet, SheetContent, SheetSide};
use leptos::*;
use leptos_router::components::A;
use tailwind_fuse::*;

#[derive(TwVariant, Serialize, Display, PartialEq)]
pub enum SidebarVariant {
    #[tw(
        default,
        class = "group-data-[side=left]:border-r group-data-[side=right]:border-l bg-sidebar"
    )]
    #[display("sidebar")]
    Sidebar,
    #[tw(class = "p-2 \
        group-data-[variant=floating]:rounded-lg \
        group-data-[variant=floating]:border \
        group-data-[variant=floating]:border-sidebar-border \
        group-data-[variant=floating]:shadow")]
    #[display("floating")]
    Floating,
    #[tw(class = "p-2 bg-sidebar")]
    #[display("inset")]
    Inset,
}

#[derive(TwVariant, Serialize, Display, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SidebarSide {
    #[tw(
        default,
        class = "left-0 group-data-[collapsible=offcanvas]:left-[calc(var(--sidebar-width)*-1)]"
    )]
    #[display("left")]
    Left,
    #[tw(
        class = "right-0 group-data-[collapsible=offcanvas]:right-[calc(var(--sidebar-width)*-1)]"
    )]
    #[display("right")]
    Right,
}

#[derive(TwVariant, Display, PartialEq)]
pub enum SidebarCollapsible {
    #[tw(default, class = "data-[state=collapsed]:w-0")]
    #[display("offcanvas")]
    Offcanvas,
    #[tw(class = "data-[state=collapsed]:group-data-[collapsible=icon]:w-[--sidebar-width-icon]")]
    #[display("icon")]
    Icon,
    #[tw(class = "")]
    #[display("none")]
    None,
}

#[derive(TwClass)]
#[tw(class = "group peer hidden md:block text-sidebar-foreground")]
pub(crate) struct SidebarStyles {
    variant: SidebarVariant,
    side: SidebarSide,
    collapsible: SidebarCollapsible,
}

use leptos::html::Div;

#[component]
pub fn Sidebar(
    #[prop(optional, into)] class: String,
    #[prop(optional, default=SidebarSide::Left)] side: SidebarSide,
    #[prop(optional, default=SidebarVariant::Sidebar)] variant: SidebarVariant,
    #[prop(optional, default=SidebarCollapsible::Offcanvas)] collapsible: SidebarCollapsible,
    #[prop(optional)] node_ref: Option<NodeRef<Div>>,
    children: ChildrenFn,
) -> impl IntoView {
    let SidebarContext {
        state,
        is_mobile,
        open_mobile,
        set_open_mobile,
        ..
    } = use_sidebar();

    let side = StoredValue::new(side);
    let variant = StoredValue::new(variant);
    let collapsible = StoredValue::new(collapsible);
    let node_ref = node_ref.unwrap_or(NodeRef::new());

    let base_styles = StoredValue::new(
        SidebarStyles::builder()
            .variant(variant.with_value(|v| v.clone()))
            .side(side.with_value(|s| s.clone()))
            .collapsible(collapsible.with_value(|c| c.clone()))
            .with_class(class),
    );

    let gap_styles = StoredValue::new(
        cn!("duration-200 relative h-svh w-[--sidebar-width] bg-transparent \
        transition-[width] ease-linear \
        group-data-[state=collapsed]:group-data-[collapsible=icon]:w-[--sidebar-width-icon] \
        group-data-[collapsible=offcanvas]:w-0 \
        group-data-[side=right]:rotate-180 {}",
            match variant.with_value(|v| v.clone()) {
                SidebarVariant::Floating | SidebarVariant::Inset =>
                    "group-data-[state=collapsed]:group-data-[collapsible=icon]:w-[calc(var(--sidebar-width-icon)_+_theme(spacing.4))]",
                _ => ""
            }
    )
    );
    let container_styles = StoredValue::new(
        cn!("duration-200 fixed inset-y-0 z-10 hidden h-svh w-[--sidebar-width] \
    transition-[left,right,width] ease-linear md:flex \
    group-data-[state=collapsed]:group-data-[collapsible=icon]:w-[--sidebar-width-icon] {}",
        match variant.with_value(|v| v.clone()) {
            SidebarVariant::Floating | SidebarVariant::Inset =>
                "p-2 group-data-[state=collapsed]:group-data-[collapsible=icon]:w-[calc(var(--sidebar-width-icon)_+_theme(spacing.4)_+2px)]",
            _ => "group-data-[side=left]:border-r group-data-[side=right]:border-l"
        }
    )
    );


    let content_styles = StoredValue::new(
        "flex h-full w-full flex-col bg-sidebar \
        group-data-[variant=floating]:rounded-lg \
        group-data-[variant=floating]:border \
        group-data-[variant=floating]:border-sidebar-border \
        group-data-[variant=floating]:shadow",
    );

    let sheet_styles = StoredValue::new(
        "w-[--sidebar-width] bg-sidebar p-0 text-sidebar-foreground [&>button]:hidden",
    );

    let children = StoredValue::new(children);

    // Use <Show> to handle the three main conditions:
    // 1. Collapsible = none
    // 2. Mobile mode
    // 3. Desktop mode

    view! {
        <Show
            // If collapsible is not "none", show nested <Show>, else fallback to the simple sidebar
            when=move || collapsible.with_value(|c| c.clone()) != SidebarCollapsible::None
            fallback=move || {
                // Collapsible = none: simple sidebar
                view! {
                    <div class=content_styles.with_value(|s| s.clone()) node_ref=node_ref>
                        {children.with_value(|c| c())}
                    </div>
                }
            }
        >
            <Show
                // If mobile, show Sheet version; else fallback to desktop
                when=move || is_mobile.get()
                fallback=move || {
                    // Desktop sidebar
                    view! {
                        <div
                            class=base_styles.with_value(|s| s.clone())
                            data-state=move || state.get().to_string()
                            data-collapsible=move || {
                                if state.get() == SidebarState::Collapsed {
                                    Some(collapsible.with_value(|c| c.to_string()))
                                } else {
                                    None
                                }
                            }
                            data-variant=move || variant.with_value(|v| v.to_string())
                            data-side=move || side.with_value(|s| s.to_string())
                        >
                            <div class=gap_styles.with_value(|s| s.clone()) />
                            <div class=container_styles.with_value(|s| s.clone()) node_ref=node_ref>
                                <div
                                    class=content_styles.with_value(|s| s.clone())
                                    data-sidebar="sidebar"
                                >
                                    {children.with_value(|c| c())}
                                </div>
                            </div>
                        </div>
                    }
                }
            >
                // Mobile sidebar with Sheet
                <Sheet open=open_mobile on_open_change=set_open_mobile>
                    <SheetContent
                        class=sheet_styles.with_value(|x| x.to_string())
                        attr:data-sidebar="sidebar"
                        attr:data-mobile="true"
                        attr:style=format!("--sidebar-width: {}", SIDEBAR_WIDTH_MOBILE)
                        side=match side.with_value(|s| s.clone()) {
                            SidebarSide::Left => SheetSide::Left,
                            SidebarSide::Right => SheetSide::Right,
                        }
                    >
                        <div class=content_styles
                            .with_value(|x| *x)>{children.with_value(|c| c())}</div>
                    </SheetContent>
                </Sheet>
            </Show>
        </Show>
    }
}

// #[component]
// pub fn Sidebar(
//     #[prop(optional, into)] class: String,
//     #[prop(optional, default=SidebarSide::Left)] side: SidebarSide,
//     #[prop(optional, default=SidebarVariant::Sidebar)] variant: SidebarVariant,
//     #[prop(optional, default=SidebarCollapsible::Offcanvas)] collapsible: SidebarCollapsible,
//     #[prop(optional)] node_ref: Option<NodeRef<Div>>,
//     children: ChildrenFn,
// ) -> impl IntoView {
//     let SidebarContext {
//         state,
//         is_mobile,
//         open_mobile,
//         set_open_mobile,
//         ..
//     } = use_sidebar();
//
//     let side = StoredValue::new(side);
//     let variant = StoredValue::new(variant);
//     let collapsible = StoredValue::new(collapsible);
//     let node_ref = node_ref.unwrap_or(NodeRef::new());
//
//     let children = StoredValue::new(children);
//
//     view! {
//       <Show
//         when=move || collapsible.with_value(|c| c.clone()) != SidebarCollapsible::None
//         fallback=move || {
//           view! {
//             <div
//               node_ref={node_ref}
//               {..}
//               class=(
//                 [
//                   "flex",
//                   "h-full",
//                   "w-full",
//                   "flex-col",
//                   "bg-sidebar",
//                   "group-data-[variant=floating]:rounded-lg",
//                   "group-data-[variant=floating]:border",
//                   "group-data-[variant=floating]:border-sidebar-border",
//                   "group-data-[variant=floating]:shadow",
//                 ],
//                 move || true,
//               )
//             >
//               {children.with_value(|c| c())}
//             </div>
//           }
//         }
//       >
//         <Show
//           when=move || is_mobile.get()
//           fallback=move || {
//             view! {
//               <div
//                 data-state=move || state.get().to_string()
//                 data-collapsible=move || {
//                   if state.get() == SidebarState::Collapsed {
//                     Some(collapsible.with_value(|c| c.to_string()))
//                   } else {
//                     None
//                   }
//                 }
//                 data-variant=move || variant.with_value(|v| v.to_string())
//                 data-side=move || side.with_value(|s| s.to_string())
//               >
//                 <div
//                   class=(
//                     [
//                       "duration-200",
//                       "relative",
//                       "h-svh",
//                       "w-[--sidebar-width]",
//                       "bg-transparent",
//                       "transition-[width]",
//                       "ease-linear",
//                       "group-data-[state=collapsed]:group-data-[collapsible=icon]:w-[--sidebar-width-icon]",
//                       "group-data-[collapsible=offcanvas]:w-0",
//                       "group-data-[side=right]:rotate-180",
//                     ],
//                     move || true,
//                   )
//                   class=(
//                     "group-data-[state=collapsed]:group-data-[collapsible=icon]:w-[calc(var(--sidebar-width-icon)_+_theme(spacing.4))]",
//                     match variant.with_value(|v| v.clone()) {
//                       SidebarVariant::Floating | SidebarVariant::Inset => true,
//                       _ => false,
//                     },
//                   )
//                 />
//                 <div
//                   node_ref={node_ref}
//                   {..}
//                   class=(
//                     [
//                       "duration-200",
//                       "fixed",
//                       "inset-y-0",
//                       "z-10",
//                       "hidden",
//                       "h-svh",
//                       "w-[--sidebar-width]",
//                       "transition-[left,right,width]",
//                       "ease-linear",
//                       "md:flex",
//                       "group-data-[state=collapsed]:group-data-[collapsible=icon]:w-[--sidebar-width-icon]",
//                     ],
//                     true,
//                   )
//                   class=(
//                     move || match variant.with_value(|v| v.clone()) {
//                       SidebarVariant::Floating | SidebarVariant::Inset => {
//                         "p-2 group-data-[state=collapsed]:group-data-[collapsible=icon]:w-[calc(var(--sidebar-width-icon)_+_theme(spacing.4)_+2px)]"
//                       }
//                       _ => "group-data-[side=left]:border-r group-data-[side=right]:border-l",
//                     },
//                     true,
//                   )
//                 >
//                   <div
//                     data-sidebar="sidebar"
//                     class=(
//                       [
//                         "flex",
//                         "h-full",
//                         "w-full",
//                         "flex-col",
//                         "bg-sidebar",
//                         "group-data-[variant=floating]:rounded-lg",
//                         "group-data-[variant=floating]:border",
//                         "group-data-[variant=floating]:border-sidebar-border",
//                         "group-data-[variant=floating]:shadow",
//                       ],
//                       true,
//                     )
//                   >
//                     {children.with_value(|c| c())}
//                   </div>
//                 </div>
//               </div>
//             }
//           }
//         >
//           <Sheet open=open_mobile on_open_change=set_open_mobile>
//             <SheetContent
//               attr:data-sidebar="sidebar"
//               attr:data-mobile="true"
//               attr:style=format!("--sidebar-width: {}", SIDEBAR_WIDTH_MOBILE)
//               side=match side.with_value(|s| s.clone()) {
//                 SidebarSide::Left => SheetSide::Left,
//                 SidebarSide::Right => SheetSide::Right,
//               }
//               {..}
//               class=(
//                 ["w-[--sidebar-width]", "bg-sidebar", "p-0", "text-sidebar-foreground", "[&>button]:hidden"],
//                 move || true,
//               )
//             >
//               <div
//                 {..}
//                 class=(
//                   [
//                     "flex",
//                     "h-full",
//                     "w-full",
//                     "flex-col",
//                     "bg-sidebar",
//                     "group-data-[variant=floating]:rounded-lg",
//                     "group-data-[variant=floating]:border",
//                     "group-data-[variant=floating]:border-sidebar-border",
//                     "group-data-[variant=floating]:shadow",
//                   ],
//                   move || true,
//                 )
//               >
//                 {children.with_value(|c| c())}
//               </div>
//             </SheetContent>
//           </Sheet>
//         </Show>
//       </Show>
//     }
// }

#[component]
pub fn SidebarContent(
    /// Optional CSS classes
    #[prop(optional, into)]
    class: String,
    /// Optional reference to the div element
    #[prop(optional)]
    node_ref: NodeRef<html::Div>,
    /// Child elements to render
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            data-sidebar="content"
            class=cn!(
                "flex min-h-0 flex-1 flex-col gap-2 overflow-auto group-data-[collapsible=icon]:overflow-hidden",
                class
            )
        >
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarRail(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: Option<NodeRef<html::Button>>,
) -> impl IntoView {
    let ctx = use_sidebar();
    let node_ref = node_ref.unwrap_or_else(|| NodeRef::new());

    view! {
        <button
            node_ref=node_ref
            data-sidebar="rail"
            aria-label="Toggle Sidebar"
            tabindex=-1
            on:click=move |_| ctx.toggle_sidebar.run(())
            title="Toggle Sidebar"
            class=cn!(
                "absolute inset-y-0 z-20 hidden w-4 -translate-x-1/2 transition-all ease-linear",
                "after:absolute after:inset-y-0 after:left-1/2 after:w-[2px]",
                "hover:after:bg-sidebar-border group-data-[side=left]:-right-4 group-data-[side=right]:right-0 sm:flex", // Changed here

                // Cursor styles based on side and state
                "[[data-side=left]_&]:cursor-w-resize [[data-side=right]_&]:cursor-e-resize",
                "[[data-side=left][data-state=collapsed]_&]:cursor-e-resize [[data-side=right][data-state=collapsed]_&]:cursor-w-resize",

                // Offcanvas specific styles
                "group-data-[collapsible=offcanvas]:translate-x-0",
                "group-data-[collapsible=offcanvas]:after:left-full",
                "group-data-[collapsible=offcanvas]:hover:bg-sidebar",

                // Positioning
                "[[data-side=left][data-collapsible=offcanvas]_&]:-right-2",
                "[[data-side=right][data-collapsible=offcanvas]_&]:right-2", // Intended for right side

                // Additional classes
                class
            )
        />
    }
}


/// Core styles for the SidebarMenuButton using TwClass
#[derive(TwClass)]
#[tw(
    class = "peer/menu-button flex w-full items-center gap-2 overflow-hidden rounded-md p-2 \
    text-left text-sm outline-none ring-sidebar-ring transition-[width,height,padding] \
    hover:bg-sidebar-accent hover:text-sidebar-accent-foreground focus-visible:ring-2 \
    active:bg-sidebar-accent active:text-sidebar-accent-foreground disabled:pointer-events-none \
    disabled:opacity-50 group-has-[[data-sidebar=menu-action]]/menu-item:pr-8 \
    aria-disabled:pointer-events-none aria-disabled:opacity-50 data-[active=true]:bg-sidebar-accent \
    data-[active=true]:font-medium data-[active=true]:text-sidebar-accent-foreground \
    data-[state=open]:hover:bg-sidebar-accent data-[state=open]:hover:text-sidebar-accent-foreground \
    group-data-[collapsible=icon]:!size-8 group-data-[collapsible=icon]:!p-2 \
    [&>span:last-child]:truncate [&>svg]:size-4 [&>svg]:shrink-0"
)]
pub(crate) struct SidebarMenuButtonStyles {
    variant: SidebarMenuButtonVariant,
    size: SidebarMenuButtonSize,
}

#[derive(TwVariant, Display)]
pub enum SidebarMenuButtonVariant {
    #[tw(
        default,
        class = "hover:bg-sidebar-accent hover:text-sidebar-accent-foreground"
    )]
    #[display("default")]
    Default,
    #[tw(class = "bg-background shadow-[0_0_0_1px_hsl(var(--sidebar-border))] \
                  hover:bg-sidebar-accent hover:text-sidebar-accent-foreground \
                  hover:shadow-[0_0_0_1px_hsl(var(--sidebar-accent))]")]
    #[display("outline")]
    Outline,
}

#[derive(TwVariant, Display)]
pub enum SidebarMenuButtonSize {
    #[tw(default, class = "h-8 text-sm")]
    #[display("default")]
    Default,
    #[tw(class = "h-7 text-xs")]
    #[display("sm")]
    Sm,
    #[tw(class = "h-12 text-sm")]
    #[display("lg")]
    Lg,
}

#[component]
pub fn SidebarMenuButton(
    /// Additional CSS classes
    #[prop(into, optional)]
    class: MaybeProp<String>,
    /// Button variant
    #[prop(optional)]
    variant: Option<SidebarMenuButtonVariant>,
    /// Button size
    #[prop(optional)]
    size: Option<SidebarMenuButtonSize>,
    /// Whether the button is active
    #[prop(into, optional)]
    is_active: MaybeProp<bool>,
    /// Whether to render only children without wrapper
    #[prop(optional, into, default=false.into())]
    as_child: MaybeProp<bool>,
    /// Optional tooltip content
    #[prop(optional)]
    tooltip: Option<String>,
    /// Node reference
    #[prop(into, optional)]
    node_ref: AnyNodeRef,
    /// Click event handler
    #[prop(into, optional)]
    on_click: MaybeCallback<ev::MouseEvent>,
    /// Children elements
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let class = Signal::derive(move ||
        SidebarMenuButtonStyles::builder()
            .variant(variant.unwrap_or_default())
            .size(size.unwrap_or_default())
            .with_class(class.get().unwrap_or_default()),
    );
    view! {
        <Primitive
            element=html::button
            as_child=as_child
            children=children
            node_ref={node_ref}
            {..}
            class=move || class.get()
            data-sidebar="menu-button"
            data-active=move || is_active.get().unwrap_or_default().to_string()
            on:click=on_click.into_handler()
        />
    }

    // // Return early if no tooltip
    // if tooltip.is_none() {
    //     return button();
    // }
    //
    // // Wrap with tooltip if provided
    // let sidebar = use_sidebar();
    //
    // view! {
    //     <Tooltip>
    //         <TooltipTrigger as_child=true>
    //             {button()}
    //         </TooltipTrigger>
    //         {move || match &tooltip {
    //             Some(TooltipProp::String(text)) => view! {
    //                 <TooltipContent
    //                     side="right"
    //                     align="center"
    //                     hidden=sidebar.state.get() != "collapsed" || sidebar.is_mobile.get()
    //                 >
    //                     {text}
    //                 </TooltipContent>
    //             },
    //             Some(TooltipProp::Props(props)) => view! {
    //                 <TooltipContent
    //                     ..props.clone()
    //                     hidden=props.hidden.unwrap_or_else(||
    //                         sidebar.state.get() != "collapsed" || sidebar.is_mobile.get()
    //                     )
    //                 />
    //             },
    //             None => view! { <></> }
    //         }}
    //     </Tooltip>
    // }
}

#[component]
pub fn SidebarMenu(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
    /// Node reference for the ul element
    #[prop(optional)]
    node_ref: Option<NodeRef<Ul>>,
    /// Children elements
    children: Children,
) -> impl IntoView {
    view! {
        <ul
            class=cn!("flex w-full min-w-0 flex-col gap-1", class)
            data-sidebar="menu"
            node_ref=node_ref.unwrap_or(NodeRef::new())
        >
            {children()}
        </ul>
    }
}

#[component]
#[allow(non_snake_case)]
pub fn SidebarMenuItem(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
    /// Node reference for the li element
    #[prop(optional)]
    node_ref: Option<NodeRef<Li>>,
    /// Children elements
    children: Children,
) -> impl IntoView {
    view! {
        <li
            class=cn!("group/menu-item relative", class)
            data-sidebar="menu-item"
            node_ref=node_ref.unwrap_or(NodeRef::new())
        >
            {children()}
        </li>
    }
}

#[component]
#[allow(non_snake_case)]
pub fn SidebarGroup(
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional)] node_ref: NodeRef<html::Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            data-sidebar="group"
            class=cn!("relative flex w-full min-w-0 flex-col p-2", class)
        >
            {children()}
        </div>
    }
}


#[component]
#[allow(non_snake_case)]
pub fn SidebarGroupLabel<C: IntoView + 'static>(
    #[prop(optional, into)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: NodeRef<html::Div>,
    children: TypedChildrenFn<C>,
) -> impl IntoView {
    view! {
        <Primitive
            element=html::div
            children=children
            as_child=as_child
            node_ref=node_ref
            attr:data-sidebar="group-label"
            attr:class=cn!(
                "duration-200",
                "flex",
                "h-8",
                "shrink-0",
                "items-center",
                "rounded-md",
                "px-2",
                "text-xs",
                "font-medium",
                "text-sidebar-foreground/70",
                "outline-none",
                "ring-sidebar-ring",
                "transition-[margin,opa]",
                "ease-linear",
                "focus-visible:ring-2",
                "[&>svg]:size-4",
                "[&>svg]:shrink-0",
                "group-data-[collapsible=icon]:-mt-8",
                "group-data-[collapsible=icon]:opacity-0"
            )
            {..}
        />
    }
}

// #[component]
// #[allow(non_snake_case)]
// pub fn SidebarGroupLabel<C: IntoView + 'static>(
//     #[prop(optional, into)] as_child: MaybeProp<bool>,
//     #[prop(optional)] node_ref: NodeRef<html::Div>,
//     children: TypedChildrenFn<C>,
// ) -> impl IntoView {
//     view! {
//         <Primitive
//             element=html::div
//             children=children
//             as_child=as_child
//             node_ref=node_ref
//             attr:data-sidebar="group-label"
//             {..}
//             class=("duration-200", true)
//             class=("flex", true)
//             class=("h-8", true)
//             class=("shrink-0", true)
//             class=("items-center", true)
//             class=("rounded-md", true)
//             class=("px-2", true)
//             class=("text-xs", true)
//             class=("font-medium", true)
//             class=("text-sidebar-foreground/70", true)
//             class=("outline-none", true)
//             class=("ring-sidebar-ring", true)
//             class=("transition-[margin,opa]", true)
//             class=("ease-linear", true)
//             class=("focus-visible:ring-2", true)
//             class=("[&>svg]:size-4", true)
//             class=("[&>svg]:shrink-0", true)
//             class=("group-data-[collapsible=icon]:-mt-8", true)
//             class=("group-data-[collapsible=icon]:opacity-0", true)
//         />
//     }
// }

#[component]
pub fn SidebarMenuSub(
    #[prop(default = "")] class: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <ul
            data-sidebar="menu-sub"
            class=cn!(
                "mx-3.5 flex min-w-0 translate-x-px flex-col gap-1 border-l border-sidebar-border px-2.5 py-0.5",
                "group-data-[collapsible=icon]:hidden",
                class
            )
        >
            {children()}
        </ul>
    }
}

#[component]
pub fn SidebarMenuSubItem(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! { <li class=cn!(&class)>{children()}</li> }
}

// Size type to match React's "sm" | "md"
#[derive(Clone, Copy, PartialEq, Default)]
pub enum SidebarMenuSubButtonSize {
    #[default]
    Md,
    Sm,
}

impl std::fmt::Display for SidebarMenuSubButtonSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Md => write!(f, "md"),
            Self::Sm => write!(f, "sm"),
        }
    }
}

impl IntoAttributeValue for SidebarMenuSubButtonSize {
    type Output = String;

    fn into_attribute_value(self) -> Self::Output {
        self.to_string()
    }
}

// Also implement for references
impl IntoAttributeValue for &SidebarMenuSubButtonSize {
    type Output = String;

    fn into_attribute_value(self) -> Self::Output {
        (*self).to_string()
    }
}

// #[component]
// #[allow(non_snake_case)]
// pub fn SidebarMenuSubButton<C: IntoView + 'static>(
//     children: TypedChildrenFn<C>,
//     #[prop(optional, into)] as_child: MaybeProp<bool>,
//     #[prop(optional)] size: MaybeProp<SidebarMenuSubButtonSize>,
//     #[prop(optional)] is_active: MaybeProp<bool>,
//     #[prop(optional)] node_ref: AnyNodeRef,
// ) -> impl IntoView {
//     let size = Signal::derive(move || size.get().unwrap_or_default());
//     let is_active = Signal::derive(move || is_active.get().unwrap_or_default());
//
//     view! {
//         <Primitive
//             element=html::a
//             children=children
//             as_child=as_child
//             node_ref=node_ref
//             // Data attributes
//             attr:data-sidebar="menu-sub-button"
//             attr:data-size=move || size.get().to_string()
//             attr:data-active=move || is_active.get().to_string()
//             {..}
//             // Base classes
//             class=("flex", true)
//             class=("h-7", true)
//             class=("min-w-0", true)
//             class=("-translate-x-px", true)
//             class=("items-center", true)
//             class=("gap-2", true)
//             class=("overflow-hidden", true)
//             class=("rounded-md", true)
//             class=("px-2", true)
//             class=("text-sidebar-foreground", true)
//             class=("outline-none", true)
//             class=("ring-sidebar-ring", true)
//
//             // Interactive states
//             class=("hover:bg-sidebar-accent", true)
//             class=("hover:text-sidebar-accent-foreground", true)
//             class=("focus-visible:ring-2", true)
//             class=("active:bg-sidebar-accent", true)
//             class=("active:text-sidebar-accent-foreground", true)
//
//             // Disabled states
//             class=("disabled:pointer-events-none", true)
//             class=("disabled:opacity-50", true)
//             class=("aria-disabled:pointer-events-none", true)
//             class=("aria-disabled:opacity-50", true)
//
//             // Child element styles
//             class=("[&>span:last-child]:truncate", true)
//             class=("[&>svg]:size-4", true)
//             class=("[&>svg]:shrink-0", true)
//             class=("[&>svg]:text-sidebar-accent-foreground", true)
//
//             // Active state
//             class=("data-[active=true]:bg-sidebar-accent", true)
//             class=("data-[active=true]:text-sidebar-accent-foreground", true)
//
//             // Size variants - exactly matching React's conditions
//             class=("text-xs", move || size.get() == SidebarMenuSubButtonSize::Sm)
//             class=("text-sm", move || size.get() == SidebarMenuSubButtonSize::Md)
//
//             // Collapsible state
//             class=("group-data-[collapsible=icon]:hidden", true)
//         />
//     }
// }
use leptos::prelude::*;
use leptos::html;
use leptos_maybe_callback::MaybeCallback;

#[component]
#[allow(non_snake_case)]
pub fn SidebarMenuSubButton<C: IntoView + 'static>(
    children: TypedChildrenFn<C>,
    as_child: bool,
    #[prop(optional)] size: MaybeProp<SidebarMenuSubButtonSize>,
    #[prop(optional)] is_active: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let size = Signal::derive(move || size.get().unwrap_or_default());
    let is_active = Signal::derive(move || is_active.get().unwrap_or_default());

    view! {
        <Primitive
            element=html::a
            children=children
            as_child=as_child
            node_ref=node_ref
            // Data attributes
            attr:data-sidebar="menu-sub-button"
            attr:data-size=move || size.get().to_string()
            attr:data-active=move || is_active.get().to_string()

            // Static classes (predicate = true)
            attr:class=cn!(
                "flex",
                "h-7",
                "min-w-0",
                "-translate-x-px",
                "items-center",
                "gap-2",
                "overflow-hidden",
                "rounded-md",
                "px-2",
                "text-sidebar-foreground",
                "outline-none",
                "ring-sidebar-ring",
                "hover:bg-sidebar-accent",
                "hover:text-sidebar-accent-foreground",
                "focus-visible:ring-2",
                "active:bg-sidebar-accent",
                "active:text-sidebar-accent-foreground",
                "disabled:pointer-events-none",
                "disabled:opacity-50",
                "aria-disabled:pointer-events-none",
                "aria-disabled:opacity-50",
                "[&>span:last-child]:truncate",
                "[&>svg]:size-4",
                "[&>svg]:shrink-0",
                "[&>svg]:text-sidebar-accent-foreground",
                "data-[active=true]:bg-sidebar-accent",
                "data-[active=true]:text-sidebar-accent-foreground",
                "group-data-[collapsible=icon]:hidden"
            )
            {..}
            // class=([
            // "flex",
            // "h-7",
            // "min-w-0",
            // "-translate-x-px",
            // "items-center",
            // "gap-2",
            // "overflow-hidden",
            // "rounded-md",
            // "px-2",
            // "text-sidebar-foreground",
            // "outline-none",
            // "ring-sidebar-ring",
            // "hover:bg-sidebar-accent",
            // "hover:text-sidebar-accent-foreground",
            // "focus-visible:ring-2",
            // "active:bg-sidebar-accent",
            // "active:text-sidebar-accent-foreground",
            // "disabled:pointer-events-none",
            // "disabled:opacity-50",
            // "aria-disabled:pointer-events-none",
            // "aria-disabled:opacity-50",
            // "[&>span:last-child]:truncate",
            // "[&>svg]:size-4",
            // "[&>svg]:shrink-0",
            // "[&>svg]:text-sidebar-accent-foreground",
            // "data-[active=true]:bg-sidebar-accent",
            // "data-[active=true]:text-sidebar-accent-foreground",
            // "group-data-[collapsible=icon]:hidden"
            // ], move || true)

            // Size variant classes - Sm
            class=("text-xs", move || size.get() == SidebarMenuSubButtonSize::Sm)

            // Size variant classes - Md
            class=("text-sm", move || size.get() == SidebarMenuSubButtonSize::Md)
        />
    }
}

#[component]
#[allow(non_snake_case)]
pub fn SidebarHeader(
    // [x]
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<html::Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref data-sidebar="header" class=cn!("flex flex-col gap-2 p-2", class)>
            {children()}
        </div>
    }
}

#[component]
#[allow(non_snake_case)]
pub fn SidebarFooter(
    // [x]
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<html::Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref data-sidebar="footer" class=cn!("flex flex-col gap-2 p-2", class)>
            {children()}
        </div>
    }
}

use lucide_leptos::PanelLeft;
use leptos_node_ref::AnyNodeRef;
use crate::cn;
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};

#[component]
#[allow(non_snake_case)]
pub fn SidebarTrigger(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] node_ref: NodeRef<html::Button>,
    #[prop(optional, default=Callback::new(|_|{}))] on_click: Callback<ev::MouseEvent>,
    //children: ChildrenFn,
) -> impl IntoView {
    let SidebarContext { toggle_sidebar, .. } = use_sidebar();
    let on_click = Callback::new(move |ev: ev::MouseEvent| {
        toggle_sidebar.run(());
        on_click.run(ev);
    });
    view! {
        <Button
            node_ref=node_ref
            // class=cn!("h-7 w-7", &class)
            variant=ButtonVariant::Ghost
            size=ButtonSize::Icon
            attr:data-sidebar="trigger"
            on:click=move |e| on_click.run(e)
            {..}
            class=("h-7 w-7", move || true)
        >
            <PanelLeft />
            <span class="sr-only">"Toggle Sidebar"</span>
        </Button>
    }
}

#[component]
#[allow(non_snake_case)]
pub fn SidebarInset(
    /// Optional CSS classes to apply to the main element
    #[prop(optional, into)]
    class: String,
    /// Optional reference to forward
    #[prop(optional)]
    node_ref: NodeRef<html::Main>,
    /// Child elements to render inside the main element
    children: Children,
) -> impl IntoView {
    let class = tw_merge!(
            "relative flex min-h-svh flex-1 flex-col bg-background",
            "peer-data-[variant=inset]:min-h-[calc(100svh-theme(spacing.4))] md:peer-data-[variant=inset]:m-2 md:peer-data-[state=collapsed]:peer-data-[variant=inset]:ml-2 md:peer-data-[variant=inset]:ml-0 md:peer-data-[variant=inset]:rounded-xl md:peer-data-[variant=inset]:shadow",
            &class
        );

    view! {
        <main class=class node_ref=node_ref>
            {children()}
        </main>
    }
}