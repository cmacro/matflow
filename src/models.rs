use serde::{Deserialize, Deserializer, Serialize};

/// 把 JSON 的 null（或缺失）反序列化为空字符串
fn null_to_empty_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

/// 把 JSON 的 null 反序列化为 None（用于可选字符串字段，保持 Option 语义）
fn null_to_none_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseItem {
    #[serde(rename = "序号", default)]
    pub id: Option<i32>,
    #[serde(rename = "采购日期", default, deserialize_with = "null_to_none_string")]
    pub date: Option<String>,
    #[serde(rename = "项目编号", default, deserialize_with = "null_to_none_string")]
    pub project_id: Option<String>,
    #[serde(rename = "项目名称", default, deserialize_with = "null_to_none_string")]
    pub project_name: Option<String>,
    #[serde(rename = "编号", default, deserialize_with = "null_to_empty_string")]
    pub product_id: String,
    #[serde(rename = "位置", default, deserialize_with = "null_to_none_string")]
    pub location: Option<String>,
    #[serde(rename = "内容", default, deserialize_with = "null_to_empty_string")]
    pub name: String,
    #[serde(rename = "材质工艺", default, deserialize_with = "null_to_none_string")]
    pub material: Option<String>,
    #[serde(rename = "尺 寸 ( mm )", default, deserialize_with = "null_to_none_string")]
    pub size: Option<String>,
    #[serde(rename = "数量", default)]
    pub quantity: Option<serde_json::Value>,
    #[serde(rename = "单位", default, deserialize_with = "null_to_none_string")]
    pub unit: Option<String>,
    #[serde(rename = "单价", default)]
    pub unit_price: Option<serde_json::Value>,
    #[serde(rename = "总价", default)]
    pub total_price: Option<serde_json::Value>,
    #[serde(rename = "类别", default, deserialize_with = "null_to_none_string")]
    pub category: Option<String>,
}

impl PurchaseItem {
    pub fn parse_f64(val: &Option<serde_json::Value>) -> f64 {
        match val {
            Some(serde_json::Value::Number(n)) => n.as_f64().unwrap_or(0.0),
            Some(serde_json::Value::String(s)) => s.parse::<f64>().unwrap_or(0.0),
            _ => 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductInfo {
    #[serde(rename = "产品编号", default, deserialize_with = "null_to_empty_string")]
    pub product_id: String,
    #[serde(rename = "产品名称", default, deserialize_with = "null_to_empty_string")]
    pub name: String,
    #[serde(rename = "规格型号", default, deserialize_with = "null_to_none_string")]
    pub spec: Option<String>,
    #[serde(rename = "单位", default, deserialize_with = "null_to_none_string")]
    pub unit: Option<String>,
    #[serde(rename = "类别", default, deserialize_with = "null_to_none_string")]
    pub category: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockLog {
    #[serde(rename = "日期", default, deserialize_with = "null_to_none_string")]
    pub date: Option<String>,
    #[serde(rename = "产品编号", default, deserialize_with = "null_to_empty_string")]
    pub product_id: String,
    #[serde(rename = "产品名称", default, deserialize_with = "null_to_none_string")]
    pub name: Option<String>,
    #[serde(rename = "数量", default)]
    pub quantity: serde_json::Value,
    #[serde(rename = "单位", default, deserialize_with = "null_to_none_string")]
    pub unit: Option<String>,
    #[serde(rename = "备注", default, deserialize_with = "null_to_none_string")]
    pub remark: Option<String>,
}

impl StockLog {
    pub fn get_quantity(&self) -> f64 {
        match &self.quantity {
            serde_json::Value::Number(n) => n.as_f64().unwrap_or(0.0),
            serde_json::Value::String(s) => s.parse::<f64>().unwrap_or(0.0),
            _ => 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventorySummary {
    #[serde(rename = "产品编号", default, deserialize_with = "null_to_empty_string")]
    pub product_id: String,
    #[serde(rename = "产品名称", default, deserialize_with = "null_to_empty_string")]
    pub name: String,
    #[serde(rename = "规格型号", default, deserialize_with = "null_to_none_string")]
    pub spec: Option<String>,
    #[serde(rename = "单位", default, deserialize_with = "null_to_none_string")]
    pub unit: Option<String>,
    #[serde(rename = "类别", default, deserialize_with = "null_to_none_string")]
    pub category: Option<String>,
    #[serde(rename = "累计入库", default)]
    pub total_in: serde_json::Value,
    #[serde(rename = "累计出库", default)]
    pub total_out: serde_json::Value,
    #[serde(rename = "期末库存", default)]
    pub stock: serde_json::Value,
}

impl InventorySummary {
    pub fn total_in_f64(&self) -> f64 {
        match &self.total_in {
            serde_json::Value::Number(n) => n.as_f64().unwrap_or(0.0),
            serde_json::Value::String(s) => s.parse::<f64>().unwrap_or(0.0),
            _ => 0.0,
        }
    }
    pub fn total_out_f64(&self) -> f64 {
        match &self.total_out {
            serde_json::Value::Number(n) => n.as_f64().unwrap_or(0.0),
            serde_json::Value::String(s) => s.parse::<f64>().unwrap_or(0.0),
            _ => 0.0,
        }
    }
    pub fn stock_f64(&self) -> f64 {
        match &self.stock {
            serde_json::Value::Number(n) => n.as_f64().unwrap_or(0.0),
            serde_json::Value::String(s) => s.parse::<f64>().unwrap_or(0.0),
            _ => 0.0,
        }
    }
}