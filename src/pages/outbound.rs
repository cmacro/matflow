use leptos::prelude::*;

#[component]
pub fn OutboundPage() -> impl IntoView {
    // Mock data for outbound records
    let (outbound_data, _set_outbound_data) = signal(vec![
        OutboundItem {
            id: 1,
            product_id: "MAT-001".to_string(),
            material_name: "乳胶漆".to_string(),
            specification: "10kg/桶".to_string(),
            outbound_date: "2024-01-18".to_string(),
            quantity: 5,
            unit: "桶".to_string(),
            operator: "张三".to_string(),
            purpose: "客厅装修".to_string(),
        },
        OutboundItem {
            id: 2,
            product_id: "MAT-002".to_string(),
            material_name: "地板".to_string(),
            specification: "1200x120mm".to_string(),
            outbound_date: "2024-01-20".to_string(),
            quantity: 30,
            unit: "平方米".to_string(),
            operator: "李四".to_string(),
            purpose: "卧室装修".to_string(),
        },
        OutboundItem {
            id: 3,
            product_id: "MAT-003".to_string(),
            material_name: "瓷砖".to_string(),
            specification: "600x600mm".to_string(),
            outbound_date: "2024-01-22".to_string(),
            quantity: 25,
            unit: "平方米".to_string(),
            operator: "王五".to_string(),
            purpose: "厨房装修".to_string(),
        },
    ]);

    view! {
        <div class="bg-slate-900/30 backdrop-blur-md border border-slate-800 rounded-2xl p-6 shadow-xl">
            <div class="flex items-center justify-between mb-6">
                <div>
                    <h2 class="text-xl font-semibold text-slate-200">"出库流水"</h2>
                    <p class="text-sm text-slate-500">"Track material outbound records and warehouse dispatch"</p>
                </div>
                <button class="bg-green-600 hover:bg-green-700 text-white px-4 py-2 rounded-lg text-sm font-medium transition-colors flex items-center gap-2">
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
                    "Add Outbound"
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
                <div class="relative">
                    <input 
                        type="date" 
                        class="bg-slate-950 border border-slate-700 rounded-lg px-3 py-2 text-sm text-slate-200 focus:outline-none focus:ring-2 focus:ring-green-500/50 transition-all"
                    />
                </div>
                <select class="bg-slate-950 border border-slate-700 rounded-lg px-3 py-2 text-sm text-slate-200 focus:outline-none focus:ring-2 focus:ring-green-500/50 transition-all">
                    <option value="all">All Operators</option>
                    <option value="zhangsan">张三</option>
                    <option value="lisi">李四</option>
                    <option value="wangwu">王五</option>
                </select>
            </div>

            <div class="overflow-x-auto">
                <table class="w-full">
                    <thead>
                        <tr class="border-b border-slate-800">
                            <th class="text-left py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"Date"</th>
                            <th class="text-left py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"Product ID"</th>
                            <th class="text-left py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"Material Name"</th>
                            <th class="text-left py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"Specification"</th>
                            <th class="text-right py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"Outbound Qty"</th>
                            <th class="text-left py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"Unit"</th>
                            <th class="text-left py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"Operator"</th>
                            <th class="text-left py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"Purpose"</th>
                            <th class="text-center py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider">"Actions"</th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-slate-800">
                        {move || {
                            outbound_data.with(|data| {
                                data.iter().map(|item| {
                                    view! {
                                        <tr class="hover:bg-slate-800/30 transition-colors" id={item.id}>
                                            <td class="py-3 px-4 text-sm text-slate-300">{item.outbound_date.clone()}</td>
                                            <td class="py-3 px-4 text-sm text-green-400 font-mono">{item.product_id.clone()}</td>
                                            <td class="py-3 px-4 text-sm text-slate-300">{item.material_name.clone()}</td>
                                            <td class="py-3 px-4 text-sm text-slate-400">{item.specification.clone()}</td>
                                            <td class="py-3 px-4 text-sm text-red-400 text-right font-mono">{item.quantity}</td>
                                            <td class="py-3 px-4 text-sm text-slate-400">{item.unit.clone()}</td>
                                            <td class="py-3 px-4 text-sm text-slate-300">{item.operator.clone()}</td>
                                            <td class="py-3 px-4 text-sm text-slate-300">{item.purpose.clone()}</td>
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
struct OutboundItem {
    id: usize,
    product_id: String,
    material_name: String,
    specification: String,
    outbound_date: String,
    quantity: i32,
    unit: String,
    operator: String,
    purpose: String,
}
