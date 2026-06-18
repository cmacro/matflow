use crate::components::{NavItem, Topbar};
use crate::pages::*;
use leptos::prelude::*;

use crate::utils::icons::*;

#[derive(Clone, PartialEq)]
pub enum Page {
    Overview,
    Purchase,
    Master,
    Logistics,
    Summary,
}

#[component]
pub fn App() -> impl IntoView {
    let (current_page, set_current_page) = signal(Page::Overview);

    view! {
        <div class="flex h-screen bg-slate-950 text-slate-50 font-sans overflow-hidden">
            <aside class="w-64 h-full border-r border-slate-800 bg-slate-950/50 flex flex-col">
                <div class="p-6 text-xl font-bold tracking-tight text-green-500">"MATFLOW"</div>
                <nav class="flex-1 px-4 space-y-2">
                    <NavItem
                        label={"概览".to_string()}
                        icon_svg=get_dashboard_icon()
                        active=move || current_page.get() == Page::Overview
                        on_click=move |page| set_current_page.set(page)
                        page=Page::Overview
                    />
                    <NavItem
                        label={"采购计划".to_string()}
                        icon_svg=get_cart_icon()
                        active=move || current_page.get() == Page::Purchase
                        on_click=move |page| set_current_page.set(page)
                        page=Page::Purchase
                    />
                    <NavItem
                        label={"物料主库".to_string()}
                        icon_svg=get_inventory_icon()
                        active=move || current_page.get() == Page::Master
                        on_click=move |page| set_current_page.set(page)
                        page=Page::Master
                    />
                    <NavItem
                        label={"入出库流水".to_string()}
                        icon_svg=get_analytics_icon()
                        active=move || current_page.get() == Page::Logistics
                        on_click=move |page| set_current_page.set(page)
                        page=Page::Logistics
                    />
                    <NavItem
                        label={"库存汇总".to_string()}
                        icon_svg=get_settings_icon()
                        active=move || current_page.get() == Page::Summary
                        on_click=move |page| set_current_page.set(page)
                        page=Page::Summary
                    />
                </nav>
                <div class="p-4 border-t border-slate-800">
                    <div class="text-xs text-slate-500 px-3">"v1.0.0-beta"</div>
                </div>
            </aside>
            <div class="flex-1 flex flex-col overflow-hidden">
                <Topbar />
                <main class="flex-1 overflow-y-auto p-8 space-y-8">
                    {move || {
                        match current_page.get() {
                            Page::Overview => { view! { <OverviewPage/> }.into_any() }
                            Page::Purchase => { view! { <PurchasePage/> }.into_any() }
                            Page::Master => { view! { <MasterPage/> }.into_any() }
                            Page::Logistics => { view! { <LogisticsPage/> }.into_any() }
                            Page::Summary => { view! { <SummaryPage/> }.into_any() }
                        }
                    }}
                </main>
            </div>
        </div>
    }
}
