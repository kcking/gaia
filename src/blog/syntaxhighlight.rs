use yew::prelude::*;

#[function_component]
pub fn HighlightCode(c: &super::ChildProps) -> Html {
    try_highlight_code().unwrap_or_default()
}

fn try_highlight_code() -> Option<Html> {
    Some(html! {})
}
