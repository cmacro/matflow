use leptos::prelude::*;
use crate::app::Page;

#[component]
pub fn NavItem(
    label: String,
    icon_svg: &'static str,
    #[prop(into)] active: Signal<bool>,        // ← 修改这里
    on_click: impl Fn(Page) + 'static,
    page: Page,
) -> impl IntoView {
    view! {
        <div 
            class=move || format!(
                "flex items-center gap-3 px-3 py-2 rounded-lg transition-colors duration-200 cursor-pointer {}",
                if active.get() { 
                    "bg-slate-800 text-white shadow-sm" 
                } else { 
                    "text-slate-400 hover:bg-slate-900 hover:text-slate-200" 
                }
            )
            on:click=move |_| on_click(page.clone())
        >
            <div class="flex items-center justify-center w-5 h-5">
                <div inner_html=icon_svg></div>
            </div>
            <span class="text-sm font-medium">{label}</span>
        </div>
    }
}
