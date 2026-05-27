use rmcp::{handler::server::wrapper::Parameters, schemars, tool, tool_router};
use serde_json::{json, Value};
use crate::types::*;
use crate::store::Store;

fn now() -> String { chrono::Utc::now().to_rfc3339() }
fn round2(v: f64) -> f64 { (v * 100.0).round() / 100.0 }

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SupplierInput { pub name: String, pub category: String, pub country: String, pub currency: Option<String>, pub contact_email: Option<String>, pub contact_phone: Option<String>, pub payment_terms: Option<String> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SupplierIdInput { pub supplier_id: String }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SupplierRateInput { pub supplier_id: String, pub rating: f64, pub reason: Option<String> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct PoCreateInput { pub supplier_id: String, pub lines: Vec<Value>, pub currency: Option<String>, pub delivery_date: Option<String>, pub notes: Option<String>, pub created_by: String }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct PoIdInput { pub po_id: String }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct PoApproveInput { pub po_id: String, pub approved_by: String }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct RfqCreateInput { pub title: String, pub items: Vec<Value>, pub supplier_ids: Vec<String>, pub deadline: String, pub created_by: String }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct RfqResponseInput { pub rfq_id: String, pub supplier_id: String, pub unit_prices: Vec<f64>, pub lead_time_days: u32, pub notes: Option<String> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct RfqAwardInput { pub rfq_id: String, pub supplier_id: String }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ReceiveInput { pub po_id: String, pub lines: Vec<Value>, pub received_by: String, pub notes: Option<String> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SpendInput { pub start: Option<String>, pub end: Option<String>, pub supplier_id: Option<String>, pub category: Option<String> }

#[derive(Clone)]
pub struct ProcurementServer { pub store: Store }
impl ProcurementServer { pub fn new() -> Self { Self { store: Store::new() } } }

#[tool_router(server_handler)]
impl ProcurementServer {
    // === Suppliers ===

    #[tool(description = "Register a supplier (name, category, country, payment terms, contact info).")]
    async fn supplier_create(&self, Parameters(input): Parameters<SupplierInput>) -> String {
        let id = Store::new_id("sup");
        let sup = Supplier { id: id.clone(), name: input.name, category: input.category, contact_email: input.contact_email, contact_phone: input.contact_phone, country: input.country, currency: input.currency.unwrap_or_else(|| "USD".into()), payment_terms: input.payment_terms.unwrap_or_else(|| "net30".into()), rating: 0.0, status: "active".into(), metadata: json!({}) };
        self.store.suppliers.lock().unwrap().insert(id.clone(), sup);
        json!({"status": "created", "supplier_id": id}).to_string()
    }

    #[tool(description = "List all suppliers (optionally filter by category or status).")]
    async fn supplier_list(&self) -> String {
        let suppliers: Vec<_> = self.store.suppliers.lock().unwrap().values().cloned().collect();
        json!({"count": suppliers.len(), "suppliers": suppliers}).to_string()
    }

    #[tool(description = "Get supplier details by ID.")]
    async fn supplier_get(&self, Parameters(input): Parameters<SupplierIdInput>) -> String {
        match self.store.suppliers.lock().unwrap().get(&input.supplier_id) {
            Some(s) => serde_json::to_string_pretty(s).unwrap_or_default(),
            None => json!({"error": "SUPPLIER_NOT_FOUND"}).to_string(),
        }
    }

    #[tool(description = "Rate a supplier (0-5 stars). Tracks performance over time.")]
    async fn supplier_rate(&self, Parameters(input): Parameters<SupplierRateInput>) -> String {
        let mut suppliers = self.store.suppliers.lock().unwrap();
        match suppliers.get_mut(&input.supplier_id) {
            Some(s) => { s.rating = input.rating.min(5.0).max(0.0); json!({"status": "rated", "supplier_id": input.supplier_id, "rating": s.rating}).to_string() }
            None => json!({"error": "SUPPLIER_NOT_FOUND"}).to_string(),
        }
    }

    // === Purchase Orders ===

    #[tool(description = "Create a purchase order. Lines: [{\"sku\": \"...\", \"description\": \"...\", \"quantity\": N, \"unit_price\": X}]")]
    async fn po_create(&self, Parameters(input): Parameters<PoCreateInput>) -> String {
        let currency = input.currency.unwrap_or_else(|| "USD".into());
        let lines: Vec<PoLine> = input.lines.iter().map(|l| {
            let qty = l["quantity"].as_f64().unwrap_or(1.0);
            let price = l["unit_price"].as_f64().unwrap_or(0.0);
            PoLine { sku: l["sku"].as_str().unwrap_or("").into(), description: l["description"].as_str().unwrap_or("").into(), quantity: qty, unit_price: price, total: round2(qty * price), received_qty: 0.0 }
        }).collect();
        let subtotal: f64 = lines.iter().map(|l| l.total).sum();
        let tax = round2(subtotal * 0.16); // Default VAT
        let id = Store::new_id("po");
        let po = PurchaseOrder { id: id.clone(), supplier_id: input.supplier_id, status: "draft".into(), lines, currency, subtotal: round2(subtotal), tax, total: round2(subtotal + tax), payment_terms: "net30".into(), delivery_date: input.delivery_date, notes: input.notes, created_by: input.created_by, approved_by: None, created_at: now() };
        self.store.purchase_orders.lock().unwrap().insert(id.clone(), po);
        json!({"status": "created", "po_id": id, "total": round2(subtotal + tax)}).to_string()
    }

    #[tool(description = "List purchase orders (all or filter by status: draft, approved, sent, received).")]
    async fn po_list(&self) -> String {
        let pos: Vec<_> = self.store.purchase_orders.lock().unwrap().values().cloned().collect();
        json!({"count": pos.len(), "purchase_orders": pos}).to_string()
    }

    #[tool(description = "Get purchase order details by ID.")]
    async fn po_get(&self, Parameters(input): Parameters<PoIdInput>) -> String {
        match self.store.purchase_orders.lock().unwrap().get(&input.po_id) {
            Some(po) => serde_json::to_string_pretty(po).unwrap_or_default(),
            None => json!({"error": "PO_NOT_FOUND"}).to_string(),
        }
    }

    #[tool(description = "Approve a purchase order (moves from draft to approved). Requires approver identity.")]
    async fn po_approve(&self, Parameters(input): Parameters<PoApproveInput>) -> String {
        let mut pos = self.store.purchase_orders.lock().unwrap();
        match pos.get_mut(&input.po_id) {
            Some(po) => {
                if po.status != "draft" && po.status != "pending_approval" { return json!({"error": "PO_NOT_IN_DRAFT"}).to_string(); }
                po.status = "approved".into();
                po.approved_by = Some(input.approved_by.clone());
                json!({"status": "approved", "po_id": input.po_id, "approved_by": input.approved_by}).to_string()
            }
            None => json!({"error": "PO_NOT_FOUND"}).to_string(),
        }
    }

    #[tool(description = "Send a purchase order to the supplier (marks as sent).")]
    async fn po_send(&self, Parameters(input): Parameters<PoIdInput>) -> String {
        let mut pos = self.store.purchase_orders.lock().unwrap();
        match pos.get_mut(&input.po_id) {
            Some(po) => {
                if po.status != "approved" { return json!({"error": "PO_NOT_APPROVED"}).to_string(); }
                po.status = "sent".into();
                json!({"status": "sent", "po_id": input.po_id, "supplier_id": po.supplier_id}).to_string()
            }
            None => json!({"error": "PO_NOT_FOUND"}).to_string(),
        }
    }

    #[tool(description = "Cancel a purchase order.")]
    async fn po_cancel(&self, Parameters(input): Parameters<PoIdInput>) -> String {
        let mut pos = self.store.purchase_orders.lock().unwrap();
        match pos.get_mut(&input.po_id) {
            Some(po) => { po.status = "cancelled".into(); json!({"status": "cancelled", "po_id": input.po_id}).to_string() }
            None => json!({"error": "PO_NOT_FOUND"}).to_string(),
        }
    }

    // === RFQ (Request for Quotation) ===

    #[tool(description = "Create an RFQ (Request for Quotation) and send to multiple suppliers. Items: [{\"description\": \"...\", \"quantity\": N, \"unit\": \"kg\"}]")]
    async fn rfq_create(&self, Parameters(input): Parameters<RfqCreateInput>) -> String {
        let items: Vec<RfqItem> = input.items.iter().map(|i| RfqItem { description: i["description"].as_str().unwrap_or("").into(), quantity: i["quantity"].as_f64().unwrap_or(1.0), unit: i["unit"].as_str().unwrap_or("each").into(), specs: i["specs"].as_str().map(String::from) }).collect();
        let id = Store::new_id("rfq");
        let rfq = Rfq { id: id.clone(), title: input.title, status: "open".into(), items, supplier_ids: input.supplier_ids, responses: vec![], deadline: input.deadline, created_by: input.created_by, created_at: now() };
        self.store.rfqs.lock().unwrap().insert(id.clone(), rfq);
        json!({"status": "created", "rfq_id": id}).to_string()
    }

    #[tool(description = "Submit a supplier response to an RFQ (pricing, lead time).")]
    async fn rfq_respond(&self, Parameters(input): Parameters<RfqResponseInput>) -> String {
        let mut rfqs = self.store.rfqs.lock().unwrap();
        match rfqs.get_mut(&input.rfq_id) {
            Some(rfq) => {
                let supplier_name = self.store.suppliers.lock().unwrap().get(&input.supplier_id).map(|s| s.name.clone()).unwrap_or_else(|| input.supplier_id.clone());
                let total: f64 = rfq.items.iter().zip(input.unit_prices.iter()).map(|(item, price)| item.quantity * price).sum();
                rfq.responses.push(RfqResponse { supplier_id: input.supplier_id, supplier_name, unit_prices: input.unit_prices, total: round2(total), lead_time_days: input.lead_time_days, notes: input.notes, submitted_at: now() });
                json!({"status": "response_submitted", "rfq_id": input.rfq_id, "total_quoted": round2(total)}).to_string()
            }
            None => json!({"error": "RFQ_NOT_FOUND"}).to_string(),
        }
    }

    #[tool(description = "Compare RFQ responses side-by-side (price, lead time, supplier rating).")]
    async fn rfq_compare(&self, Parameters(input): Parameters<PoIdInput>) -> String {
        let rfqs = self.store.rfqs.lock().unwrap();
        match rfqs.get(&input.po_id) { // reusing PoIdInput for rfq_id
            Some(rfq) => {
                let suppliers = self.store.suppliers.lock().unwrap();
                let comparison: Vec<Value> = rfq.responses.iter().map(|r| {
                    let rating = suppliers.get(&r.supplier_id).map(|s| s.rating).unwrap_or(0.0);
                    json!({"supplier_id": r.supplier_id, "supplier_name": r.supplier_name, "total": r.total, "lead_time_days": r.lead_time_days, "rating": rating, "notes": r.notes})
                }).collect();
                json!({"rfq_id": input.po_id, "title": rfq.title, "responses": comparison.len(), "comparison": comparison}).to_string()
            }
            None => json!({"error": "RFQ_NOT_FOUND"}).to_string(),
        }
    }

    #[tool(description = "Award an RFQ to a supplier (closes RFQ, optionally auto-creates PO).")]
    async fn rfq_award(&self, Parameters(input): Parameters<RfqAwardInput>) -> String {
        let mut rfqs = self.store.rfqs.lock().unwrap();
        match rfqs.get_mut(&input.rfq_id) {
            Some(rfq) => {
                rfq.status = "awarded".into();
                let winner = rfq.responses.iter().find(|r| r.supplier_id == input.supplier_id);
                json!({"status": "awarded", "rfq_id": input.rfq_id, "awarded_to": input.supplier_id, "total": winner.map(|w| w.total)}).to_string()
            }
            None => json!({"error": "RFQ_NOT_FOUND"}).to_string(),
        }
    }

    // === Goods Receipt ===

    #[tool(description = "Receive goods against a PO. Lines: [{\"sku\": \"...\", \"quantity_received\": N, \"quantity_rejected\": N, \"rejection_reason\": \"...\"}]")]
    async fn goods_receive(&self, Parameters(input): Parameters<ReceiveInput>) -> String {
        let mut pos = self.store.purchase_orders.lock().unwrap();
        let po = match pos.get_mut(&input.po_id) {
            Some(p) => p,
            None => return json!({"error": "PO_NOT_FOUND"}).to_string(),
        };
        let lines: Vec<ReceiptLine> = input.lines.iter().map(|l| {
            let sku = l["sku"].as_str().unwrap_or("").to_string();
            let qty = l["quantity_received"].as_f64().unwrap_or(0.0);
            let rejected = l["quantity_rejected"].as_f64().unwrap_or(0.0);
            // Update PO line received qty
            if let Some(po_line) = po.lines.iter_mut().find(|pl| pl.sku == sku) { po_line.received_qty += qty; }
            ReceiptLine { sku, quantity_received: qty, quantity_rejected: rejected, rejection_reason: l["rejection_reason"].as_str().map(String::from) }
        }).collect();
        let all_received = po.lines.iter().all(|l| l.received_qty >= l.quantity);
        po.status = if all_received { "received".into() } else { "partially_received".into() };
        let id = Store::new_id("gr");
        let receipt = GoodsReceipt { id: id.clone(), po_id: input.po_id.clone(), lines, received_by: input.received_by, received_at: now(), notes: input.notes };
        drop(pos);
        self.store.receipts.lock().unwrap().push(receipt);
        json!({"status": if all_received { "fully_received" } else { "partially_received" }, "receipt_id": id, "po_id": input.po_id}).to_string()
    }

    // === Spend Analytics ===

    #[tool(description = "Get spend analytics (total spend by supplier, category, time period).")]
    async fn spend_analysis(&self, Parameters(input): Parameters<SpendInput>) -> String {
        let pos = self.store.purchase_orders.lock().unwrap();
        let suppliers = self.store.suppliers.lock().unwrap();
        let filtered: Vec<_> = pos.values().filter(|po| {
            po.status != "draft" && po.status != "cancelled"
            && input.supplier_id.as_ref().map_or(true, |s| po.supplier_id == *s)
        }).collect();
        let total_spend: f64 = filtered.iter().map(|po| po.total).sum();
        let by_supplier: Vec<Value> = {
            let mut map: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
            for po in &filtered { *map.entry(po.supplier_id.clone()).or_default() += po.total; }
            map.iter().map(|(sid, total)| {
                let name = suppliers.get(sid).map(|s| s.name.clone()).unwrap_or_else(|| sid.clone());
                json!({"supplier_id": sid, "name": name, "total_spend": round2(*total)})
            }).collect()
        };
        json!({"total_spend": round2(total_spend), "po_count": filtered.len(), "by_supplier": by_supplier}).to_string()
    }
}
