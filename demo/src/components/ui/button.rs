
use leptos::*;
use leptos::attr::{Attr, Attribute, AttributeKey, AttributeValue};
use leptos::html::{ElementType, HtmlElement, Main};
use leptos::prelude::*;
use tailwind_fuse::*;
use web_sys::MouseEvent;
use leptos_node_ref::AnyNodeRef;
use radix_leptos_primitive::{Primitive};

/// Core styles for the Button component using TwClass
#[derive(TwClass, PartialEq)]
#[tw(class = "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md \
    text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 \
    focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 \
    [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0")]
pub(crate) struct ButtonStyles {
    variant: ButtonVariant,
    size: ButtonSize,
}

#[derive(TwVariant, PartialEq)]
pub enum ButtonVariant {
    #[tw(default, class = "bg-primary text-primary-foreground shadow hover:bg-primary/90")]
    Default,
    #[tw(class = "bg-destructive text-destructive-foreground shadow-sm hover:bg-destructive/90")]
    Destructive,
    #[tw(
        class = "border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground"
    )]
    Outline,
    #[tw(class = "bg-secondary text-secondary-foreground shadow-sm hover:bg-secondary/80")]
    Secondary,
    #[tw(class = "hover:bg-accent hover:text-accent-foreground")]
    Ghost,
    #[tw(class = "text-primary underline-offset-4 hover:underline")]
    Link,
}

#[derive(TwVariant, PartialEq)]
pub enum ButtonSize {
    #[tw(default, class = "h-9 px-4 py-2")]
    Default,
    #[tw(class = "h-8 rounded-md px-3 text-xs")]
    Sm,
    #[tw(class = "h-10 rounded-md px-8")]
    Lg,
    #[tw(class = "h-9 w-9")]
    Icon,
}

#[component]
#[allow(non_snake_case)]
pub fn Button<C: IntoView + 'static>(
    /// Variant of the button (e.g., Default, Destructive)
    #[prop(optional, into)] variant: MaybeProp<ButtonVariant>,
    /// Size of the button (e.g., Default, Sm, Lg, Icon)
    #[prop(optional, into)] size: MaybeProp<ButtonSize>,
    /// Node reference for the button element
    #[prop(optional, into)] node_ref: AnyNodeRef,
    /// Whether the button is disabled
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    /// Whether the button is disabled
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Whether to render only the children without the wrapper element
    #[prop(optional, into)] as_child: MaybeProp<bool>,
    children: TypedChildrenFn<C>,
) -> impl IntoView {
    let class = Signal::derive(move || ButtonStyles::builder()
        .variant(variant.get().unwrap_or_default())
        .size(size.get().unwrap_or_default())
        .with_class(class.get().unwrap_or_default())
    );

    view! {
        <Primitive
            element=html::button
            children=children
            as_child=as_child
            node_ref=node_ref
            attr:disabled=move || disabled.get().unwrap_or_default()
            attr:class=move || class.get()
        />
    }
}
