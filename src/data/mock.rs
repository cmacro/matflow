use crate::models::*;
use serde_json;

#[derive(Debug, Clone)]
pub struct MockStore {
    pub purchases: Vec<PurchaseItem>,
    pub products: Vec<ProductInfo>,
    pub inbound: Vec<StockLog>,
    pub outbound: Vec<StockLog>,
    pub summary: Vec<InventorySummary>,
}


impl MockStore {
    pub fn new() -> Self {
        let purchases_json = include_str!("../../raw/采购表(范例).json");
        let products_json = include_str!("../../raw/产品信息表.json");
        let inbound_json = include_str!("../../raw/入库表格.json");
        let outbound_json = include_str!("../../raw/出库表格.json");
        let summary_json = include_str!("../../raw/库存汇总.json");

        let purchases = serde_json::from_str::<Vec<PurchaseItem>>(purchases_json)
            .unwrap_or_else(|e| {
                web_sys::console::error_1(&format!("Purchase JSON Parse Error: {}", e).into());
                Vec::new()
            })
            .into_iter()
            .filter(|item| !item.name.is_empty())
            .collect();

        let products = serde_json::from_str::<Vec<ProductInfo>>(products_json)
            .unwrap_or_else(|e| {
                web_sys::console::error_1(&format!("Products JSON Parse Error: {}", e).into());
                Vec::new()
            })
            .into_iter()
            .filter(|item| !item.name.is_empty())
            .collect();

        let inbound = serde_json::from_str::<Vec<StockLog>>(inbound_json)
            .unwrap_or_else(|e| {
                web_sys::console::error_1(&format!("Inbound JSON Parse Error: {}", e).into());
                Vec::new()
            })
            .into_iter()
            .filter(|item| !item.product_id.is_empty())
            .collect();

        let outbound = serde_json::from_str::<Vec<StockLog>>(outbound_json)
            .unwrap_or_else(|e| {
                web_sys::console::error_1(&format!("Outbound JSON Parse Error: {}", e).into());
                Vec::new()
            })
            .into_iter()
            .filter(|item| !item.product_id.is_empty())
            .collect();

        let summary = serde_json::from_str::<Vec<InventorySummary>>(summary_json)
            .unwrap_or_else(|e| {
                web_sys::console::error_1(&format!("Summary JSON Parse Error: {}", e).into());
                Vec::new()
            })
            .into_iter()
            .filter(|item| !item.product_id.is_empty())
            .collect();

        Self {
            purchases,
            products,
            inbound,
            outbound,
            summary,
        }
    }
}
