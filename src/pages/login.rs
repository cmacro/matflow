use leptos::ev::KeyboardEvent;
use leptos::prelude::*;

#[component]
pub fn LoginPage(on_login: impl Fn() + Clone + 'static) -> impl IntoView {
    let (username, set_username) = signal("admin".to_string());
    let (password, set_password) = signal(String::new());

    let login_action = {
        let on_login = on_login.clone();
        move || {
            if !username.get().trim().is_empty() {
                on_login();
            }
        }
    };
    let login_action = login_action.clone();
    let login_action2 = login_action.clone();

    view! {
        <div class="relative min-h-screen overflow-hidden bg-slate-950 text-slate-50">

            // 背景光效
            <div class="absolute inset-0 overflow-hidden">

                <div class="
                    absolute
                    -top-40
                    -left-40
                    w-[600px]
                    h-[600px]
                    rounded-full
                    bg-blue-500/10
                    blur-3xl
                "></div>

                <div class="
                    absolute
                    -bottom-40
                    -right-40
                    w-[500px]
                    h-[500px]
                    rounded-full
                    bg-cyan-500/10
                    blur-3xl
                "></div>

                <div class="
                    absolute
                    top-1/2
                    left-1/2
                    -translate-x-1/2
                    -translate-y-1/2
                    w-[800px]
                    h-[800px]
                    rounded-full
                    bg-blue-600/5
                    blur-3xl
                "></div>

            </div>

            <div class="relative z-10 flex min-h-screen items-center justify-center p-6">

                <div class="w-full max-w-7xl">

                    <div class="grid lg:grid-cols-2 gap-16 items-center">
                        // =========================
                        // 左侧品牌区
                        // =========================
                        <div class="hidden lg:block">
                            <div class="
                                inline-flex
                                items-center
                                justify-center
                                w-16
                                h-16
                                rounded-2xl
                                bg-blue-600/20
                                border
                                border-blue-500/30
                                mb-8
                            ">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    width="32"
                                    height="32"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="2"
                                >
                                    <path d="M3 7L12 2L21 7L12 12L3 7Z"/>
                                    <path d="M3 17L12 22L21 17"/>
                                    <path d="M3 12L12 17L21 12"/>
                                </svg>
                            </div>

                            <h1 class="text-6xl font-bold tracking-tight">
                                "MatFlow"
                            </h1>

                            <p class="mt-4 text-2xl text-slate-400">
                                "智能物料与库存管理平台"
                            </p>

                            <p class="mt-6 max-w-xl text-slate-500 leading-8">
                                "覆盖采购、入库、出库、库存管理、物料档案与统计分析，
                                帮助企业实现全流程数字化仓储管理。"
                            </p>

                        </div>

                        // =========================
                        // 登录框
                        // =========================

                        <div class="flex justify-center">

                            <div class="
                                w-full
                                max-w-md
                                rounded-3xl
                                border
                                border-slate-800
                                bg-slate-900/70
                                backdrop-blur-2xl
                                shadow-2xl
                                p-8
                            ">

                                <div class="text-center mb-8">
                                    <h2 class="text-3xl font-bold"> "欢迎回来" </h2>
                                    <p class="mt-2 text-slate-500"> "登录 MatFlow 管理平台" </p>
                                </div>

                                <div class="space-y-5">
                                    // 用户名
                                    <div>
                                        <label class="block text-sm text-slate-400 mb-2"> "用户名" </label>
                                        <input
                                            type="text"
                                            class="
                                                w-full
                                                rounded-xl
                                                border
                                                border-slate-700
                                                bg-slate-950
                                                px-4
                                                py-3
                                                text-sm
                                                outline-none
                                                transition
                                                focus:border-blue-500
                                                focus:ring-2
                                                focus:ring-blue-500/30
                                            "
                                            placeholder="请输入用户名"
                                            prop:value=move || username.get()
                                            on:input=move |ev| {
                                                set_username.set(event_target_value(&ev));
                                            }
                                        />

                                    </div>

                                    // 密码
                                    <div>
                                        <label class="block text-sm text-slate-400 mb-2"> "密码" </label>
                                        <input
                                            type="password"
                                            class="
                                                w-full
                                                rounded-xl
                                                border
                                                border-slate-700
                                                bg-slate-950
                                                px-4
                                                py-3
                                                text-sm
                                                outline-none
                                                transition
                                                focus:border-blue-500
                                                focus:ring-2
                                                focus:ring-blue-500/30
                                            "
                                            placeholder="请输入密码"
                                            prop:value=move || password.get()
                                            on:input=move |ev| {
                                                set_password.set(event_target_value(&ev));
                                            }
                                            on:keydown=move |ev: KeyboardEvent| {
                                                if ev.key() == "Enter" {
                                                    login_action();
                                                }
                                            }
                                        />

                                    </div>

                                    <button
                                        class="
                                            w-full
                                            rounded-xl
                                            bg-gradient-to-r
                                            from-blue-600
                                            to-cyan-500
                                            py-3
                                            font-semibold
                                            text-white
                                            transition-all
                                            hover:scale-[1.01]
                                            active:scale-[0.99]
                                            shadow-lg
                                            shadow-blue-900/30
                                        "
                                        on:click=move |_| { login_action2(); }
                                    >
                                        "登录系统"
                                    </button>

                                </div>

                                <div class="mt-6 text-center">
                                    <a
                                        href="#"
                                        class=" text-sm text-slate-500 hover:text-slate-300 transition-colors "
                                    >
                                        "忘记密码？"
                                    </a>
                                </div>

                                <div class=" mt-8 border-t border-slate-800 pt-6 text-center text-xs text-slate-600 ">
                                    "MatFlow v0.1.0"
                                </div>

                            </div>
                        </div>
                    </div>
                </div>
            </div>

        </div>
    }
}
