use leptos::prelude::*;

#[component]
pub fn ReportPage() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <div class="flex justify-between items-center">
                <h1 class="text-2xl font-bold text-white">"报表中心"</h1>
                <div class="flex gap-2">
                    <button class="px-4 py-2 bg-slate-800 text-slate-300 rounded-lg text-sm hover:bg-slate-700 transition-colors">"导出报表"</button>
                </div>
            </div>
            
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                <div class="p-6 bg-slate-900 border border-slate-800 rounded-xl">
                    <div class="text-slate-400 text-sm mb-2">"库存周转率"</div>
                    <div class="text-2xl font-bold text-white">"12.5%"</div>
                    <div class="text-green-500 text-xs mt-2">"↑ 2.1% 比上周"</div>
                </div>
                <div class="p-6 bg-slate-900 border border-slate-800 rounded-xl">
                    <div class="text-slate-400 text-sm mb-2">"物料缺口预警"</div>
                    <div class="text-2xl font-bold text-red-500">"14 项"</div>
                    <div class="text-slate-500 text-xs mt-2">"需尽快补货"</div>
                </div>
                <div class="p-6 bg-slate-900 border border-slate-800 rounded-xl">
                    <div class="text-slate-400 text-sm mb-2">"资金占用总额"</div>
                    <div class="text-2xl font-bold text-white">"¥ 1.2M"</div>
                    <div class="text-slate-500 text-xs mt-2">"对比预算: 正常"</div>
                </div>
            </div>

            <div class="p-6 bg-slate-900 border border-slate-800 rounded-xl h-64 flex items-center justify-center text-slate-500 italic">
                "图表可视化区域 (Chart.js / ECharts Integration)"
            </div>
        </div>
    }
}
