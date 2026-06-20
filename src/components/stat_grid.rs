use leptos::prelude::*;

#[derive(Clone)]
pub struct StatItem {
    pub label: String,
    pub value: String,
    pub icon_svg: String,
    pub color_class: String,
}

#[component]
pub fn StatGrid(items: Vec<StatItem>) -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
            {items.into_iter().map(|item| {
                view! {
                    <div class="bg-slate-950/50 border border-slate-800 p-4 rounded-xl flex items-center gap-4">
                        <div class=format!("p-3 rounded-lg {}", item.color_class)>
                            <div class="text-white">
                                // 这里的 icon_svg 应当是完整的 svg 字符串，但 Leptos view! 无法直接注入 HTML 字符串
                                // 因此我们将在此处使用 inner_html 技巧或者简单的 wrapper
                                <div inner_html=item.icon_svg />
                            </div>
                        </div>
                        <div class="flex flex-col">
                            <div class="text-xs text-slate-500 uppercase font-semibold tracking-wider">{item.label}</div>
                            <div class="text-2xl font-bold text-slate-200">{item.value}</div>
                        </div>
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
