use syntect::parsing::SyntaxSet;
use yew::{
    prelude::*,
    virtual_dom::{VNode, VTag},
};

#[function_component]
pub fn HighlightCode(c: &super::ChildProps) -> Html {
    try_highlight_code(&c.children).unwrap_or_default()
}

fn try_highlight_code(c: &Children) -> Option<Html> {
    let mut iter = c.iter();
    let first = iter.next()?;
    if iter.next().is_some() {
        return None;
    }

    let (code_children, code_attrs) = match &first {
        VNode::VTag(tag) if tag.tag().to_lowercase() == "code" => (tag.children(), &tag.attributes),
        _ => return None,
    };
    if code_children.len() != 1 {
        return None;
    }
    let code_child = code_children.get(0)?;
    let code = match code_child {
        VNode::VText(txt) => &txt.text,
        _ => return None,
    };
    let code_lang = code_attrs
        .iter()
        .find(|(k, _)| k == &"class")
        .map(|(_, v)| {
            v.split_whitespace()
                .find(|class| class.starts_with("language-"))
                .map(|class| class.split_at("language-".len()).1)
        })
        .flatten();
    let ss = SyntaxSet::load_defaults_newlines();
    let syntax = match code_lang {
        Some(lang) => ss.find_syntax_by_name(lang).unwrap_or_else(|| {
            ss.find_syntax_by_extension(lang)
                .unwrap_or_else(|| ss.find_syntax_plain_text())
        }),
        None => todo!(),
    };

    // syntect::parsing::
    // let gen = syntect::html::ClassedHTMLGenerator::new_with_class_style(
    //     syntax,
    //     &ss,
    //     syntect::html::ClassStyle::Spaced,
    // );

    Some(html! {
        {code}
    })
}
