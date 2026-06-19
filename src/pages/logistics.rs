use crate::data::mock::MockStore;
use crate::pages::inbound::InboundPage;
use crate::pages::outbound::OutboundPage;
use leptos::prelude::*;

#[component]
pub fn LogisticsPage(store: MockStore) -> impl IntoView {
    let inbound_store = store.clone(); // 或后面只传需要的字段
    let outbound_store = store;
    view! {
        <div class="space-y-8">
            <div class="flex items-center justify-between">
                <div class="flex flex-col">
                    <h2 class="text-xl font-semibold text-slate-200">"物流管理"</h2>
                    <p class="text-sm text-slate-500">"Inbound & Outbound Transaction Logs"</p>
                </div>
            </div>
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                <InboundPage store=inbound_store />
                <OutboundPage store=outbound_store />
            </div>
        </div>
    }
}
