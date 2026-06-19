use crate::data::mock::MockStore;
use leptos::prelude::*;

#[component]
pub fn InboundPage(store: MockStore) -> impl IntoView {
    let inbound_logs = store.inbound;

    view! {
        <div class="bg-slate-900/30 backdrop-blur-md border border-slate-800 rounded-2xl p-6 shadow-xl">
            <div class="flex items-center justify-between mb-6">
                <div class="flex flex-col">
                    <h2 class="text-xl font-semibold text-slate-200">"入库流水记录"</h2>
                    <p class="text-sm text-slate-500">"记录物料实际到货情况 $\rightarrow$ 驱动库存增加"</p>
                </div>
                <div class="flex gap-3">
                    <button class="bg-green-600 hover:bg-green-700 text-white px-4 py-2 rounded-lg text-sm font-medium transition-colors flex items-center gap-2">
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
                        "录入到货"
                    </button>
                </div>
            </div>

            <div class="mb-6 flex items-center gap-3">
                <div class="relative flex-1">
                    <span class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none text-slate-500">
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/></svg>
                    </span>
                    <input
                        type="text"
                        class="w-full bg-slate-950 border border-slate-700 rounded-lg pl-10 pr-4 py-2 text-sm text-slate-200 focus:outline-none focus:ring-2 focus:ring-green-500/50 transition-all"
                        placeholder="搜索日期、产品编号..."
                    />
                </div>
            </div>

            <div class="overflow-x-auto rounded-xl border border-slate-800">
                <table class="w-full text-left border-collapse">
                    <thead>
                        <tr class="bg-slate-800/50">
                            <th class="py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"入库日期"</th>
                            <th class="py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"产品编号"</th>
                            <th class="py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"物料名称"</th>
                            <th class="py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider text-center">"入库数量"</th>
                            <th class="py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"单位"</th>
                            <th class="py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"备注"</th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-slate-800 bg-slate-950/20">
                        {inbound_logs.into_iter().map(|log| {
                            let product_name = log.name.clone().unwrap_or_else(|| "未知物料".to_string());
                            let qty = log.get_quantity();

                            view! {
                                <tr class="hover:bg-slate-800/30 transition-colors group">
                                    <td class="py-3 px-4 text-sm text-slate-400 font-mono">{log.date.unwrap_or_default()}</td>
                                    <td class="py-3 px-4 text-sm font-mono text-green-400">{log.product_id}</td>
                                    <td class="py-3 px-4 text-sm text-slate-200 font-medium">{product_name}</td>
                                    <td class="py-3 px-4 text-sm text-slate-100 text-center font-bold font-mono">{qty}</td>
                                    <td class="py-3 px-4 text-sm text-slate-400">{log.unit.unwrap_or_else(|| "件".to_string())}</td>
                                    <td class="py-3 px-4 text-sm text-slate-500 italic">{log.remark.unwrap_or_else(|| "-".to_string())}</td>
                                </tr>
                            }
                        }).collect::<Vec<_>>()}
                    </tbody>
                </table>
            </div>
        </div>
    }
}
