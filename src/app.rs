use crate::components::{Aside, NavConfig, Topbar};
use crate::pages::*;
use leptos::prelude::*;

use crate::utils::icons::*;

struct PageRoute {
    page: Page,
    label: &'static str,
    icon_svg: &'static str,
}

#[derive(Clone, Copy, PartialEq)]
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

    let routes = vec![
        PageRoute { page: Page::Overview, label: "首页", icon_svg: get_dashboard_icon() },
        PageRoute { page: Page::Purchase, label: "采购管理", icon_svg: get_cart_icon() },
        PageRoute { page: Page::Master, label: "物料信息", icon_svg: get_inventory_icon() },
        PageRoute { page: Page::Logistics, label: "入出库流水", icon_svg: get_analytics_icon() },
        PageRoute { page: Page::Summary, label: "库存记录", icon_svg: get_settings_icon() },
    ];

    let nav_configs = routes.iter().map(|r| NavConfig {
        page: r.page.clone(),
        label: r.label,
        icon_svg: r.icon_svg,
    }).collect::<Vec<_>>();

    view! {
        <div class="flex h-screen bg-slate-950 text-slate-50 font-sans overflow-hidden">
            <Aside 
                current_page=current_page 
                set_current_page=set_current_page 
                nav_configs=nav_configs
            />
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
