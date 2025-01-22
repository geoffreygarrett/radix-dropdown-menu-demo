use std::fmt::Debug;
use std::{collections::HashMap, hash::Hash};
use leptos::prelude::*;

type Machine<S, E> = HashMap<S, HashMap<E, S>>;

pub fn use_state_machine<S, E>(
    initial_state: S,
    machine: Machine<S, E>,
) -> (ReadSignal<S>, Callback<E>)
where
    S: Clone + Debug + Eq + Hash + Send + Sync + 'static,
    E: Clone + Debug + Eq + Hash + Send + Sync + 'static,
{
    let (state, set_state) = signal(initial_state);

    (
        state,
        Callback::new(move |event| {
            let current_state = state.get();
            let next_state = machine
                .get(&current_state)
                .and_then(|events| events.get(&event));

            if let Some(next_state) = next_state {
                set_state.set(next_state.clone());
            }
        }),
    )
}