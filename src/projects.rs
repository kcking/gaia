use std::sync::Arc;

use stylist::{css, style};
use yew::{prelude::*, virtual_dom::VNode};

#[derive(Properties, PartialEq)]
struct ProjectProps {
    description: Html,
    name: String,
    image: String,
}

#[function_component]
fn Project(props: &ProjectProps) -> Html {
    let style = style!(
        r#"
        background-size: 80% 80%;
    "#
    )
    .unwrap();
    html! {
    <div
      className="group p-2 border-0 border-zinc-700 border-solid w-64 h-64 flex flex-col justify-between select-none "
      tabIndex={0}
      key={props.name.as_str()}
    >
      <div
        className={
          "w-60 h-60 absolute group-hover:invisible group-focus:invisible"
        }
        style={
            style.get_style_str().to_owned()
        //   backgroundImage: `url('${props.image}')`,
        //   backgroundSize: "80% 80%",
        //   backgroundPosition: "50% 20%",
        //   backgroundRepeat: "no-repeat",
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
