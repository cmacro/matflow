use leptos::prelude::*;
use std::sync::Arc;

/// 定义每一列的渲染配置
#[derive(Clone)]
pub struct ColumnDef<T> {
    pub header: String,
    pub align: Alignment,
    pub className: String,
    pub render: Arc<dyn Fn(&T) -> String>,
}

#[derive(Clone)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

impl Alignment {
    pub fn to_class(&self) -> &'static str {
        match self {
            Alignment::Left => "text-left",
            Alignment::Center => "text-center",
            Alignment::Right => "text-right",
        }
    }
}

#[component]
pub fn DataGrid<T>(data: Vec<T>, columns: Vec<ColumnDef<T>>) -> impl IntoView {
    let columns = Arc::new(columns);

    view! {
        <div class="overflow-x-auto rounded-xl border border-slate-800">
            <table class="w-full text-left border-collapse">
                <thead class="bg-slate-800/50">
                    <tr>
                        {columns.iter().map(|col| {
                            view! {
                                <th class=format!("py-3 px-4 text-xs font-semibold text-slate-400 uppercase tracking-wider {} ", col.align.to_class())>
                                    {col.header.clone()}
                                </th>
                            }
                        }).collect::<Vec<_>>()}
                    </tr>
                </thead>
                <tbody class="divide-y divide-slate-800 bg-slate-950/20">
                    {data.into_iter().map(|item| {
                        let item_ref = &item;
                        let cols = columns.clone();
                        view! {
                            <tr class="hover:bg-slate-800/30 transition-colors group">
                                {cols.iter().map(|col| {
                                    view! {
                                         <td class=format!("py-3 px-4 text-sm {} {}", col.align.to_class(), col.className)>
                                             <span inner_html=(col.render)(item_ref)></span>
                                         </td>
                                    }
                                }).collect::<Vec<_>>()}
                            </tr>
                        }
                    }).collect::<Vec<_>>()}
                </tbody>
            </table>
        </div>
    }
}
