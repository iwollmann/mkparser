mod components;
mod hooks;
mod utils;

use components::editor::Editor;
use wasm_logger;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let preview = use_state(|| "".to_string());
    let in_preview = use_state(|| false);

    let on_preview_change = {
        let preview = preview.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            preview.set(input.value());
        })
    };

    html! {
        <>
            <div class={classes!("flex", "flex-column", "items-center", "justify-center", "bg-sky-500", "h-12")}>
                <span class={classes!("ml-4", "text-2xl", "text-slate-700")}>{ "Markdown Viewer" }</span>
            </div>
            <div>
                <div class={classes!("container", "mx-auto", "flex", "flex-col")}>
                    <div class={classes!("flex", "flex-row")}>
                        <Editor onchange={on_preview_change.clone()} data={preview.to_string()} in_preview={*in_preview} />
                        <Editor onchange={on_preview_change} data={preview.to_string()} in_preview={true} />
                    </div>
                </div>
            </div>
            <footer class={classes!("fixed", "bottom-0", "-z-1","text-center", "lg:text-left", "bottom-0", "w-full")} style="z-index:-1;">
                <div class={classes!("text-slate-300", "text-center", "p-4")}>
                    {"Powered by IMW"}
                </div>
            </footer>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
