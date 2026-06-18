use leptos::prelude::*;

#[component]
pub fn SystemPage() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <h1 class="text-2xl font-bold text-white">"系统管理"</h1>
            
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div class="p-6 bg-slate-900 border border-slate-800 rounded-xl space-y-4">
                    <h2 class="text-lg font-medium text-slate-300">"用户账户管理"</h2>
                    <div class="space-y-2">
                        <div class="flex justify-between p-2 bg-slate-800 rounded text-sm">
                            <span class="text-white">"管理员 (admin)"</span>
                            <span class="text-slate-400">"超级权限"</span>
                        </div>
                        <div class="flex justify-between p-2 bg-slate-800 rounded text-sm">
                            <span class="text-white">"仓库管理员 (store_mgr)"</span>
                            <span class="text-slate-400">"出入库权限"</span>
                        </div>
                    </div>
                    <button class="px-4 py-2 bg-green-600 text-white rounded-lg text-xs hover:bg-green-500 transition-colors">"添加用户"</button>
                </div>

                <div class="p-6 bg-slate-900 border border-slate-800 rounded-xl space-y-4">
                    <h2 class="text-lg font-medium text-slate-300">"系统日志"</h2>
                    <div class="text-xs text-slate-500 space-y-2 font-mono">
                        <div class="flex gap-2"><span>"[2026-06-18 10:00]"</span><span class="text-slate-300">"用户 admin 修改了物料 A001 的库存"</span></div>
                        <div class="flex gap-2"><span>"[2026-06-18 09:30]"</span><span class="text-slate-300">"系统完成每日备份"</span></div>
                        <div class="flex gap-2"><span>"[2026-06-18 08:15]"</span><span class="text-slate-300">"用户 store_mgr 登录成功"</span></div>
                    </div>
                    <button class="px-4 py-2 bg-slate-800 text-slate-300 rounded-lg text-xs hover:bg-slate-700 transition-colors">"查看全部日志"</button>
                </div>
            </div>
        </div>
    }
}
