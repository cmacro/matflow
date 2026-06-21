use crate::components::{Aside, NavConfig, Topbar};
use crate::data::mock::MockStore;
use crate::pages::*;
use crate::utils::icons::*;
use leptos::prelude::*;
use std::sync::Arc;

struct PageRoute {
    page: Page,
    label: &'static str,
    icon_svg: &'static str,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Page {
    Overview,
    Purchase,
    Inbound,
    Outbound,
    Summary,
    Master,
    Report,
    Settings,
    System,
}

#[component]
pub fn App() -> impl IntoView {
    let (is_logged_in, set_is_logged_in) = signal(false);
    let (current_page, set_current_page) = signal(Page::Overview);

    let store = Arc::new(MockStore::new());

    let routes = vec![
        PageRoute {
            page: Page::Overview,
            label: "首页",
            icon_svg: get_dashboard_icon(),
        },
        PageRoute {
            page: Page::Purchase,
            label: "采购管理",
            icon_svg: get_cart_icon(),
        },
        PageRoute {
            page: Page::Inbound,
            label: "入库流水",
            icon_svg: get_analytics_icon(),
        },
        PageRoute {
            page: Page::Outbound,
            label: "出库流水",
            icon_svg: get_analytics_icon(),
        },
        PageRoute {
            page: Page::Summary,
            label: "库存记录",
            icon_svg: get_settings_icon(),
        },
        PageRoute {
            page: Page::Master,
            label: "物料信息",
            icon_svg: get_inventory_icon(),
        },
        PageRoute {
            page: Page::Report,
            label: "报表中心",
            icon_svg: get_analytics_icon(),
        },
        PageRoute {
            page: Page::Settings,
            label: "基础设置",
            icon_svg: get_settings_icon(),
        },
        PageRoute {
            page: Page::System,
            label: "系统管理",
            icon_svg: get_settings_icon(),
        },
    ];

    let nav_configs = routes
        .iter()
        .map(|r| NavConfig {
            page: r.page,
            label: r.label,
            icon_svg: r.icon_svg,
        })
        .collect::<Vec<_>>();

    view! {
        <div class="flex h-screen bg-slate-950 text-slate-50 font-sans overflow-hidden">

            {move || {
                if !is_logged_in.get() {
                    view! {
                        <LoginPage
                            on_login=move || {
                                set_is_logged_in.set(true);
                            }
                        />
                    }
                    .into_any()
                } else {
                    let nav_configs = nav_configs.clone();
                    let store = store.clone();

                    view! {
                        <div class="flex w-full h-full">

                            <Aside
                                current_page=current_page
                                set_current_page=set_current_page
                                nav_configs=nav_configs
                            />

                            <div class="flex-1 flex flex-col overflow-hidden">

                                <Topbar />

                                <main class="flex-1 overflow-y-auto p-8 space-y-8">

                                    {move || {
                                        let store: MockStore = store.as_ref().clone();

                                        match current_page.get() {
                                            Page::Overview =>
                                                view! { <OverviewPage /> }.into_any(),

                                            Page::Purchase =>
                                                view! {
                                                    <PurchasePage store=store.clone() />
                                                }.into_any(),

                                            Page::Inbound =>
                                                view! {
                                                    <InboundPage store=store.clone() />
                                                }.into_any(),

                                            Page::Outbound =>
                                                view! {
                                                    <OutboundPage store=store.clone() />
                                                }.into_any(),

                                            Page::Summary =>
                                                view! {
                                                    <SummaryPage store=store.clone() />
                                                }.into_any(),

                                            Page::Master =>
                                                view! {
                                                    <MasterPage store=store.clone() />
                                                }.into_any(),

                                            Page::Report =>
                                                view! { <ReportPage /> }.into_any(),

                                            Page::Settings =>
                                                view! { <SettingsPage /> }.into_any(),

                                            Page::System =>
                                                view! { <SystemPage /> }.into_any(),
                                        }
                                    }}

                                </main>

                            </div>

                        </div>
                    }
                    .into_any()
                }
            }}

        </div>
    }
}
