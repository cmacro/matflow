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

        Self {
            purchases: serde_json::from_str(purchases_json).expect("Failed to parse purchases"),
            products: serde_json::from_str(products_json).expect("Failed to parse products"),
            inbound: serde_json::from_str(inbound_json).expect("Failed to parse inbound"),
            outbound: serde_json::from_str(outbound_json).expect("Failed to parse outbound"),
            summary: serde_json::from_str(summary_json).expect("Failed to parse summary"),
        }
    }

    pub fn get_product_by_id(&self, id: &str) -> Option<&ProductInfo> {
        self.products.iter().find(|p| p.product_id == id)
    }
}
