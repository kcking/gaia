use std::collections::HashMap;

use yew::prelude::*;
use yew_router::{
    history::{AnyHistory, History, MemoryHistory},
    prelude::*,
};

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/blog")]
    Blog,
    #[at("/Projects")]
    Projects,
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[derive(Properties, PartialEq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
    pub queries: HashMap<String, String>,
}

#[function_component]
pub fn ServerApp(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history
        .push_with_query(&*props.url, &props.queries)
        .unwrap();

    html! {
        <Router history={history}>
            <Switch<Route> render={switch} />
        </Router>
    }
}

#[function_component]
fn Navbar() -> Html {
    html! {
        <div class="flex justify-evenly flex-wrap w-full">
            <h1 class="font-display text-6xl p-10">
                <Link<Route> to={Route::Home} classes="p-4" >{"impl Future {}"}</Link<Route>>
            </h1>
            <div class="flex">
                <Link<Route> classes="p-4 text-3xl" to={Route::Blog}>{"Blog"}</Link<Route>>
                <Link<Route> classes="p-4 text-3xl" to={Route::Projects}>{"Projects"}</Link<Route>>
                // <Link<Route> classes="p-4 text-3xl" to="https://twitter.com/4kevinking">{"Contact"}</Link<Route>>
            </div>
        </div>
    }
}

#[function_component]
fn Home() -> Html {
    html!{

    }
}

fn switch(route: Route) -> Html {
    html! {
        <>
            <Navbar />
            {
                match route {
                    Route::Home => html! {
                        <>
                        {"Home2 3"}
                        </>
                    },
                    Route::Blog => html! {
                        "Blog"
                    },
                    Route::Projects => html! {
                        "Projects"
                    },
                }
            }
        </>
    }
}
