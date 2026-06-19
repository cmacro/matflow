use leptos::prelude::*;

mod app;
mod components;
mod data;
mod pages;
mod models;
mod utils;

fn main() {
    mount_to_body(|| view! { <app::App/> });
}
