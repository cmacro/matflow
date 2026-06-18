use leptos::prelude::*;

#[component]
pub fn SettingsPage() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <h1 class="text-2xl font-bold text-white">"基础设置"</h1>
            
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                <div class="space-y-4">
                    <h2 class="text-lg font-medium text-slate-300">"单位管理"</h2>
                    <div class="p-4 bg-slate-900 border border-slate-800 rounded-xl space-y-3">
                        <div class="flex justify-between items-center py-2 border-b border-slate-800">
                            <span class="text-slate-400">"个"</span>
                            <span class="text-xs text-slate-500">"基础单位"</span>
                        </div>
                        <div class="flex justify-between items-center py-2 border-b border-slate-800">
                            <span class="text-slate-400">"平方米"</span>
                            <span class="text-xs text-slate-500">"面积单位"</span>
                        </div>
                        <button class="w-full py-2 text-xs text-green-500 hover:text-green-400 text-center">"+ 添加新单位"</button>
                    </div>
                </div>

                <div class="space-y-4">
                    <h2 class="text-lg font-medium text-slate-300">"物料分类管理"</h2>
                    <div class="p-4 bg-slate-900 border border-slate-800 rounded-xl space-y-3">
                        <div class="flex justify-between items-center py-2 border-b border-slate-800">
                            <span class="text-slate-400">"瓷砖"</span>
                            <span class="text-xs text-slate-500">"主材"</span>
                        </div>
                        <div class="flex justify-between items-center py-2 border-b border-slate-800">
                            <span class="text-slate-400">"五金件"</span>
                            <span class="text-xs text-slate-500">"辅材"</span>
                        </div>
                        <button class="w-full py-2 text-xs text-green-500 hover:text-green-400 text-center">"+ 添加分类"</button>
                    </div>
                </div>
            </div>
        </div>
    }
}
