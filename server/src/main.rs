use std::collections::HashMap;
use std::convert::Infallible;
use std::marker::PhantomData;

use anyhow::Result;
use axum::body::{Body, BoxBody};
use axum::extract::Query;
use axum::http::{Request, Response, StatusCode};
use axum::response::{Html, IntoResponse};
use axum::routing::get_service;
use axum::Extension;
use axum::{routing::get, Router};
use futures::future::BoxFuture;
use futures::ready;
use gaia::ServerAppProps;
use once_cell::sync::Lazy;
use tokio_util::task::LocalPoolHandle;
use tower::Service;
use tower_http::services::ServeDir;
use yew_router::Routable;

lazy_static::lazy_static!(
    static ref INDEX_HTML: String = {
        String::from_utf8( std::fs::read("static/index.bzl.html").unwrap().try_into().unwrap()).unwrap()
    };
);
static LOCAL_POOL: Lazy<LocalPoolHandle> = Lazy::new(|| LocalPoolHandle::new(num_cpus::get()));

async fn index(
    Extension(index_html_s): Extension<String>,
    url: Request<Body>,
    Query(queries): Query<HashMap<String, String>>,
) -> Html<String> {
    let out = LOCAL_POOL
        .spawn_pinned(|| async move {
            let props = ServerAppProps {
                url: url.uri().to_string().into(),
                queries,
            };
            let mut out = String::new();
            yew::ServerRenderer::<gaia::ServerApp>::with_props(props)
                .render_to_string(&mut out)
                .await;
            out
        })
        .await
        .unwrap();
    Html(index_html_s.replace("<body>", &format!("<body>{}", out)))
}

fn debug_dirs() -> Result<()> {
    for d in std::fs::read_dir(".")? {
        eprintln!("{:?}", d?.file_name());
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    debug_dirs()?;
    let route_service = RoutableService::<gaia::Route, _, _>::new(
        get(index),
        get_service(ServeDir::new("dist")).handle_error(|e| async move {
            dbg!(e);
            StatusCode::BAD_REQUEST
        }),
    );
    let route_service = get_service(route_service).layer(Extension(INDEX_HTML.to_string()));

    let addr = "127.0.0.1:8080";
    eprintln!("starting server on {}", addr);
    axum::Server::bind(&addr.parse()?)
        .serve(get_service(route_service).into_make_service())
        .await?;

    Ok(())
}

#[derive(Clone)]
struct RoutableService<R, S: Clone, F: Clone> {
    r: PhantomData<R>,
    s_ready: bool,
    s: S,
    f_ready: bool,
    f: F,
}

impl<R, S: Clone, F: Clone> RoutableService<R, S, F> {
    pub fn new(s: S, f: F) -> Self {
        Self {
            s,
            f,
            s_ready: false,
            f_ready: false,
            r: PhantomData,
        }
    }
}

impl<R, S, F> Service<Request<Body>> for RoutableService<R, S, F>
where
    R: Routable,
    S: Service<Request<Body>, Error = Infallible> + Clone,
    S::Response: IntoResponse,
    S::Future: Send + 'static,
    F: Service<Request<Body>, Error = Infallible> + Clone,
    F::Response: IntoResponse,
    F::Future: Send + 'static,
{
    type Response = Response<BoxBody>;
    type Error = Infallible;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        loop {
            match (self.s_ready, self.f_ready) {
                (true, true) => {
                    return Ok(()).into();
                }
                (false, _) => {
                    ready!(self.s.poll_ready(cx))?;
                    self.s_ready = true;
                }
                (_, false) => {
                    ready!(self.f.poll_ready(cx))?;
                    self.f_ready = true;
                }
            }
        }
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        //  TODO: think about how this treats not_found_path
        match <R as Routable>::recognize(req.uri().path()).is_some() {
            true => {
                self.s_ready = false;
                let fut = self.s.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res.into_response())
                })
            }
            false => {
                self.f_ready = false;
                let fut = self.f.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res.into_response())
                })
            }
        }
    }
}
