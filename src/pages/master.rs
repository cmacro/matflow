use leptos::prelude::*;
use crate::data::mock::MockStore;
use crate::components::{StatGrid, StatItem};

#[component]
pub fn MasterPage(store: MockStore) -> impl IntoView {
    let products = store.products;

    let stats = vec![
        StatItem {
            label: "物料总数".to_string(),
            value: format!("{} 项", products.len()),
            icon_svg: r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="18" height="18" x="3" y="3" rx="2" ry="2"/><line x1="3" y1="9" x2="21" y2="9"/><line x1="3" y1="15" x2="21" y2="15"/><line x1="9" y1="3" x2="9" y2="21"/></svg>"#.to_string(),
            color_class: "bg-blue-500/10 text-blue-400".to_string(),
        },
        StatItem {
            label: "覆盖项目".to_string(),
            value: "3 个".to_string(),
            icon_svg: r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m7.5 4.27 9 9.73"/><path d="M12 12 21 21"/><path d="m4.5 16.27 9-9.73"/><path d="M3 12h18"/></svg>"#.to_string(),
            color_class: "bg-purple-500/10 text-purple-400".to_string(),
        },
        StatItem {
            label: "数据状态".to_string(),
            value: "同步中".to_string(),
            icon_svg: r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>"#.to_string(),
            color_class: "bg-amber-500/10 text-amber-400".to_string(),
        },
    ];

    view! {
        <div class="bg-slate-900/30 backdrop-blur-md border border-slate-800 rounded-2xl p-6 shadow-xl">
            <div class="flex items-center justify-between mb-6">
                <div class="flex flex-col">
                    <h2 class="text-xl font-semibold text-slate-200">"物料主数据库"</h2>
                    <p class="text-sm text-slate-500">"物料标准字典：所有出入库操作的唯一真值源"</p>
                </div>
                <div class="flex gap-3">
                    <button class="bg-green-600 hover:bg-green-700 text-white px-4 py-2 rounded-lg text-sm font-medium transition-colors flex items-center gap-2">
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
                        "同步采购计划"
                    </button>
                </div>
            </div >

            <StatGrid items=stats />

            <div class="overflow-x-auto rounded-xl border border-slate-800">
                <table class="w-full text-left border-collapse">
                    <thead>
                        <tr class="bg-slate-800/50">
                            <th class="py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"产品编号"</th>
                            <th class="py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"产品名称"</th>
                            <th class="py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"规格型号"</th>
                            <th class="py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"单位"</th>
                            <th class="py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"类别"</th>
                            <th class="py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider text-center">"操作"</th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-slate-800 bg-slate-950/20">
                        {products.into_iter().map(|item| {
                            view! {
                                <tr class="hover:bg-slate-800/30 transition-colors group">
                                    <td class="py-3 px-4 text-sm font-mono text-green-400">{item.product_id}</td>
                                    <td class="py-3 px-4 text-sm text-slate-200 font-medium">{item.name}</td>
                                    <td class="py-3 px-4 text-sm text-slate-400">{item.spec.unwrap_or_else(|| "标准规格".to_string())}</td>
                                    <td class="py-3 px-4 text-sm text-slate-400">{item.unit.unwrap_or_else(|| "件".to_string())}</td>
                                    <td class="py-3 px-4 text-sm">
                                        <span class="px-2 py-0.5 rounded-full text-xs bg-slate-800 text-slate-400 border border-slate-700">
                                            {item.category.unwrap_or_else(|| "未分类".to_string())}
                                        </span>
                                    </td>
                                    <td class="py-3 px-4 text-center">
                                        <button class="text-slate-500 hover:text-white transition-colors p-1">
                                            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M17 3a5 5 0 0 0-9.95 0"/><path d="M12 20V10"/></svg>
                                        </button>
                                    </td>
                                </tr>
                            }
                        }).collect::<Vec<_>>()}
                    </tbody>
                </table>
            </div >
        </div>
    }
}
