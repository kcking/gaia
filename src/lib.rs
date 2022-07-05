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

fn switch(route: Route) -> Html {
    html! {
        <>
            <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
            <Link<Route> to={Route::Blog}>{"Blog"}</Link<Route>>
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
                }
            }
        </>
    }
}
