use leptos::prelude::*;

#[component]
pub fnOverviewPage() -> impl IntoView {
    view! {
        <div class="bg-slate-900/30 backdrop-blur-md border border-slate-800 rounded-2xl p-6 shadow-xl">
            <h2 class="text-xl font-semibold text-slate-200 mb-6">"系统概览"</h2>
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                <StatCard label="Total Stock" value="12,482" trend="+12%" trend_color="green" />
                <StatCard label="Inbound Today" value="428" trend="" trend_color="default" />
                <StatCard label="Pending Orders" value="15" trend="3 Urgent" trend_color="amber" />
                <StatCard label="System Health" value="99.9%" trend="Optimal" trend_color="green" />
            </div>
            <div class="mt-8">
                <h3 class="text-lg font-semibold text-slate-200 mb-4">"Quick Actions"</h3>
                <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                    <button class="bg-slate-800 hover:bg-slate-700 text-slate-200 p-4 rounded-xl transition-colors flex flex-col items-center gap-2 border border-slate-700">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/><polyline points="10 9 9 9 8 9"/></svg>
                        <span class="text-sm font-medium">"New Purchase"</span>
                    </button>
                    <button class="bg-slate-800 hover:bg-slate-700 text-slate-200 p-4 rounded-xl transition-colors flex flex-col items-center gap-2 border border-slate-700">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 8a2 2 0 0 0-2-2H3a2 2 0 0 0-2 2v8a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V8z"/><path d="M3 8V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v3"/><path d="M12 15v-5"/><path d="M8 11h8"/></svg>
                        <span class="text-sm font-medium">"Add Material"</span>
                    </button>
                    <button class="bg-slate-800 hover:bg-slate-700 text-slate-200 p-4 rounded-xl transition-colors flex flex-col items-center gap-2 border border-slate-700">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/></svg>
                        <span class="text-sm font-medium">"New Inbound"</span>
                    </button>
                    <button class="bg-slate-800 hover:bg-slate-700 text-slate-200 p-4 rounded-xl transition-colors flex flex-col items-center gap-2 border border-slate-700">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/><path d="M12 10V4"/></svg>
                        <span class="text-sm font-medium">"New Outbound"</span>
                    </button>
                </div>
            </div>
        </div>
    }
}

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
