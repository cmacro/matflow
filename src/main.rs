use leptos::prelude::*;

mod app;
mod components;
mod pages;
mod utils;

fn main() {
    mount_to_body(|| view! { <app::App/> });
}
