use leptos::prelude::*;

#[component]
pub fn MasterPage() -> impl IntoView {
    // Mock data for material master database
    let master_data = create_signal(vec![
        MasterItem {
            id: 1,
            product_id: "MAT-001".to_string(),
            material_name: "乳胶漆".to_string(),
            specification: "10kg/桶".to_string(),
            category: "涂料".to_string(),
            unit: "桶".to_string(),
            minimum_stock: 10,
            last_updated: "2024-01-15".to_string(),
        },
        MasterItem {
            id: 2,
            product_id: "MAT-002".to_string(),
            material_name: "地板".to_string(),
            specification: "1200x120mm".to_string(),
            category: "地面材料".to_string(),
            unit: "平方米".to_string(),
            minimum_stock: 50,
            last_updated: "2024-01-14".to_string(),
        },
        MasterItem {
            id: 3,
            product_id: "MAT-003".to_string(),
            material_name: "瓷砖".to_string(),
            specification: "600x600mm".to_string(),
            category: "墙面材料".to_string(),
            unit: "平方米".to_string(),
            minimum_stock: 100,
            last_updated: "2024-01-16".to_string(),
        },
    ]);

    view! {
        <div class="bg-slate-900/30 backdrop-blur-md border border-slate-800 rounded-2xl p-6 shadow-xl">
            <div class="flex items-center justify-between mb-6">
                <div>
                    <h2 class="text-xl font-semibold text-slate-200">"物料主库"</h2>
                    <p class="text-sm text-slate-500">"Centralized material database with unified product IDs"</p>
                </div>
                <button class="bg-green-600 hover:bg-green-700 text-white px-4 py-2 rounded-lg text-sm font-medium transition-colors flex items-center gap-2">
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
                    "Add Material"
                </button>
            </div>

            <div class="mb-4 flex items-center gap-3">
                <div class="relative flex-1">
                    <input 
                        type="text" 
                        class="w-full bg-slate-950 border border-slate-700 rounded-lg px-4 py-2 text-sm text-slate-200 focus:outline-none focus:ring-2 focus:ring-green-500/50 transition-all"
                        placeholder="Search by Product ID, Material Name..."
                    />
                </div>
                <select class="bg-slate-950 border border-slate-700 rounded-lg px-3 py-2 text-sm text-slate-200 focus:outline-none focus:ring-2 focus:ring-green-500/50 transition-all">
                    <option value="all">All Categories</option>
                    <option value="paint">涂料</option>
                    <option value="floor">地面材料</option>
                    <option value="wall">墙面材料</option>
                </select>
            </div>

            <div class="overflow-x-auto">
                <table class="w-full">
                    <thead>
                        <tr class="border-b border-slate-800">
                            <th class="text-left py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"Product ID"</th>
                            <th class="text-left py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"Material Name"</th>
                            <th class="text-left py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"Specification"</th>
                            <th class="text-left py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"Category"</th>
                            <th class="text-left py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"Unit"</th>
                            <th class="text-left py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"Min Stock"</th>
                            <th class="text-left py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"Last Updated"</th>
                            <th class="text-center py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"Actions"</th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-slate-800">
                        {move || {
                            master_data.with(|data| {
                                data.iter().map(|item| {
                                    view! {
                                        <tr class="hover:bg-slate-800/30 transition-colors">
                                            <td class="py-3 px-4 text-sm text-green-400 font-mono">{&item.product_id}</td>
                                            <td class="py-3 px-4 text-sm text-slate-300">{&item.material_name}</td>
                                            <td class="py-3 px-4 text-sm text-slate-400">{&item.specification}</td>
                                            <td class="py-3 px-4 text-sm text-slate-300">{&item.category}</td>
                                            <td class="py-3 px-4 text-sm text-slate-400">{&item.unit}</td>
                                            <td class="py-3 px-4 text-sm text-slate-300">{item.minimum_stock}</td>
                                            <td class="py-3 px-4 text-sm text-slate-400">{&item.last_updated}</td>
                                            <td class="py-3 px-4 text-center">
                                                <div class="flex items-center justify-center gap-2">
                                                    <button class="text-slate-400 hover:text-green-400 transition-colors">
                                                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/><path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/></svg>
                                                    </button>
                                                    <button class="text-slate-400 hover:text-red-400 transition-colors">
                                                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 6h18"/><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/></svg>
                                                    </button>
                                                </div>
                                            </td>
                                        </tr>
                                    }
                                }).collect::<Vec<_>>()
                            })
                        }}
                    </tbody>
                </table>
            </div>
        </div>
    }
}

#[derive(Clone, Debug)]
struct MasterItem {
    id: usize,
    product_id: String,
    material_name: String,
    specification: String,
    category: String,
    unit: String,
    minimum_stock: i32,
    last_updated: String,
}
