use leptos::prelude::*;
use crate::cn;

#[derive(Clone, Debug, Default, PartialEq, Copy)]
pub enum SeparatorOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl std::fmt::Display for SeparatorOrientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Horizontal => write!(f, "horizontal"),
            Self::Vertical => write!(f, "vertical"),
        }
    }
}

impl IntoAttributeValue for SeparatorOrientation {
    type Output = String;

    fn into_attribute_value(self) -> Self::Output {
        self.to_string()
    }
}

impl From<String> for SeparatorOrientation {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "vertical" => SeparatorOrientation::Vertical,
            _ => SeparatorOrientation::Horizontal,
        }
    }
}

impl From<&str> for SeparatorOrientation {
    fn from(s: &str) -> Self {
        s.to_string().into()
    }
}

#[component]
pub fn Separator(
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] orientation: SeparatorOrientation,
    #[prop(into, optional)] decorative: MaybeProp<bool>,
) -> impl IntoView {
    view! {
        <div
            role=if decorative.get().unwrap_or(true) { "none" } else { "separator" }
            attr:aria-orientation=orientation
            class=(["shrink-0", "bg-border"], true)
            class=("h-[1px] w-full", matches!(orientation, SeparatorOrientation::Horizontal))
            class=("h-full w-[1px]", matches!(orientation, SeparatorOrientation::Horizontal))
        />
    }
}