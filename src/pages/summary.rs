use crate::data::mock::MockStore;
use leptos::prelude::*;

#[component]
pub fn SummaryPage(store: MockStore) -> impl IntoView {
    let summary_data = store.summary;

    view! {
        <div class="bg-slate-900/30 backdrop-blur-md border border-slate-800 rounded-2xl p-6 shadow-xl">
            <div class="flex items-center justify-between mb-6">
                <div class="flex flex-col">
                    <h2 class="text-xl font-semibold text-slate-200">"库存实时汇总"</h2>
                    <p class="text-sm text-slate-500">"实时结存统计：基于入出库流水的自动聚合结果"</p>
                </div>
                <div class="flex gap-3">
                    <button class="bg-slate-800 hover:bg-slate-700 text-slate-300 px-4 py-2 rounded-lg text-sm font-medium transition-colors flex items-center gap-2 border border-slate-700">
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
                        "导出库存报表"
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
                        class="w-full bg-slate-950 border border-slate-700 rounded-lg pl-10 pr-4 py-2 text-sm text-slate-200 focus:outline-none focus:ring-2 focus:ring-blue-500/50 transition-all"
                        placeholder="搜索产品编号、名称..."
                    />
                </div>
            </div>

            <div class="overflow-x-auto rounded-xl border border-slate-800">
                <table class="w-full text-left border-collapse">
                    <thead>
                        <tr class="bg-slate-800/50">
                            <th class="py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"产品编号"</th>
                            <th class="py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"产品名称"</th>
                            <th class="py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"规格型号"</th>
                            <th class="py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider text-center">"累计入库"</th>
                            <th class="py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider text-center">"累计出库"</th>
                            <th class="py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider text-right">"期末结存"</th>
                            <th class="py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider text-center">"状态"</th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-slate-800 bg-slate-950/20">
                        {summary_data.into_iter().map(|item| {
                            let stock_val = item.stock_f64();
                            let is_low_stock = stock_val < 1.0;
                            let total_in_val = item.total_in_f64();
                            let total_out_val = item.total_out_f64();
                            let calc_stock = total_in_val - total_out_val;
                            let is_verified = (calc_stock - stock_val).abs() < 0.001;
                            let product_id = item.product_id.clone();
                            let name = item.name.clone();
                            let spec = item.spec.as_ref().map(|s| s.clone()).unwrap_or_else(|| "-".to_string());

                            view! {
                                <tr class="hover:bg-slate-800/30 transition-colors group">
                                    <td class="py-3 px-4 text-sm font-mono text-blue-400">{product_id}</td>
                                    <td class="py-3 px-4 text-sm text-slate-200 font-medium">{name}</td>
                                    <td class="py-3 px-4 text-sm text-slate-400">{spec}</td>
                                    <td class="py-3 px-4 text-sm text-slate-300 text-center font-mono">{total_in_val}</td>
                                    <td class="py-3 px-4 text-sm text-slate-300 text-center font-mono">{total_out_val}</td>
                                    <td class="py-3 px-4 text-sm font-bold text-right font-mono text-slate-100">
                                        {stock_val}
                                    </td>
                                    <td class="py-3 px-4 text-center">
                                        <div class="flex items-center justify-center gap-2">
                                            <span class=format!(
                                                "px-2 py-0.5 rounded-full text-[10px] font-bold uppercase border {}",
                                                if is_low_stock { "bg-red-500/10 border-red-500/20 text-red-400" } else { "bg-green-500/10 border-green-500/20 text-green-400" }
                                            )>
                                                {if is_low_stock { "缺货" } else { "充足" }}
                                            </span>
                                            <span class=format!(
                                                "px-1.5 py-0.5 rounded-full text-[10px] font-bold border {}",
                                                if is_verified { "bg-blue-500/10 border-blue-500/20 text-blue-400" } else { "bg-amber-500/10 border-amber-500/20 text-amber-400" }
                                            )>
                                                {if is_verified { "✓ 验证" } else { "⚠ 差异" }}
                                            </span>
                                        </div>
                                    </td>
                                </tr>
                            }
                        }).collect::<Vec<_>>()}
                    </tbody>
                </table>
            </div>
        </div>
    }
}
