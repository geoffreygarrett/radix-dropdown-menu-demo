use leptos::prelude::*;

pub struct UseControllableStateParams<T: 'static + Send + Sync> {
    pub prop: MaybeProp<T>,
    pub default_prop: MaybeProp<T>,
    pub on_change: Option<Callback<T>>,
}

pub struct UseControllableOptionalStateParams<T: 'static + Send + Sync> {
    pub prop: MaybeProp<Option<T>>,
    pub default_prop: MaybeProp<Option<T>>,
    pub on_change: Option<Callback<Option<T>>>,
}

pub fn use_controllable_optional_state<T: Clone + PartialEq + Send + Sync>(
    UseControllableOptionalStateParams {
        prop,
        default_prop,
        on_change,
    }: UseControllableOptionalStateParams<T>,
) -> (Signal<Option<T>>, Callback<Option<T>>) {
    let (uncontrolled_prop, set_uncontrolled_prop) =
        use_uncontrolled_optional_state(UseUncontrollableOptionalStateParams {
            default_prop,
            on_change,
        });
    let prop = Signal::derive(move || prop.get().flatten());
    let is_controlled = Signal::derive(move || prop.get().is_some());
    let value = Signal::derive(move || match is_controlled.get() {
        true => prop.get(),
        false => uncontrolled_prop.get(),
    });

    let set_value = Callback::new(move |next_value: Option<T>| {
        if is_controlled.get() {
            if next_value != prop.get() {
                if let Some(on_change) = on_change.clone() {
                    on_change.run(next_value);
                }
            }
        } else {
            set_uncontrolled_prop.set(next_value);
        }
    });

    (value, set_value)
}

pub fn use_controllable_state<T: Clone + PartialEq + Send + Sync + Default>(
    UseControllableStateParams {
        prop,
        default_prop,
        on_change,
    }: UseControllableStateParams<T>,
) -> (Signal<T>, Callback<T>) {
    let (uncontrolled_prop, set_uncontrolled_prop) =
        use_uncontrolled_state(UseUncontrollableStateParams {
            default_prop,
            on_change,
        });

    let prop = Signal::derive(move || prop.get());
    let is_controlled = Signal::derive(move || prop.get().is_some());

    let value = Signal::derive(move || {
        if is_controlled.get() {
            prop.get().unwrap()
        } else {
            uncontrolled_prop
                .get()
                .unwrap_or_else(|| default_prop.get().unwrap_or_default().clone())
        }
    });

    let set_value = Callback::new(move |next_value: T| {
        if is_controlled.get() {
            if Some(next_value.clone()) != prop.get() {
                if let Some(on_change) = on_change.clone() {
                    on_change.run(next_value);
                }
            }
        } else {
            set_uncontrolled_prop.set(Some(next_value));
        }
    });

    (value, set_value)
}

pub struct UseUncontrollableStateParams<T: 'static + Send + Sync> {
    pub default_prop: MaybeProp<T>,
    pub on_change: Option<Callback<T>>,
}

pub struct UseUncontrollableOptionalStateParams<T: 'static + Send + Sync> {
    pub default_prop: MaybeProp<Option<T>>,
    pub on_change: Option<Callback<Option<T>>>,
}

fn use_uncontrolled_optional_state<T: Clone + PartialEq + Send + Sync>(
    UseUncontrollableOptionalStateParams {
        default_prop,
        on_change,
    }: UseUncontrollableOptionalStateParams<T>,
) -> (ReadSignal<Option<T>>, WriteSignal<Option<T>>) {
    let uncontrolled_state = signal::<Option<T>>(default_prop.get().unwrap_or_default());
    let (value, _) = uncontrolled_state;
    let prev_value = RwSignal::new(value.get_untracked());

    Effect::new(move |_| {
        let value = value.get();
        if prev_value.get() != value {
            if let Some(on_change) = on_change.clone() {
                on_change.run(value.clone());
                prev_value.set(value);
            }
        }
    });

    uncontrolled_state
}

fn use_uncontrolled_state<T: Clone + PartialEq + Send + Sync + Default>(
    UseUncontrollableStateParams {
        default_prop,
        on_change,
    }: UseUncontrollableStateParams<T>,
) -> (ReadSignal<Option<T>>, WriteSignal<Option<T>>) {
    let uncontrolled_state = signal::<Option<T>>(Some(default_prop.get().unwrap_or_default()));
    let (value, set_value) = uncontrolled_state;
    let prev_value = RwSignal::new(value.get_untracked());

    Effect::new(move |_| {
        let current_value = value.get();
        if prev_value.get() != current_value {
            if let Some(on_change) = &on_change {
                if let Some(val) = current_value.clone() {
                    on_change.run(val);
                }
                prev_value.set(current_value.clone());
            }
        }
    });

    (value, set_value)
}
