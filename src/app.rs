use leptos::prelude::*;

pub enum Page {
    Overview,
    Purchase,
    Master,
    Logistics,
    Summary,
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="flex h-screen bg-slate-950 text-slate-50 font-sans overflow-hidden">
            <aside class="w-64 h-full border-r border-slate-800 bg-slate-950/50 flex flex-col">
                <div class="p-6 text-xl font-bold tracking-tight text-green-500">"MATFLOW"</div>
                <nav class="flex-1 px-4 space-y-2">
                    <NavItem label="概览".to_string() icon_svg=get_dashboard_icon() active=true />
                    <NavItem label="采购计划".to_string() icon_svg=get_cart_icon() active=false />
                    <NavItem label="物料主库".to_string() icon_svg=get_inventory_icon() active=false />
                    <NavItem label="入出库流水".to_string() icon_svg=get_analytics_icon() active=false />
                    <NavItem label="库存汇总".to_string() icon_svg=get_settings_icon() active=false />
                </nav>
                <div class="p-4 border-t border-slate-800">
                    <div class="text-xs text-slate-500 px-3">"v1.0.0-beta"</div>
                </div>
            </aside>
            <div class="flex-1 flex flex-col overflow-hidden">
                <Topbar />
                <main class="flex-1 overflow-y-auto p-8 space-y-8">
                    <div class="grid grid-cols-1 md:grid-cols-4 gap-6">
                        <StatCard label="Total Stock" value="12,482" trend="+12%" trend_color="green" />
                        <StatCard label="Inbound Today" value="428" trend="" trend_color="default" />
                        <StatCard label="Pending Orders" value="15" trend="3 Urgent" trend_color="amber" />
                        <StatCard label="System Health" value="99.9%" trend="Optimal" trend_color="green" />
                    </div>
                </main>
            </div>
        </div>
    }
}

fn get_dashboard_icon() -> &'static str {
    r#"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect width="7" height="9" x="3" y="3" rx="1"/><rect width="7" height="5" x="14" y="3" rx="1"/><rect width="7" height="9" x="14" y="13" rx="1"/><rect width="7" height="5" x="3" y="13" rx="1"/></svg>"#
}

fn get_inventory_icon() -> &'static str {
    r#"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 8a2 2 0 0 0-2-2H3a2 2 0 0 0-2 2v8a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V8z"/><path d="M3 8V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v3"/></svg>"#
}

fn get_cart_icon() -> &'static str {
    r#"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="8" cy="21" r="1"/><circle cx="19" cy="21" r="1"/><path d="M2 12.09A25 25 0 0 1 22 12H2"/></svg>"#
}

fn get_analytics_icon() -> &'static str {
    r#"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M3 3v18h18"/><path d="m19 16-5-5-4 4-3-3"/></svg>"#
}

fn get_settings_icon() -> &'static str {
    r#"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 2a2 2 0 0 1 2 2v1.5"/><path d="M18 2a2 2 0 0 1 2 2v1.5"/><path d="M2 2a2 2 0 0 0-2 2v1.5"/><path d="M12 12h.01"/><path d="M16 12a4 4 0 1 0-8 0 4 4 0 0 0 8 0z"/><path d="M12 18a3 3 0 1 0 0 6 3 3 0 0 0 0-6z"/></svg>"#
}

#[component]
pub fn NavItem(label: String, icon_svg: &'static str, active: bool) -> impl IntoView {
    let style = if active {
        "bg-slate-800 text-white shadow-sm"
    } else {
        "text-slate-400 hover:bg-slate-900 hover:text-slate-200"
    };
    view! {
        <div class=format!("flex items-center gap-3 px-3 py-2 rounded-lg transition-colors duration-200 cursor-pointer {}", style)>
            <div class="flex items-center justify-center w-5 h-5">
                <div inner_html=icon_svg />
            </div>
            <span class="text-sm font-medium">{label}</span>
        </div>
    }.into_view()
}

#[component]
pub fn Topbar() -> impl IntoView {
    view! {
        <header class="h-16 border-b border-slate-800 flex items-center justify-between px-8 bg-slate-950/20 backdrop-blur-sm">
            <div class="text-slate-400 text-sm font-medium">"MATFLOW Management System"</div>
            <div class="flex items-center gap-4">
                <div class="relative">
                    <input class="bg-slate-900 border border-slate-700 rounded-full px-4 py-1 text-sm text-slate-200 focus:outline-none focus:ring-2 focus:ring-green-500/50 transition-all w-64" placeholder="Search materials..." />
                </div>
                <div class="w-8 h-8 rounded-full bg-gradient-to-tr from-green-500 to-emerald-600 border border-slate-700 cursor-pointer"></div>
            </div>
        </header>
    }
}

#[component]
pub fn StatCard(label: &'static str, value: &'static str, trend: &'static str, trend_color: &'static str) -> impl IntoView {
    let trend_class = match trend_color {
        "green" => "text-green-500",
        "amber" => "text-amber-500",
        _ => "text-slate-500",
    };
    view! {
        <div class="bg-slate-900/50 backdrop-blur-md border border-slate-800 rounded-2xl p-6 shadow-xl">
            <div class="text-slate-400 text-xs font-medium uppercase tracking-wider">{label}</div>
            <div class="text-2xl font-bold mt-1 font-mono">{value}</div>
            <div class=format!("text-xs mt-2 {}", if !trend.is_empty() { trend_class } else { "text-slate-500" })>{trend}</div>
        </div>
    }
}
