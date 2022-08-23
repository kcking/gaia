use syntect::{
    highlighting::{
        Color, HighlightIterator, HighlightState, Highlighter, ScopeSelector, ScopeSelectors,
        ThemeSet,
    },
    parsing::{ParseState, Scope, ScopeStack, SyntaxSet},
    util::LinesWithEndings,
};
use web_sys::{console, Element};
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
    //  IDEA: try NodeRef and call Prism.highlightElement on it
    //      but how do we store a ref??
    let code_ref = use_state_eq(|| NodeRef::default());
    let mut code_tag = c.children.iter().next().unwrap().clone();
    match &mut code_tag {
        VNode::VTag(t) => t.node_ref = (*code_ref).clone(),
        _ => {}
    };

    use_effect_with_deps(
        move |_| {
            console::log_1(&"highlighting...".to_string().into());
            let element = code_ref.cast::<Element>().unwrap();
            prism::highlightElement(element.clone());
            move || {
                element
                    .closest(".codecontainer")
                    .ok()
                    .flatten()
                    .map(|e| e.remove());
            }
        },
        c.children.clone(),
    );

    // try_highlight_code(&c.children).unwrap_or_default()
    html! {
        <div class="codecontainer">
            <pre class="overflow-auto m-4 p-6 bg-gray-300/5 rounded">
                {code_tag}
            </pre>
        </div>
    }
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

    let theme = THEME_SET.themes.values().next()?;
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

mod prism {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        pub type Language;
        #[wasm_bindgen(js_namespace = Prism)]
        pub static languages: Language;

        #[wasm_bindgen(method, structural, indexing_getter)]
        pub fn get(this: &Language, prop: String) -> Language;

        #[wasm_bindgen(js_namespace = Prism)]
        pub fn highlight(code: String, lang: Language) -> String;

        #[wasm_bindgen(js_namespace = Prism)]
        pub fn highlightElement(element: web_sys::Element);
    }
}
