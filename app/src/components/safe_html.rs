use yew::{function_component, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub html: String,
}

#[function_component(SafeHtml)]
pub fn safe_html(props: &Props) -> Html {
    let div = gloo_utils::document().create_element("article").unwrap();
    div.set_class_name("prose prose-slate");
    div.set_inner_html(&props.html.clone());

    Html::VRef(div.into())
}
