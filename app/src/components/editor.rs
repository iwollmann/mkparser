use crate::components::copy_button::CopyButton;
use crate::components::safe_html::SafeHtml;
use crate::utils;
use markdown;
use web_sys::InputEvent;
use yew::Callback;
use yew::{classes, function_component, html, Properties};

#[derive(Properties, Debug, PartialEq)]
pub struct EditorProps {
    pub data: String,
    pub onchange: Callback<InputEvent>,
    pub in_preview: bool,
}

fn render_editor(props: &EditorProps) -> yew::Html {
    if !&props.in_preview {
        html! {
            <textarea oninput={props.onchange.clone()} class={classes!("whitespace-pre-wrap", "rounded", "break-all", "overflow-y-auto", "h-full", "w-full", "p-1")} value={props.data.clone()}>
            </textarea>
        }
    } else {
        let emojified_data = utils::emojify(props.data.as_str());
        let safe_html = markdown::to_html(emojified_data.as_str());
        html! {
            <div class={classes!("flex-1", "bg-[url('dots.svg')]","overflow-y-auto", "h-full", "w-full", "p-1")}>
                <SafeHtml html={safe_html} />
            </div>
        }
    }
}

#[function_component(Editor)]
pub fn editor(props: &EditorProps) -> yew::Html {
    html! {
        <div class={classes!("relative", "flex", "flex-1", "bg-stone-50", "h-96")}>
            <CopyButton data={props.data.clone()} />
            <div class={classes!("flex", "bg-neutral-50", "flex-1","m-4", "shadow-md")}>
            { render_editor(props) }
            </div>
        </div>
    }
}
