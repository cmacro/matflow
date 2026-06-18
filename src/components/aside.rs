use leptos::prelude::*;
use crate::components::NavItem;
use crate::app::Page;

#[derive(Clone)]
pub struct NavConfig {
    pub page: Page,
    pub label: &'static str,
    pub icon_svg: &'static str,
}

#[component]
pub fn Aside(
    current_page: ReadSignal<Page>,
    set_current_page: WriteSignal<Page>,
    #[prop(into)] nav_configs: Vec<NavConfig>,
) -> impl IntoView {
    let (collapsed, set_collapsed) = signal(false);

    view! {
        <aside class=move || format!("h-full border-r border-slate-800 bg-slate-900 text-slate-300 transition-all duration-300 flex flex-col {}", if collapsed.get() { "w-16" } else { "w-64" })>
            <div class="p-6 text-xl font-bold tracking-tight text-green-500 overflow-hidden whitespace-nowrap">
                {move || if collapsed.get() { "M".to_string() } else { "MATFLOW".to_string() }}
            </div>
            
            <nav class="flex-1 px-3 space-y-1">
                {nav_configs.into_iter().map(|config| {
                    view! {
                        <NavItem
                            label=config.label.to_string()
                            icon_svg=config.icon_svg
                            active=move || current_page.get() == config.page
                            on_click=move |p| set_current_page.set(p)
                            page=config.page
                            collapsed=collapsed
                        />
                    }
                }).collect_view()}
            </nav>

            <div class="p-4 border-t border-slate-800">
                <button 
                    on:click=move |_| set_collapsed.update(|c| *c = !*c)
                    class="flex items-center gap-3 w-full px-3 py-2 text-sm text-slate-400 hover:text-white transition-colors"
                >
                    <span class="text-lg">"◀"</span>
                    {move || if collapsed.get() { "".to_string() } else { "收起菜单".to_string() }}
                </button>
            </div>
        </aside>
    }
}
