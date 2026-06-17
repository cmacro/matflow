use leptos::prelude::*;

#[component]
pub fn PurchasePage() -> impl IntoView {
    // Mock data for purchase plan
    let purchase_data = create_signal(vec![
        PurchaseItem {
            id: 1,
            product_id: "MAT-001".to_string(),
            project_name: "客厅装修".to_string(),
            material_name: "乳胶漆".to_string(),
            specification: "10kg/桶".to_string(),
            quantity: 50,
            unit: "桶".to_string(),
            estimated_cost: 2500.0,
            status: "Pending".to_string(),
        },
        PurchaseItem {
            id: 2,
            product_id: "MAT-002".to_string(),
            project_name: "卧室装修".to_string(),
            material_name: "地板".to_string(),
            specification: "1200x120mm".to_string(),
            quantity: 100,
            unit: "平方米".to_string(),
            estimated_cost: 12000.0,
            status: "Approved".to_string(),
        },
        PurchaseItem {
            id: 3,
            product_id: "MAT-003".to_string(),
            project_name: "厨房装修".to_string(),
            material_name: "瓷砖".to_string(),
            specification: "600x600mm".to_string(),
            quantity: 200,
            unit: "平方米".to_string(),
            estimated_cost: 18000.0,
            status: "Pending".to_string(),
        },
    ]);

    view! {
        <div class="bg-slate-900/30 backdrop-blur-md border border-slate-800 rounded-2xl p-6 shadow-xl">
            <div class="flex items-center justify-between mb-6">
                <div>
                    <h2 class="text-xl font-semibold text-slate-200">"采购计划"</h2>
                    <p class="text-sm text-slate-500">"Manage purchase orders and material requirements"</p>
                </div>
                <button class="bg-green-600 hover:bg-green-700 text-white px-4 py-2 rounded-lg text-sm font-medium transition-colors flex items-center gap-2">
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
                    "Add New"
                </button>
            </div>

            <div class="mb-4 flex items-center gap-3">
                <div class="relative flex-1">
                    <input 
                        type="text" 
                        class="w-full bg-slate-950 border border-slate-700 rounded-lg px-4 py-2 text-sm text-slate-200 focus:outline-none focus:ring-2 focus:ring-green-500/50 transition-all"
                        placeholder="Search by Product ID, Project Name..."
                    />
                </div>
                <select class="bg-slate-950 border border-slate-700 rounded-lg px-3 py-2 text-sm text-slate-200 focus:outline-none focus:ring-2 focus:ring-green-500/50 transition-all">
                    <option value="all">All Status</option>
                    <option value="pending">Pending</option>
                    <option value="approved">Approved</option>
                    <option value="completed">Completed</option>
                </select>
            </div>

            <div class="overflow-x-auto">
                <table class="w-full">
                    <thead>
                        <tr class="border-b border-slate-800">
                            <th class="text-left py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"Product ID"</th>
                            <th class="text-left py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"Project"</th>
                            <th class="text-left py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"Material"</th>
                            <th class="text-left py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"Spec"</th>
                            <th class="text-right py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"Qty"</th>
                            <th class="text-left py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"Unit"</th>
                            <th class="text-right py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"Est. Cost"</th>
                            <th class="text-left py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"Status"</th>
                            <th class="text-center py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"Actions"</th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-slate-800">
                        {move || {
                            purchase_data.with(|data| {
                                data.iter().map(|item| {
                                    view! {
                                        <tr class="hover:bg-slate-800/30 transition-colors">
                                            <td class="py-3 px-4 text-sm text-slate-300">{&item.product_id}</td>
                                            <td class="py-3 px-4 text-sm text-slate-300">{&item.project_name}</td>
                                            <td class="py-3 px-4 text-sm text-slate-300">{&item.material_name}</td>
                                            <td class="py-3 px-4 text-sm text-slate-400">{&item.specification}</td>
                                            <td class="py-3 px-4 text-sm text-slate-300 text-right">{item.quantity}</td>
                                            <td class="py-3 px-4 text-sm text-slate-400">{&item.unit}</td>
                                            <td class="py-3 px-4 text-sm text-slate-300 text-right">{format!("¥{:.2}", item.estimated_cost)}</td>
                                            <td class="py-3 px-4">
                                                <span class=format!(
                                                    "px-2 py-1 rounded-full text-xs font-medium inline-flex items-center gap-1.5 border {}",
                                                    match item.status.as_str() {
                                                        "Pending" => "bg-amber-500/10 border-amber-500/20 text-amber-400",
                                                        "Approved" => "bg-green-500/10 border-green-500/20 text-green-400",
                                                        "Completed" => "bg-blue-500/10 border-blue-500/20 text-blue-400",
                                                        _ => "bg-slate-700/30 border-slate-600/30 text-slate-400",
                                                    }
                                                )>{&item.status}</span>
                                            </td>
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
                        }}</tbody>
                </table>
            </div>
        </div>
    }
}

#[derive(Clone, Debug)]
struct PurchaseItem {
    id: usize,
    product_id: String,
    project_name: String,
    material_name: String,
    specification: String,
    quantity: i32,
    unit: String,
    estimated_cost: f64,
    status: String,
}
