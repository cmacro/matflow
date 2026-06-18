// src/components/stat_card.rs
use leptos::prelude::*;

#[component]
pub fn StatCard(
    label: String,
    value: String,
    trend: String,
    trend_color: String,
) -> impl IntoView {
    let trend_color_class = match trend_color.as_str() {
        "green" => "text-green-400",
        "amber" => "text-amber-400",
        "red" => "text-red-400",
        _ => "text-slate-400",
    };

    view! {
        <div class="bg-slate-900/50 border border-slate-700 rounded-2xl p-6 hover:border-slate-600 transition-colors">
            <div class="text-sm text-slate-400 mb-2">{label}</div>
            <div class="text-3xl font-semibold text-white tracking-tight mb-3">{value}</div>
            
            {if !trend.is_empty() {
                view! {
                    <div class=format!("text-sm font-medium {}", trend_color_class)>
                        {trend}
                    </div>
                }.into_any()
            } else {
                view! { <div class="h-5"></div> }.into_any()
            }}
        </div>
    }
}
