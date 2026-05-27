use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Supplier {
    pub id: String,
    pub name: String,
    pub category: String,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub country: String,
    pub currency: String,
    pub payment_terms: String, // net30, net60, cod
    pub rating: f64, // 0-5
    pub status: String, // active, suspended, blacklisted
    pub metadata: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PurchaseOrder {
    pub id: String,
    pub supplier_id: String,
    pub status: String, // draft, pending_approval, approved, sent, partially_received, received, cancelled
    pub lines: Vec<PoLine>,
    pub currency: String,
    pub subtotal: f64,
    pub tax: f64,
    pub total: f64,
    pub payment_terms: String,
    pub delivery_date: Option<String>,
    pub notes: Option<String>,
    pub created_by: String,
    pub approved_by: Option<String>,
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PoLine {
    pub sku: String,
    pub description: String,
    pub quantity: f64,
    pub unit_price: f64,
    pub total: f64,
    pub received_qty: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rfq {
    pub id: String,
    pub title: String,
    pub status: String, // open, closed, awarded
    pub items: Vec<RfqItem>,
    pub supplier_ids: Vec<String>,
    pub responses: Vec<RfqResponse>,
    pub deadline: String,
    pub created_by: String,
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RfqItem {
    pub description: String,
    pub quantity: f64,
    pub unit: String,
    pub specs: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RfqResponse {
    pub supplier_id: String,
    pub supplier_name: String,
    pub unit_prices: Vec<f64>,
    pub total: f64,
    pub lead_time_days: u32,
    pub notes: Option<String>,
    pub submitted_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GoodsReceipt {
    pub id: String,
    pub po_id: String,
    pub lines: Vec<ReceiptLine>,
    pub received_by: String,
    pub received_at: String,
    pub notes: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReceiptLine {
    pub sku: String,
    pub quantity_received: f64,
    pub quantity_rejected: f64,
    pub rejection_reason: Option<String>,
}
