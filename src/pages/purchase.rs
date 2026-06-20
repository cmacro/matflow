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

    // 导入对话框显示状态
    let (show_import, set_show_import) = signal(false);

    view! {
        <div class="space-y-0">
        <div class="bg-slate-900/30 backdrop-blur-md border border-slate-800 rounded-2xl p-6 shadow-xl">
            <div class="flex items-center justify-between mb-6">
                <div class="flex flex-col">
                    <h2 class="text-xl font-semibold text-slate-200">"采购计划管理"</h2>
                    <p class="text-sm text-slate-500">"数据量: " {purchases.len()} " 条记录"</p>
                </div>
                <div class="flex gap-3">
                    <button
                        on:click=move |_| set_show_import.set(true)
                        class="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg text-sm font-medium transition-colors flex items-center gap-2"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="17 8 12 3 7 8"/><line x1="12" y1="3" x2="12" y2="15"/></svg>
                        "导入"
                    </button>
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

        // 导入对话框
        <Show when=move || show_import.get()>
            <div
                class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm"
                on:click=move |_| set_show_import.set(false)
            >
                <div
                    class="bg-slate-900 border border-slate-700 rounded-2xl shadow-2xl w-full max-w-lg mx-4 overflow-hidden"
                    on:click=move |ev| ev.stop_propagation()
                >
                    // 对话框头部
                    <div class="flex items-center justify-between px-6 py-4 border-b border-slate-800">
                        <div class="flex items-center gap-3">
                            <div class="p-2 bg-blue-500/10 rounded-lg text-blue-400">
                                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="17 8 12 3 7 8"/><line x1="12" y1="3" x2="12" y2="15"/></svg>
                            </div>
                            <h3 class="text-lg font-semibold text-slate-200">"导入采购数据"</h3>
                        </div>
                        <button
                            on:click=move |_| set_show_import.set(false)
                            class="text-slate-500 hover:text-slate-200 transition-colors"
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
                        </button>
                    </div>

                    // 对话框主体
                    <div class="px-6 py-5 space-y-5">
                        // 文件拖放区
                        <div class="border-2 border-dashed border-slate-700 rounded-xl p-8 text-center hover:border-blue-500/50 transition-colors cursor-pointer">
                            <div class="flex justify-center mb-3 text-slate-500">
                                <svg xmlns="http://www.w3.org/2000/svg" width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/></svg>
                            </div>
                            <p class="text-sm text-slate-300 mb-1">"点击选择文件或拖拽到此处"</p>
                            <p class="text-xs text-slate-500">"支持 Excel (.xlsx) 和 JSON 格式"</p>
                        </div>

                        // 导入选项
                        <div class="space-y-3">
                            <label class="flex items-center gap-3 text-sm text-slate-300 cursor-pointer">
                                <input type="checkbox" class="rounded border-slate-600 bg-slate-800 text-blue-500 focus:ring-blue-500/50" checked />
                                "遇重复编号时跳过"
                            </label>
                            <label class="flex items-center gap-3 text-sm text-slate-300 cursor-pointer">
                                <input type="checkbox" class="rounded border-slate-600 bg-slate-800 text-blue-500 focus:ring-blue-500/50" />
                                "导入后自动同步至物料主库"
                            </label>
                        </div>
                    </div>

                    // 对话框底部
                    <div class="flex items-center justify-end gap-3 px-6 py-4 border-t border-slate-800 bg-slate-950/30">
                        <button
                            on:click=move |_| set_show_import.set(false)
                            class="px-4 py-2 rounded-lg text-sm font-medium text-slate-300 hover:bg-slate-800 transition-colors border border-slate-700"
                        >
                            "取消"
                        </button>
                        <button
                            on:click=move |_| {
                                web_sys::console::log_1(&"Demo: import triggered".into());
                                set_show_import.set(false);
                            }
                            class="px-4 py-2 rounded-lg text-sm font-medium bg-blue-600 hover:bg-blue-700 text-white transition-colors flex items-center gap-2"
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg>
                            "开始导入"
                        </button>
                    </div>
                </div>
            </div>
        </Show>
        </div>
    }
}
