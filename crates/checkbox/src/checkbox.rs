use leptos::{prelude::*, context::Provider, ev, html};
use leptos_node_ref::prelude::*;
use radix_leptos_compose_refs::use_composed_refs;
use radix_leptos_presence::Presence;
use radix_leptos_primitive::{compose_callbacks, Primitive};
use radix_leptos_use_controllable_state::{use_controllable_state, UseControllableStateParams};
use radix_leptos_use_previous::use_previous;
use radix_leptos_use_size::use_size;

/* -------------------------------------------------------------------------------------------------
 * CheckedState enum and trait implementations
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub enum CheckedState {
    #[default]
    False,
    True,
    Indeterminate,
}

impl CheckedState {
    pub fn toggled(&self) -> Self {
        match self {
            CheckedState::False => CheckedState::True,
            CheckedState::True => CheckedState::False,
            CheckedState::Indeterminate => CheckedState::True,
        }
    }

    pub fn is_indeterminate(&self) -> bool {
        is_indeterminate(*self)
    }

    pub fn get_state(&self) -> &'static str {
        get_state(*self)
    }
}

impl From<bool> for CheckedState {
    fn from(value: bool) -> Self {
        if value {
            CheckedState::True
        } else {
            CheckedState::False
        }
    }
}

// Our trait for converting to CheckedState
pub trait ToChecked {
    fn to_checked(self) -> MaybeProp<CheckedState>;
}

// Implement for MaybeProp<bool>
impl ToChecked for MaybeProp<bool> {
    fn to_checked(self) -> MaybeProp<CheckedState> {
        Signal::derive(move || self.get().map(|v| v.into())).into()
    }
}

// Implement for Signal<bool>
impl ToChecked for Signal<bool> {
    fn to_checked(self) -> MaybeProp<CheckedState> {
        Signal::derive(move || Some(self.get().into())).into()
    }
}

// Implement for ReadSignal<bool>
impl ToChecked for ReadSignal<bool> {
    fn to_checked(self) -> MaybeProp<CheckedState> {
        Signal::derive(move || Some(self.get().into())).into()
    }
}
impl From<CheckedState> for Option<bool> {
    fn from(state: CheckedState) -> Self {
        match state {
            CheckedState::False => Some(false),
            CheckedState::True => Some(true),
            CheckedState::Indeterminate => None,
        }
    }
}

impl From<CheckedState> for bool {
    fn from(state: CheckedState) -> Self {
        match state {
            CheckedState::False => false,
            CheckedState::True => true,
            CheckedState::Indeterminate => false,
        }
    }
}

impl IntoAttributeValue for CheckedState {
    type Output = Option<bool>;

    fn into_attribute_value(self) -> Self::Output {
        self.into()
    }
}

/* -------------------------------------------------------------------------------------------------
 * Checkbox
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Debug)]
struct CheckboxContextValue {
    state: Signal<CheckedState>,
    disabled: Signal<bool>,
}

#[component]
pub fn Checkbox<C: IntoView + 'static>(
    /// HTML `name` attribute (if any).
    #[prop(into, optional)] name: MaybeProp<String>,

    /// Controlled checked state.
    #[prop(into, optional)] checked: MaybeProp<CheckedState>,

    /// Uncontrolled default checked state.
    #[prop(into, optional)] default_checked: MaybeProp<CheckedState>,

    /// Callback when the checked state changes.
    #[prop(into, optional)] on_checked_change: Option<Callback<CheckedState>>,

    #[prop(into, optional)] required: MaybeProp<bool>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] value: MaybeProp<String>,

    /// Custom keydown handler, composed with internal logic.
    #[prop(into, optional)] on_keydown: Option<Callback<ev::KeyboardEvent>>,

    /// Custom click handler, composed with internal logic.
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,

    /// Render only children (no extra wrapper) if `true`.
    #[prop(into, optional)] as_child: MaybeProp<bool>,

    /// NodeRef for the underlying button element.
    #[prop(optional)] node_ref: AnyNodeRef,

    /// Typed children for content inside the button.
    children: TypedChildrenFn<C>,
) -> impl IntoView
{
    let name = Signal::derive(move || name.get());
    let required = Signal::derive(move || required.get().unwrap_or(false));
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let value = Signal::derive(move || value.get().unwrap_or_else(|| "on".to_string()));

    // Button reference for capturing size, form detection, etc.
    let button_ref = NodeRef::<html::Button>::new();
    let composed_refs = use_composed_refs((node_ref, button_ref));

    // Detect whether this button is part of a form
    let is_form_control = Signal::derive(move || {
        button_ref
            .get()
            .and_then(|button| button.closest("form").ok())
            .flatten()
            .is_some()
    });

    // Manage controlled/uncontrolled checked state
    let (checked_signal, set_checked) = use_controllable_state(UseControllableStateParams {
        prop: checked,
        on_change: on_checked_change.map(|cb| {
            Callback::new(move |val: CheckedState| {
                cb.run(val);
            })
        }),
        default_prop: default_checked,
    });

    let initial_checked_state = checked_signal.get_untracked();
    let handle_reset = move |_| set_checked.run(initial_checked_state);

    // If part of a form, reset the checkbox on "reset" event
    let _ = leptos_use::use_event_listener(button_ref.clone(), ev::reset, handle_reset);

    // Provide context for our Indicator
    let context_value = CheckboxContextValue {
        state: checked_signal,
        disabled,
    };

    // Handle the composed keydown event
    let on_keydown_internal = Some(Callback::new(move |event: ev::KeyboardEvent| {
        // By WAI-ARIA, pressing Enter shouldn't toggle the checkbox
        if event.key() == "Enter" {
            event.prevent_default();
        }
    }));

    // Handle the composed click event
    let on_click_internal = Some(Callback::new(move |event: ev::MouseEvent| {
        set_checked.run(checked_signal.get().toggled());
        // If part of a form, stop the event so that only the hidden input can bubble.
        if is_form_control.get() {
            event.stop_propagation();
        }
    }));

    view! {
        <Provider value=context_value>
            <Primitive
                element=html::button
                children=children
                as_child=as_child
                node_ref=composed_refs
                // ARIA roles and states
                attr:role="checkbox"
                attr:r#type="button"
                attr:aria-checked=move || {
                    let st = checked_signal.get();
                    if st == CheckedState::Indeterminate {
                        "mixed"
                    } else if st == CheckedState::True {
                        "true"
                    } else {
                        "false"
                    }
                }
                attr:aria-required=move || if required.get() { "true" } else { "false" }
                // Data attributes
                attr:data-state=move || get_state(checked_signal.get())
                attr:data-disabled=move || disabled.get().then_some("")
                // Standard button attributes
                attr:disabled=move || disabled.get()
                attr:value=value
                // Compose user-provided handlers with our internal logic
                on:keydown=compose_callbacks(on_keydown, on_keydown_internal, None)
                on:click=compose_callbacks(on_click, on_click_internal, None)
            />

            // Bubble input for form usage
            <Show when=move || is_form_control.get()>
                <BubbleInput
                    control_ref=button_ref
                    checked=checked_signal
                    required=required
                    disabled=disabled
                    value=value
                />
            </Show>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * CheckboxIndicator
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn CheckboxIndicator<C: IntoView + 'static>(
    /// Force mounting (useful for manual control of animations).
    #[prop(into, optional)]
    force_mount: MaybeProp<bool>,

    /// Render only children if `true`.
    #[prop(into, optional)]
    as_child: MaybeProp<bool>,

    /// Indicator children (e.g., icons).
    children: TypedChildrenFn<C>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<CheckboxContextValue>();
    let force_mount = Signal::derive(move || force_mount.get().unwrap_or(false));

    // Determine if the Indicator should be present
    let present = Signal::derive(move || {
        force_mount.get()
            || context.state.get() == CheckedState::True
            || context.state.get() == CheckedState::Indeterminate
    });

    view! {
        <Presence present=present>
            <Primitive
                element=html::span
                as_child=as_child {}
                data-state=move || get_state(context.state.get())
                data-disabled=move || context.disabled.get().then_some("")
                style:pointer-events="none"
            >
                {children.with_value(|children| children())}
            </Primitive>
        </Presence>
    }
}

/* -------------------------------------------------------------------------------------------------
 * BubbleInput (for form integration)
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn BubbleInput(
    #[prop(into)] control_ref: AnyNodeRef,
    #[prop(into)] checked: Signal<CheckedState>,
    #[prop(into, optional)] required: MaybeProp<bool>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] value: MaybeProp<String>,
) -> impl IntoView {
    let required = Signal::derive(move || required.get().unwrap_or(false));
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let value = Signal::derive(move || value.get().unwrap_or_else(|| "on".to_string()));

    let node_ref: NodeRef<html::Input> = NodeRef::new();
    let prev_checked = use_previous(checked);
    let control_size = use_size(control_ref);

    // Whenever `checked` changes, bubble a click event to the parent form
    Effect::new(move |_| {
        if let Some(input) = node_ref.get() {
            let old = prev_checked.get();
            let new = checked.get();
            if old != new {
                // Mark the native input
                input.set_indeterminate(new == CheckedState::Indeterminate);
                input.set_checked(matches!(new, CheckedState::True));

                // Bubble a "click" event
                let event = web_sys::Event::new("click").unwrap();
                let _ = input.dispatch_event(&event);
            }
        }
    });

    view! {
        <input
            node_ref=node_ref
            r#type="checkbox"
            aria-hidden="true"
            checked=move || matches!(checked.get(), CheckedState::True)
            required=move || required.get().then_some("")
            disabled=move || disabled.get().then_some("")
            value=value
            tab-index="-1"
            style:transform="translateX(-100%)"
            style:width=move || {
                control_size.get().map(|s| format!("{}px", s.width)).unwrap_or_default()
            }
            style:height=move || {
                control_size.get().map(|s| format!("{}px", s.height)).unwrap_or_default()
            }
            style:position="absolute"
            style:pointer-events="none"
            style:opacity="0"
            style:margin="0"
        />
    }
}

/* -------------------------------------------------------------------------------------------------
 * Helpers
 * -----------------------------------------------------------------------------------------------*/

pub fn get_state<T: Into<CheckedState>>(checked: T) -> &'static str {
    match checked.into() {
        CheckedState::True => "checked",
        CheckedState::False => "unchecked",
        CheckedState::Indeterminate => "indeterminate",
    }
}

pub fn is_indeterminate<T: Into<CheckedState>>(checked: T) -> bool {
    matches!(checked.into(), CheckedState::Indeterminate)
}

/* -------------------------------------------------------------------------------------------------
 * Primitive re-exports
 * -----------------------------------------------------------------------------------------------*/

pub mod primitive {
    // Re-export core items so consumers can `use checkbox::primitive::* as CheckboxPrimitive`
    pub use super::*;
    pub use Checkbox as Root;
    pub use CheckboxIndicator as Indicator;
}
