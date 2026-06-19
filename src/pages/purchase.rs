use crate::data::mock::MockStore;
use crate::models::PurchaseItem;
use leptos::prelude::*;

#[component]
pub fn PurchasePage(store: MockStore) -> impl IntoView {
    let purchases = store.purchases;

    // 打印数据到浏览器控制台
    web_sys::console::log_1(&format!("Purchases count: {}", purchases.len()).into());
    web_sys::console::log_1(
        &format!(
            "First item ID: {:?}",
            purchases.get(0).map(|i| &i.product_id)
        )
        .into(),
    );

    view! {
        <div class="bg-slate-900/30 backdrop-blur-md border border-slate-800 rounded-2xl p-6 shadow-xl">
            <div class="flex items-center justify-between mb-6">
                <div class="flex flex-col">
                    <h2 class="text-xl font-semibold text-slate-200">"采购计划管理"</h2>
                    <p class="text-sm text-slate-500">"数据量: " {purchases.len()} " 条记录"</p>
                </div>
                <div class="flex gap-3">
                    <button class="bg-slate-800 hover:bg-slate-700 text-slate-300 px-4 py-2 rounded-lg text-sm font-medium transition-colors flex items-center gap-2 border border-slate-700">
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
                        "导出清单"
                    </button>
                    <button class="bg-green-600 hover:bg-green-700 text-white px-4 py-2 rounded-lg text-sm font-medium transition-colors flex items-center gap-2">
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
                        "新增采购"
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
                        placeholder="搜索产品编号、项目名称、物料名称..."
                    />
                </div>
                <select class="bg-slate-950 border border-slate-700 rounded-lg px-3 py-2 text-sm text-slate-200 focus:outline-none focus:ring-2 focus:ring-green-500/50 transition-all">
                    <option value="all">所有项目</option>
                    <option value="project1">青岛中巍白云山居</option>
                    <option value="project2">象屿六叠中平</option>
                </select>
            </div>

            <div class="overflow-x-auto rounded-xl border border-slate-800">
                <table class="w-full text-left border-collapse">
                    <thead>
                        <tr class="bg-slate-800/50">
                            <th class="py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"序号"</th>
                            <th class="py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"产品编号"</th>
                            <th class="py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"项目名称"</th>
                            <th class="py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"位置"</th>
                            <th class="py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"物料名称"</th>
                            <th class="py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"规格尺寸"</th>
                            <th class="py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider text-right">"数量"</th>
                            <th class="py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"单位"</th>
                            <th class="py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider text-right">"单价"</th>
                            <th class="py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"类别"</th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-slate-800 bg-slate-950/20">
                        {purchases.into_iter().map(|item| {
                            view! {
                                <tr class="hover:bg-slate-800/30 transition-colors group">
                                    <td class="py-3 px-4 text-sm text-slate-500">{item.id.unwrap_or(0)}</td>
                                    <td class="py-3 px-4 text-sm font-mono text-green-400">{item.product_id}</td>
                                    <td class="py-3 px-4 text-sm text-slate-300">{item.project_name.unwrap_or_default()}</td>
                                    <td class="py-3 px-4 text-sm text-slate-400">{item.location.unwrap_or_default()}</td>
                                    <td class="py-3 px-4 text-sm text-slate-200 font-medium">{item.name}</td>
                                    <td class="py-3 px-4 text-sm text-slate-400">{item.size.unwrap_or_default()}</td>
                                    <td class="py-3 px-4 text-sm text-slate-300 text-right font-mono">{PurchaseItem::parse_f64(&item.quantity)}</td>
                                    <td class="py-3 px-4 text-sm text-slate-400">{item.unit.unwrap_or_default()}</td>
                                    <td class="py-3 px-4 text-sm text-slate-300 text-right font-mono">{format!("¥{:.2}", PurchaseItem::parse_f64(&item.unit_price))}</td>
                                    <td class="py-3 px-4 text-sm">
                                        <span class="px-2 py-0.5 rounded-full text-xs bg-slate-800 text-slate-400 border border-slate-700">
                                            {item.category.unwrap_or_else(|| "未分类".to_string())}
                                        </span>
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
