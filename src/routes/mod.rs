use axum::{routing::get, Router};
use axum_embed::ServeEmbed;
use maud::Markup;
use rust_embed::RustEmbed;
use tap::prelude::*;

use crate::state::AppState;
use crate::utils::layout::Page;
use crate::utils::Result;

pub fn create_routes() -> axum::Router<AppState> {
    Router::new()
        .route("/", get(root_handler))
        .route("/add_element", get(add_element))
        .nest_service("/static", ServeEmbed::<Assets>::new())
}

async fn root_handler() -> Result<Markup> {
    maud::html! {
        div class="container mx-auto" {
            h1 class="text-4xl font-bold text-center text-blue-600" {
                "Hello World with Tailwind!"
            }
            p class="mt-4 text-lg" {
                "This is a page styled with Tailwind CSS."
            }
            button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" hx-get="/add_element" hx-trigger="click" hx-swap="afterend" { "Add Element" }
        }
    }
    .page("Axum with Tailwind and HTMX")
    .pipe(Ok)
}

async fn add_element() -> Result<Markup> {
    maud::html! {
        div class="bg-cyan-700 h-5 w-5 m-5" {}
    }
    .pipe(Ok)
}

#[derive(RustEmbed, Clone)]
#[folder = "static/"]
struct Assets;
