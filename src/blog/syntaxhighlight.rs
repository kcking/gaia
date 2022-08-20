use syntect::{
    highlighting::{
        Color, HighlightIterator, HighlightState, Highlighter, ScopeSelector, ScopeSelectors,
        ThemeSet,
    },
    parsing::{ParseState, Scope, ScopeStack, SyntaxSet},
    util::LinesWithEndings,
};
use yew::{
    prelude::*,
    virtual_dom::{VNode, VTag},
};

lazy_static::lazy_static! {
    static ref SYNTAX_SET : SyntaxSet = {
        SyntaxSet::load_defaults_newlines()
    };
    static ref THEME_SET  : ThemeSet = {
        ThemeSet::load_defaults()
    };
}

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

    let syntax = match code_lang {
        Some(lang) => SYNTAX_SET.find_syntax_by_name(lang).unwrap_or_else(|| {
            SYNTAX_SET
                .find_syntax_by_extension(lang)
                .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text())
        }),
        None => todo!(),
    };
    let mut p_state = ParseState::new(syntax);

    let theme = THEME_SET.themes.values().next()?.clone();
    let highlighter = Highlighter::new(&theme);
    let mut h_state = HighlightState::new(&highlighter, ScopeStack::new());

    let mut spans = vec![];
    for line in LinesWithEndings::from(code) {
        let ops = p_state.parse_line(&line, &SYNTAX_SET).ok()?;
        let h_iter = HighlightIterator::new(&mut h_state, &ops, line, &highlighter);
        for (style, text) in h_iter {
            let Color { r, g, b, a } = style.foreground;
            let style_str = format!("color:rgba({r}, {g}, {b}, {a});");
            spans.push(html! {<span style={style_str}>{text}</span>});
        }
    }

    // syntect::parsing::
    // let gen = syntect::html::ClassedHTMLGenerator::new_with_class_style(
    //     syntax,
    //     &ss,
    //     syntect::html::ClassStyle::Spaced,
    // );

    Some(html! {
        <>
        {
            spans.into_iter().collect::<Html>()
        }
        </>
    })
}
