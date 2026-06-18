use leptos::prelude::*;

#[component]
pub fn Topbar() -> impl IntoView {
    view! {
        <header class="h-16 border-b border-slate-800 flex items-center justify-between px-8 bg-slate-950/20 backdrop-blur-sm">
            <div class="text-slate-400 text-sm font-medium">"MATFLOW Management System"</div>
            <div class="flex items-center gap-4">
                <div class="relative">
                    <input 
                        class="bg-slate-900 border border-slate-700 rounded-full px-4 py-1 text-sm text-slate-200 focus:outline-none focus:ring-2 focus:ring-green-500/50 transition-all w-64" 
                        placeholder="Search materials..."
                    />
                </div>
                <div class="w-8 h-8 rounded-full bg-gradient-to-tr from-green-500 to-emerald-600 border border-slate-700 cursor-pointer"></div>
            </div>
        </header>
    }
}
