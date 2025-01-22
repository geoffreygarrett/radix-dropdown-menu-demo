use leptos::prelude::*;

pub fn use_previous<T: Clone + PartialEq + Sync + Send + 'static>(value: Signal<T>) -> Memo<T> {
    let current = RwSignal::new(value.get_untracked());
    let previous = RwSignal::new(value.get_untracked());

    Memo::new(move |_| {
        let value = value.get();
        let current_value = current.get();
        if current_value != value {
            previous.set(current_value);
            current.set(value.clone());
        }
        previous.get()
    })
}
