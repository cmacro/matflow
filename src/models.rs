use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseItem {
    #[serde(rename = "序号")]
    pub id: Option<i32>,
    #[serde(rename = "采购日期")]
    pub date: Option<String>,
    #[serde(rename = "项目编号")]
    pub project_id: Option<String>,
    #[serde(rename = "项目名称")]
    pub project_name: Option<String>,
    #[serde(rename = "编号")]
    pub product_id: String,
    #[serde(rename = "位置")]
    pub location: Option<String>,
    #[serde(rename = "内容")]
    pub name: String,
    #[serde(rename = "材质工艺")]
    pub material: Option<String>,
    #[serde(rename = "尺 寸 ( mm )")]
    pub size: Option<String>,
    #[serde(rename = "数量")]
    pub quantity: Option<f64>,
    #[serde(rename = "单位")]
    pub unit: Option<String>,
    #[serde(rename = "单价")]
    pub unit_price: Option<f64>,
    #[serde(rename = "总价")]
    pub total_price: Option<f64>,
    #[serde(rename = "类别")]
    pub category: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductInfo {
    #[serde(rename = "产品编号")]
    pub product_id: String,
    #[serde(rename = "产品名称")]
    pub name: String,
    #[serde(rename = "规格型号")]
    pub spec: Option<String>,
    #[serde(rename = "单位")]
    pub unit: Option<String>,
    #[serde(rename = "类别")]
    pub category: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockLog {
    #[serde(rename = "日期")]
    pub date: Option<String>,
    #[serde(rename = "产品编号")]
    pub product_id: String,
    #[serde(rename = "产品名称")]
    pub name: Option<String>,
    #[serde(rename = "数量")]
    pub quantity: f64,
    #[serde(rename = "单位")]
    pub unit: Option<String>,
    #[serde(rename = "备注")]
    pub remark: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventorySummary {
    #[serde(rename = "产品编号")]
    pub product_id: String,
    #[serde(rename = "产品名称")]
    pub name: String,
    #[serde(rename = "规格型号")]
    pub spec: Option<String>,
    #[serde(rename = "单位")]
    pub unit: Option<String>,
    #[serde(rename = "类别")]
    pub category: Option<String>,
    #[serde(rename = "累计入库")]
    pub total_in: f64,
    #[serde(rename = "累计出库")]
    pub total_out: f64,
    #[serde(rename = "期末库存")]
    pub stock: f64,
}
