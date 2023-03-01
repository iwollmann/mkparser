use web_sys::{HtmlInputElement, InputEvent};
use yew::prelude::*;

#[hook]
pub fn use_input(value: String) -> (String, Callback<InputEvent>) {
    let state = use_state(|| value);
    let onchange = {
        let state = state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            state.set(input.value());
        })
    };
    (state.to_string(), onchange)
}
