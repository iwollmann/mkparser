use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;

#[hook]
pub fn use_select(value: String) -> (String, Callback<Event>) {
    let state = use_state(|| value);
    let onchange = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            state.set(input.value());
        })
    };
    (state.to_string(), onchange)
}
