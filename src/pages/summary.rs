use leptos::prelude::*;

#[component]
pub fn SummaryPage() -> impl IntoView {
    // Mock data for inventory summary
    let (summary_data, _set_summary_data) = signal(vec![
        SummaryItem {
            id: 1,
            product_id: "MAT-001".to_string(),
            material_name: "乳胶漆".to_string(),
            specification: "10kg/桶".to_string(),
            category: "涂料".to_string(),
            unit: "桶".to_string(),
            total_inbound: 70,
            total_outbound: 20,
            remaining: 50,
            last_updated: "2024-01-22".to_string(),
        },
        SummaryItem {
            id: 2,
            product_id: "MAT-002".to_string(),
            material_name: "地板".to_string(),
            specification: "1200x120mm".to_string(),
            category: "地面材料".to_string(),
            unit: "平方米".to_string(),
            total_inbound: 150,
            total_outbound: 30,
            remaining: 120,
            last_updated: "2024-01-20".to_string(),
        },
        SummaryItem {
            id: 3,
            product_id: "MAT-003".to_string(),
            material_name: "瓷砖".to_string(),
            specification: "600x600mm".to_string(),
            category: "墙面材料".to_string(),
            unit: "平方米".to_string(),
            total_inbound: 300,
            total_outbound: 25,
            remaining: 275,
            last_updated: "2024-01-22".to_string(),
        },
    ]);

    view! {
        <div class="bg-slate-900/30 backdrop-blur-md border border-slate-800 rounded-2xl p-6 shadow-xl">
            <div class="flex items-center justify-between mb-6">
                <div>
                    <h2 class="text-xl font-semibold text-slate-200">"库存汇总"</h2>
                    <p class="text-sm text-slate-500">"Real-time inventory overview with inbound/outbound aggregation"</p>
                </div>
                <div class="flex items-center gap-2">
                    <button class="bg-green-600/20 hover:bg-green-600/30 text-green-400 px-4 py-2 rounded-lg text-sm font-medium transition-colors">
                        "Export CSV"
                    </button>
                    <button class="bg-slate-700 hover:bg-slate-600 text-white px-4 py-2 rounded-lg text-sm font-medium transition-colors">
                        "Refresh"
                    </button>
                </div>
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
                <select class="bg-slate-950 border border-slate-700 rounded-lg px-3 py-2 text-sm text-slate-200 focus:outline-none focus:ring-2 focus:ring-green-500/50 transition-all">
                    <option value="all">All Status</option>
                    <option value="critical">Critical (Low Stock)</option>
                    <option value="normal">Normal</option>
                </select>
            </div>

            <div class="overflow-x-auto">
                <table class="w-full">
                    <thead>
                        <tr class="border-b border-slate-800">&
                            <th class="text-left py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"Product ID"</th>
                            <th class="text-left py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"Material Name"</th>
                            <th class="text-left py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"Specification"</th>
                            <th class="text-left py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"Category"</th>
                            <th class="text-left py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"Unit"</th>
                            <th class="text-right py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"Inbound"</th>
                            <th class="text-right py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"Outbound"</th>
                            <th class="text-right py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"Remaining"</th>
                            <th class="text-left py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"Status"</th>
                            <th class="text-left py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"Last Updated"</th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-slate-800">
                        {move || {
                            summary_data.with(|data| {
                                data.iter().map(|item| {
                                    let stock_status = if item.remaining <= 10 {
                                        "Low Stock"
                                    } else if item.remaining <= 30 {
                                        "Warning"
                                    } else {
                                        "Normal"
                                    };
                                    
                                    // let status_class = match stock_status {
                                    //     "Low Stock" => "text-red-400",
                                    //     "Warning" => "text-amber-400",
                                    //     _ => "text-green-400",
                                    // };

                                    view! {
                                        <tr class="hover:bg-slate-800/30 transition-colors" id={item.id}>
                                            <td class="py-3 px-4 text-sm text-green-400 font-mono">{item.product_id.clone()}</td>
                                            <td class="py-3 px-4 text-sm text-slate-300">{item.material_name.clone()}</td>
                                            <td class="py-3 px-4 text-sm text-slate-400">{item.specification.clone()}</td>
                                            <td class="py-3 px-4 text-sm text-slate-300">{item.category.clone()}</td>
                                            <td class="py-3 px-4 text-sm text-slate-400">{item.unit.clone()}</td>
                                            <td class="py-3 px-4 text-sm text-green-400 text-right font-mono">{item.total_inbound}</td>
                                            <td class="py-3 px-4 text-sm text-red-400 text-right font-mono">{item.total_outbound}</td>
                                            <td class="py-3 px-4 text-sm text-slate-200 text-right font-mono">{item.remaining}</td>
                                            <td class="py-3 px-4">
                                                <span class=format!(
                                                    "inline-flex items-center gap-1.5 px-2 py-1 rounded-full text-xs font-medium border {}",
                                                    match stock_status {
                                                        "Low Stock" => "bg-red-500/10 border-red-500/20 text-red-400",
                                                        "Warning" => "bg-amber-500/10 border-amber-500/20 text-amber-400",
                                                        _ => "bg-green-500/10 border-green-500/20 text-green-400",
                                                    }
                                                )>{stock_status}</span>
                                            </td>
                                            <td class="py-3 px-4 text-sm text-slate-400">{item.last_updated.clone()}</td>
                                        </tr>
                                    }
                                }).collect::<Vec<_>>()
                            })
                        }}
                    </tbody>
                </table>
            </div>

            <div class="mt-6 grid grid-cols-1 md:grid-cols-4 gap-4">
                <div class="bg-slate-950/50 border border-slate-800 rounded-xl p-4">
                    <div class="text-slate-500 text-xs uppercase tracking-wider mb-1">"Total Items"</div>
                    <div class="text-2xl font-bold text-slate-200">{move || summary_data.with(|data| data.len())}</div>
                </div>
                <div class="bg-slate-950/50 border border-slate-800 rounded-xl p-4">
                    <div class="text-slate-500 text-xs uppercase tracking-wider mb-1">"Total Inbound"</div>
                    <div class="text-2xl font-bold text-green-500">{move || summary_data.with(|data| data.iter().map(|i| i.total_inbound).sum::<i32>())}</div>
                </div>
                <div class="bg-slate-950/50 border border-slate-800 rounded-xl p-4">
                    <div class="text-slate-500 text-xs uppercase tracking-wider mb-1">"Total Outbound"</div>
                    <div class="text-2xl font-bold text-red-500">{move || summary_data.with(|data| data.iter().map(|i| i.total_outbound).sum::<i32>())}</div>
                </div>
                <div class="bg-slate-950/50 border border-slate-800 rounded-xl p-4">
                    <div class="text-slate-500 text-xs uppercase tracking-wider mb-1">"Total Remaining"</div>
                    <div class="text-2xl font-bold text-slate-200">{move || summary_data.with(|data| data.iter().map(|i| i.remaining).sum::<i32>())}</div>
                </div>
            </div>
        </div>
    }
}

#[derive(Clone, Debug)]
struct SummaryItem {
    id: usize,
    product_id: String,
    material_name: String,
    specification: String,
    category: String,
    unit: String,
    total_inbound: i32,
    total_outbound: i32,
    remaining: i32,
    last_updated: String,
}
