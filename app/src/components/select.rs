use serde::{Deserialize, Serialize};
use web_sys::Event;
use yew::html;
use yew::Callback;
use yew::{classes, function_component, Properties};

#[derive(Properties, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ListItem {
    pub id: String,
    pub caption: String,
}

#[derive(Properties, Debug, PartialEq, Clone)]
pub struct SelectProps {
    pub options: Vec<ListItem>,
    pub label_text: String,
    pub id: String,
    pub on_change: Callback<Event>,
}

#[function_component(Select)]
pub fn select(props: &SelectProps) -> yew::Html {
    let _options = props.options.clone().into_iter().map(|op| {
        html! {
            <option value={ op.id }> { op.caption } </option>
        }
    });

    html! {
        <div class={classes!("flex", "flex-col", "mt-3", "w-1/4")}>
            <label for={props.id.clone()}>{ props.label_text.clone()}</label>
            <select class={classes!("border", "rounded", "p1")} id={props.id.clone()} onchange={props.on_change.clone()}>
                <option value="" selected={true}>{"--Please choose an option--"}</option>
                { for _options }
            </select>
        </div>
    }
}
