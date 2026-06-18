use leptos::prelude::*;
use crate::pages::{InboundPage, OutboundPage};

#[component]
pub fn LogisticsPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div class="flex items-center justify-between">
                <div>
                    <h2 class="text-xl font-semibold text-slate-200">"物流管理"</h2>
                    <p class="text-sm text-slate-500">"Inbound & Outbound Transaction Logs"</p>
                </div>
            </div>
            <div class="grid grid-cols-1 gap-6">
                <InboundPage />
                <OutboundPage />
            </div>
        </div>
    }
}
