use std::sync::Arc;

use stylist::{css, style, YieldStyle};
use yew::{prelude::*, virtual_dom::VNode};

#[derive(Properties, PartialEq)]
struct ProjectProps {
    description: Html,
    name: String,
    image: String,
}

#[function_component]
fn Project(props: &ProjectProps) -> Html {
    let img_style = css!(
        r#"
    background-image: ${image};
    background-size: 80% 80%;
    background-position: 50% 20%;
    background-repeat: no-repeat;
    "#,
        image = &props.image
    );
    html! {
    <div
      className="group p-2 border-0 border-zinc-700 border-solid w-64 h-64 flex flex-col justify-between select-none "
      tabIndex={0}
      key={props.name.as_str()}
    >
      <div
        class={
          classes!("w-60 h-60 absolute group-hover:invisible
          group-focus:invisible".split_ascii_whitespace().collect::<Vec<_>>(),
          img_style)
        }
      ></div>
      <div className="invisible group-hover:visible group-focus:visible">
      {
        props.description.clone()
      }
      </div>
      <div className="text-2xl font-display place-self-center">
        {&props.name}
      </div>
    </div>
    }
}

#[function_component]
pub fn Projects() -> Html {
    html! {
        <Project name="test project" image="hi.jpg" description={html!{
            <p>{"Description"}</p>
        }}>
        </Project>
    }
}
