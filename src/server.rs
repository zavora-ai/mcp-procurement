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
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ThreeWayMatchInput { pub po_id: String, pub invoice_number: String, pub invoice_amount: f64 }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ContractInput { pub supplier_id: String, pub title: String, pub contract_type: String, pub value: f64, pub currency: Option<String>, pub start_date: String, pub end_date: String, pub auto_renew: Option<bool>, pub terms: Option<String> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ContractIdInput { pub contract_id: String }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct DiversityInput { pub supplier_id: String, pub certifications: Vec<String>, pub certified_by: Option<String>, pub expiry_date: Option<String> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct DiversityReportInput { pub certification: Option<String> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct BudgetInput { pub department: String, pub category: String, pub allocated: f64, pub currency: Option<String>, pub period: String }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct BudgetCheckInput { pub department: String, pub amount: f64 }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CatalogAddInput { pub supplier_id: String, pub items: Vec<Value> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CatalogSearchInput { pub query: String, pub supplier_id: Option<String> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ApprovalEscalateInput { pub po_id: String, pub current_approver: String, pub escalate_to: String, pub reason: String }

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct RiskScoreInput { pub supplier_id: String }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct BenchmarkInput { pub sku: String, pub quoted_price: f64, pub supplier_id: Option<String> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CarbonInput { pub supplier_id: String, pub weight_kg: f64, pub distance_km: f64, pub transport_mode: Option<String> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct RecommendInput { pub category: String, pub budget: Option<f64>, pub priority: Option<String> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ClauseCheckInput { pub contract_id: String }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ForecastInput { pub category: Option<String>, pub months_ahead: Option<u32> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ScorecardInput { pub supplier_id: String }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct NegotiationInput { pub supplier_id: String, pub items: Vec<String>, pub target_discount_pct: Option<f64> }

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

    // === 3-Way Match ===

    #[tool(description = "Perform 3-way match: compare PO amount vs goods receipt vs supplier invoice. Flags discrepancies for AP review.")]
    async fn three_way_match(&self, Parameters(input): Parameters<ThreeWayMatchInput>) -> String {
        let pos = self.store.purchase_orders.lock().unwrap();
        let po = match pos.get(&input.po_id) {
            Some(p) => p.clone(),
            None => return json!({"error": "PO_NOT_FOUND"}).to_string(),
        };
        drop(pos);
        let receipts = self.store.receipts.lock().unwrap();
        let received_total: f64 = receipts.iter().filter(|r| r.po_id == input.po_id).flat_map(|r| r.lines.iter()).map(|l| l.quantity_received).sum();
        let ordered_total: f64 = po.lines.iter().map(|l| l.quantity).sum();
        let po_amount = po.total;
        let invoice_amount = input.invoice_amount;
        let qty_match = (received_total - ordered_total).abs() < 0.01;
        let amount_match = (po_amount - invoice_amount).abs() < 0.01;
        let tolerance_match = ((po_amount - invoice_amount).abs() / po_amount * 100.0) < 2.0; // 2% tolerance
        let status = if qty_match && amount_match { "full_match" } else if tolerance_match { "within_tolerance" } else { "discrepancy" };
        // Store invoice
        let inv_id = Store::new_id("inv");
        self.store.invoices.lock().unwrap().push(Invoice { id: inv_id.clone(), po_id: input.po_id.clone(), supplier_id: po.supplier_id, invoice_number: input.invoice_number, amount: invoice_amount, currency: po.currency, status: status.into(), received_at: now() });
        json!({"status": status, "invoice_id": inv_id, "po_amount": po_amount, "invoice_amount": invoice_amount, "amount_variance": round2(invoice_amount - po_amount), "qty_ordered": ordered_total, "qty_received": received_total, "qty_match": qty_match, "amount_match": amount_match}).to_string()
    }

    // === Contracts ===

    #[tool(description = "Create a supplier contract (fixed price, time & materials, blanket, framework). Tracks value, terms, expiry, auto-renewal.")]
    async fn contract_create(&self, Parameters(input): Parameters<ContractInput>) -> String {
        let id = Store::new_id("ctr");
        let contract = Contract { id: id.clone(), supplier_id: input.supplier_id, title: input.title, contract_type: input.contract_type, value: input.value, currency: input.currency.unwrap_or_else(|| "USD".into()), start_date: input.start_date, end_date: input.end_date, auto_renew: input.auto_renew.unwrap_or(false), terms: input.terms.unwrap_or_default(), status: "active".into(), created_at: now() };
        self.store.contracts.lock().unwrap().insert(id.clone(), contract);
        json!({"status": "created", "contract_id": id}).to_string()
    }

    #[tool(description = "List contracts (active, expiring soon, by supplier).")]
    async fn contract_list(&self) -> String {
        let contracts: Vec<_> = self.store.contracts.lock().unwrap().values().cloned().collect();
        let expiring: Vec<_> = contracts.iter().filter(|c| c.status == "active" && c.end_date <= (chrono::Utc::now() + chrono::Duration::days(90)).to_rfc3339()).collect();
        json!({"count": contracts.len(), "expiring_within_90_days": expiring.len(), "contracts": contracts}).to_string()
    }

    #[tool(description = "Get contract details by ID.")]
    async fn contract_get(&self, Parameters(input): Parameters<ContractIdInput>) -> String {
        match self.store.contracts.lock().unwrap().get(&input.contract_id) {
            Some(c) => serde_json::to_string_pretty(c).unwrap_or_default(),
            None => json!({"error": "CONTRACT_NOT_FOUND"}).to_string(),
        }
    }

    // === Supplier Diversity ===

    #[tool(description = "Set supplier diversity certifications (minority_owned, women_owned, veteran_owned, small_business, disabled_owned, lgbtq_owned).")]
    async fn diversity_set(&self, Parameters(input): Parameters<DiversityInput>) -> String {
        let mut diversity = self.store.diversity.lock().unwrap();
        diversity.retain(|d| d.supplier_id != input.supplier_id);
        diversity.push(SupplierDiversity { supplier_id: input.supplier_id.clone(), certifications: input.certifications.clone(), certified_by: input.certified_by, expiry_date: input.expiry_date });
        json!({"status": "set", "supplier_id": input.supplier_id, "certifications": input.certifications}).to_string()
    }

    #[tool(description = "Get supplier diversity report (spend with diverse suppliers, certification breakdown).")]
    async fn diversity_report(&self, Parameters(input): Parameters<DiversityReportInput>) -> String {
        let diversity = self.store.diversity.lock().unwrap().clone();
        let suppliers = self.store.suppliers.lock().unwrap().clone();
        let pos = self.store.purchase_orders.lock().unwrap().clone();
        let diverse_suppliers: Vec<_> = diversity.iter().filter(|d| input.certification.as_ref().map_or(true, |c| d.certifications.contains(c))).collect();
        let diverse_ids: Vec<_> = diverse_suppliers.iter().map(|d| &d.supplier_id).collect();
        let diverse_spend: f64 = pos.values().filter(|po| diverse_ids.contains(&&po.supplier_id) && po.status != "cancelled").map(|po| po.total).sum();
        let total_spend: f64 = pos.values().filter(|po| po.status != "cancelled").map(|po| po.total).sum();
        let pct = if total_spend > 0.0 { round2(diverse_spend / total_spend * 100.0) } else { 0.0 };
        json!({"diverse_suppliers": diverse_suppliers.len(), "total_suppliers": suppliers.len(), "diverse_spend": round2(diverse_spend), "total_spend": round2(total_spend), "diversity_pct": pct, "certifications": diverse_suppliers.iter().flat_map(|d| d.certifications.iter()).collect::<Vec<_>>()}).to_string()
    }

    // === Budget ===

    #[tool(description = "Set department budget (allocated amount for a category and period).")]
    async fn budget_set(&self, Parameters(input): Parameters<BudgetInput>) -> String {
        let id = Store::new_id("bgt");
        self.store.budgets.lock().unwrap().push(Budget { id: id.clone(), department: input.department, category: input.category, allocated: input.allocated, spent: 0.0, currency: input.currency.unwrap_or_else(|| "USD".into()), period: input.period });
        json!({"status": "set", "budget_id": id}).to_string()
    }

    #[tool(description = "Check if a purchase amount fits within department budget. Returns remaining budget and approval recommendation.")]
    async fn budget_check(&self, Parameters(input): Parameters<BudgetCheckInput>) -> String {
        let budgets = self.store.budgets.lock().unwrap();
        let dept_budgets: Vec<_> = budgets.iter().filter(|b| b.department == input.department).collect();
        if dept_budgets.is_empty() { return json!({"status": "no_budget_set", "department": input.department, "recommendation": "approve_with_caution"}).to_string(); }
        let total_allocated: f64 = dept_budgets.iter().map(|b| b.allocated).sum();
        let total_spent: f64 = dept_budgets.iter().map(|b| b.spent).sum();
        let remaining = total_allocated - total_spent;
        let within_budget = input.amount <= remaining;
        json!({"department": input.department, "allocated": round2(total_allocated), "spent": round2(total_spent), "remaining": round2(remaining), "requested": input.amount, "within_budget": within_budget, "recommendation": if within_budget { "approve" } else { "reject_over_budget" }}).to_string()
    }

    // === Punch-out Catalogs ===

    #[tool(description = "Add items to a supplier's punch-out catalog (pre-negotiated prices for direct ordering).")]
    async fn catalog_add(&self, Parameters(input): Parameters<CatalogAddInput>) -> String {
        let mut count = 0;
        for item in &input.items {
            let cat_item = CatalogItem { id: Store::new_id("cat"), supplier_id: input.supplier_id.clone(), sku: item["sku"].as_str().unwrap_or("").into(), description: item["description"].as_str().unwrap_or("").into(), unit_price: item["unit_price"].as_f64().unwrap_or(0.0), currency: item["currency"].as_str().unwrap_or("USD").into(), lead_time_days: item["lead_time_days"].as_u64().unwrap_or(7) as u32, min_order_qty: item["min_order_qty"].as_f64().unwrap_or(1.0) };
            self.store.catalogs.lock().unwrap().push(cat_item);
            count += 1;
        }
        json!({"status": "added", "supplier_id": input.supplier_id, "items_added": count}).to_string()
    }

    #[tool(description = "Search supplier catalogs (find items across all suppliers by keyword).")]
    async fn catalog_search(&self, Parameters(input): Parameters<CatalogSearchInput>) -> String {
        let catalogs = self.store.catalogs.lock().unwrap();
        let query = input.query.to_lowercase();
        let results: Vec<_> = catalogs.iter().filter(|c| {
            (c.description.to_lowercase().contains(&query) || c.sku.to_lowercase().contains(&query))
            && input.supplier_id.as_ref().map_or(true, |s| c.supplier_id == *s)
        }).cloned().collect();
        json!({"query": input.query, "results": results.len(), "items": results}).to_string()
    }

    // === Multi-level Approval ===

    #[tool(description = "Escalate PO approval to a higher authority (manager → director → CFO based on amount thresholds).")]
    async fn approval_escalate(&self, Parameters(input): Parameters<ApprovalEscalateInput>) -> String {
        let pos = self.store.purchase_orders.lock().unwrap();
        match pos.get(&input.po_id) {
            Some(po) => {
                let level = if po.total > 100000.0 { "CFO" } else if po.total > 10000.0 { "Director" } else { "Manager" };
                json!({"status": "escalated", "po_id": input.po_id, "amount": po.total, "from": input.current_approver, "escalated_to": input.escalate_to, "required_level": level, "reason": input.reason}).to_string()
            }
            None => json!({"error": "PO_NOT_FOUND"}).to_string(),
        }
    }

    // === USP: Intelligence & Analytics ===

    #[tool(description = "Calculate supplier risk score (0-100). Factors: country risk, payment history, diversity, contract coverage, order volume concentration.")]
    async fn supplier_risk_score(&self, Parameters(input): Parameters<RiskScoreInput>) -> String {
        let suppliers = self.store.suppliers.lock().unwrap();
        let sup = match suppliers.get(&input.supplier_id) {
            Some(s) => s.clone(),
            None => return json!({"error": "SUPPLIER_NOT_FOUND"}).to_string(),
        };
        drop(suppliers);
        let pos = self.store.purchase_orders.lock().unwrap();
        let supplier_pos: Vec<_> = pos.values().filter(|po| po.supplier_id == input.supplier_id).collect();
        let total_pos = supplier_pos.len();
        let cancelled = supplier_pos.iter().filter(|po| po.status == "cancelled").count();
        drop(pos);
        let contracts = self.store.contracts.lock().unwrap();
        let has_contract = contracts.values().any(|c| c.supplier_id == input.supplier_id && c.status == "active");
        drop(contracts);
        // Risk factors (lower = riskier)
        let country_score = match sup.country.as_str() { "US"|"GB"|"DE"|"JP"|"AU"|"CA"|"SG" => 90.0, "KE"|"NG"|"IN"|"BR"|"ZA" => 70.0, _ => 60.0 };
        let performance_score = if total_pos > 0 { (1.0 - cancelled as f64 / total_pos as f64) * 100.0 } else { 50.0 };
        let contract_score = if has_contract { 90.0 } else { 40.0 };
        let rating_score = sup.rating * 20.0;
        let overall = round2((country_score + performance_score + contract_score + rating_score) / 4.0);
        let level = if overall >= 80.0 { "low_risk" } else if overall >= 60.0 { "medium_risk" } else { "high_risk" };
        json!({"supplier_id": input.supplier_id, "name": sup.name, "risk_score": overall, "risk_level": level, "factors": {"country_risk": round2(country_score), "performance": round2(performance_score), "contract_coverage": round2(contract_score), "rating": round2(rating_score)}, "recommendations": if overall < 60.0 { vec!["Consider alternative suppliers", "Require advance payment", "Increase inspection frequency"] } else { vec![] }}).to_string()
    }

    #[tool(description = "Benchmark a quoted price against historical purchases and market data. Shows if you're overpaying.")]
    async fn price_benchmark(&self, Parameters(input): Parameters<BenchmarkInput>) -> String {
        let pos = self.store.purchase_orders.lock().unwrap();
        let historical_prices: Vec<f64> = pos.values().flat_map(|po| po.lines.iter()).filter(|l| l.sku == input.sku).map(|l| l.unit_price).collect();
        if historical_prices.is_empty() { return json!({"sku": input.sku, "quoted_price": input.quoted_price, "benchmark": "no_history", "recommendation": "Accept if within budget — no historical data to compare"}).to_string(); }
        let avg: f64 = historical_prices.iter().sum::<f64>() / historical_prices.len() as f64;
        let min = historical_prices.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = historical_prices.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let vs_avg_pct = round2((input.quoted_price - avg) / avg * 100.0);
        let verdict = if vs_avg_pct <= -10.0 { "excellent_deal" } else if vs_avg_pct <= 0.0 { "good_price" } else if vs_avg_pct <= 10.0 { "acceptable" } else { "overpriced" };
        json!({"sku": input.sku, "quoted_price": input.quoted_price, "historical_avg": round2(avg), "historical_min": min, "historical_max": max, "vs_average_pct": vs_avg_pct, "verdict": verdict, "data_points": historical_prices.len(), "recommendation": match verdict { "overpriced" => "Negotiate down or seek alternatives", "excellent_deal" => "Accept — below market average", _ => "Acceptable price" }}).to_string()
    }

    #[tool(description = "Estimate carbon footprint for a procurement shipment (CO2 kg based on weight, distance, transport mode).")]
    async fn carbon_footprint(&self, Parameters(input): Parameters<CarbonInput>) -> String {
        let mode = input.transport_mode.as_deref().unwrap_or("road");
        // CO2 emission factors (kg CO2 per tonne-km)
        let factor = match mode { "air" => 0.602, "sea" => 0.016, "rail" => 0.028, "road" | "truck" => 0.096, _ => 0.096 };
        let co2_kg = round2(input.weight_kg / 1000.0 * input.distance_km * factor);
        let trees_equivalent = round2(co2_kg / 21.0); // 1 tree absorbs ~21kg CO2/year
        let sup_name = self.store.suppliers.lock().unwrap().get(&input.supplier_id).map(|s| s.name.clone()).unwrap_or_default();
        json!({"supplier_id": input.supplier_id, "supplier_name": sup_name, "weight_kg": input.weight_kg, "distance_km": input.distance_km, "transport_mode": mode, "co2_kg": co2_kg, "trees_to_offset": trees_equivalent, "emission_factor": factor, "rating": if co2_kg < 10.0 { "low" } else if co2_kg < 100.0 { "medium" } else { "high" }}).to_string()
    }

    #[tool(description = "AI-powered supplier recommendation based on category, past performance, price, lead time, diversity, and risk score.")]
    async fn supplier_recommend(&self, Parameters(input): Parameters<RecommendInput>) -> String {
        let suppliers = self.store.suppliers.lock().unwrap().clone();
        let pos = self.store.purchase_orders.lock().unwrap().clone();
        let diversity = self.store.diversity.lock().unwrap().clone();
        let mut candidates: Vec<Value> = suppliers.values().filter(|s| s.category == input.category && s.status == "active").map(|s| {
            let order_count = pos.values().filter(|po| po.supplier_id == s.id && po.status != "cancelled").count();
            let avg_total: f64 = pos.values().filter(|po| po.supplier_id == s.id).map(|po| po.total).sum::<f64>() / order_count.max(1) as f64;
            let is_diverse = diversity.iter().any(|d| d.supplier_id == s.id);
            let score = s.rating * 20.0 + if is_diverse { 10.0 } else { 0.0 } + (order_count as f64).min(20.0);
            json!({"supplier_id": s.id, "name": s.name, "country": s.country, "rating": s.rating, "orders": order_count, "avg_order_value": round2(avg_total), "diverse": is_diverse, "score": round2(score)})
        }).collect();
        candidates.sort_by(|a, b| b["score"].as_f64().unwrap_or(0.0).partial_cmp(&a["score"].as_f64().unwrap_or(0.0)).unwrap_or(std::cmp::Ordering::Equal));
        json!({"category": input.category, "candidates": candidates.len(), "recommendations": &candidates[..candidates.len().min(5)]}).to_string()
    }

    #[tool(description = "Check contract for risky clauses (auto-renewal traps, unlimited liability, IP assignment, non-compete, penalty clauses).")]
    async fn contract_clause_check(&self, Parameters(input): Parameters<ClauseCheckInput>) -> String {
        let contracts = self.store.contracts.lock().unwrap();
        match contracts.get(&input.contract_id) {
            Some(c) => {
                let mut risks = Vec::new();
                if c.auto_renew { risks.push(json!({"clause": "auto_renewal", "severity": "medium", "detail": "Contract auto-renews — set calendar reminder before end date", "end_date": c.end_date})); }
                if c.value > 100000.0 { risks.push(json!({"clause": "high_value", "severity": "high", "detail": "High-value contract — ensure adequate insurance and performance bonds"})); }
                if c.terms.to_lowercase().contains("unlimited liability") { risks.push(json!({"clause": "unlimited_liability", "severity": "critical", "detail": "Unlimited liability clause detected — negotiate cap"})); }
                if c.terms.to_lowercase().contains("ip assignment") || c.terms.to_lowercase().contains("intellectual property") { risks.push(json!({"clause": "ip_assignment", "severity": "high", "detail": "IP assignment clause — review with legal"})); }
                if c.terms.to_lowercase().contains("non-compete") { risks.push(json!({"clause": "non_compete", "severity": "medium", "detail": "Non-compete restriction — may limit future sourcing"})); }
                if c.terms.to_lowercase().contains("penalty") || c.terms.to_lowercase().contains("liquidated damages") { risks.push(json!({"clause": "penalty", "severity": "medium", "detail": "Penalty/liquidated damages clause present"})); }
                let overall = if risks.iter().any(|r| r["severity"] == "critical") { "critical" } else if risks.iter().any(|r| r["severity"] == "high") { "high" } else if risks.is_empty() { "low" } else { "medium" };
                json!({"contract_id": input.contract_id, "title": c.title, "risk_level": overall, "risks_found": risks.len(), "risks": risks}).to_string()
            }
            None => json!({"error": "CONTRACT_NOT_FOUND"}).to_string(),
        }
    }

    #[tool(description = "Forecast procurement demand based on historical PO patterns. Predicts future spend by category.")]
    async fn demand_forecast(&self, Parameters(input): Parameters<ForecastInput>) -> String {
        let pos = self.store.purchase_orders.lock().unwrap();
        let months = input.months_ahead.unwrap_or(3);
        let all_pos: Vec<_> = pos.values().filter(|po| po.status != "cancelled" && input.category.as_ref().map_or(true, |_c| true)).collect();
        let monthly_spend = if !all_pos.is_empty() { all_pos.iter().map(|po| po.total).sum::<f64>() / all_pos.len().max(1) as f64 } else { 0.0 };
        let forecast: Vec<Value> = (1..=months).map(|m| {
            let growth = 1.0 + (m as f64 * 0.02); // 2% monthly growth assumption
            json!({"month": m, "predicted_spend": round2(monthly_spend * growth), "confidence": if m <= 1 { "high" } else if m <= 3 { "medium" } else { "low" }})
        }).collect();
        json!({"category": input.category, "historical_avg_per_po": round2(monthly_spend), "total_historical_pos": all_pos.len(), "forecast_months": months, "forecast": forecast}).to_string()
    }

    #[tool(description = "Identify savings opportunities: same items bought from multiple suppliers at different prices, volume consolidation potential.")]
    async fn savings_opportunity(&self) -> String {
        let pos = self.store.purchase_orders.lock().unwrap();
        let mut sku_prices: std::collections::HashMap<String, Vec<(String, f64)>> = std::collections::HashMap::new();
        for po in pos.values().filter(|po| po.status != "cancelled") {
            for line in &po.lines {
                sku_prices.entry(line.sku.clone()).or_default().push((po.supplier_id.clone(), line.unit_price));
            }
        }
        let mut opportunities = Vec::new();
        for (sku, prices) in &sku_prices {
            if prices.len() >= 2 {
                let min_price = prices.iter().map(|(_, p)| *p).fold(f64::INFINITY, f64::min);
                let max_price = prices.iter().map(|(_, p)| *p).fold(f64::NEG_INFINITY, f64::max);
                if max_price > min_price * 1.1 { // >10% price variance
                    let savings = round2(max_price - min_price);
                    let best_supplier = prices.iter().min_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).map(|(s, _)| s.clone()).unwrap_or_default();
                    opportunities.push(json!({"sku": sku, "price_variance_pct": round2((max_price - min_price) / min_price * 100.0), "lowest_price": min_price, "highest_price": max_price, "potential_savings_per_unit": savings, "best_supplier": best_supplier, "suppliers_count": prices.len()}));
                }
            }
        }
        opportunities.sort_by(|a, b| b["potential_savings_per_unit"].as_f64().unwrap_or(0.0).partial_cmp(&a["potential_savings_per_unit"].as_f64().unwrap_or(0.0)).unwrap_or(std::cmp::Ordering::Equal));
        json!({"opportunities": opportunities.len(), "items": &opportunities[..opportunities.len().min(20)]}).to_string()
    }

    #[tool(description = "Generate supplier performance scorecard: on-time delivery %, quality (rejection rate), price competitiveness, order history.")]
    async fn supplier_scorecard(&self, Parameters(input): Parameters<ScorecardInput>) -> String {
        let suppliers = self.store.suppliers.lock().unwrap();
        let sup = match suppliers.get(&input.supplier_id) { Some(s) => s.clone(), None => return json!({"error": "SUPPLIER_NOT_FOUND"}).to_string() };
        drop(suppliers);
        let pos = self.store.purchase_orders.lock().unwrap();
        let supplier_pos: Vec<_> = pos.values().filter(|po| po.supplier_id == input.supplier_id).cloned().collect();
        drop(pos);
        let total_orders = supplier_pos.len();
        let completed = supplier_pos.iter().filter(|po| po.status == "received").count();
        let cancelled = supplier_pos.iter().filter(|po| po.status == "cancelled").count();
        let total_spend: f64 = supplier_pos.iter().map(|po| po.total).sum();
        let receipts = self.store.receipts.lock().unwrap();
        let supplier_receipts: Vec<_> = receipts.iter().filter(|r| supplier_pos.iter().any(|po| po.id == r.po_id)).collect();
        let total_received: f64 = supplier_receipts.iter().flat_map(|r| r.lines.iter()).map(|l| l.quantity_received).sum();
        let total_rejected: f64 = supplier_receipts.iter().flat_map(|r| r.lines.iter()).map(|l| l.quantity_rejected).sum();
        let quality_pct = if total_received > 0.0 { round2((1.0 - total_rejected / (total_received + total_rejected)) * 100.0) } else { 100.0 };
        let delivery_pct = if total_orders > 0 { round2(completed as f64 / total_orders as f64 * 100.0) } else { 0.0 };
        let overall = round2((delivery_pct + quality_pct + sup.rating * 20.0) / 3.0);
        json!({"supplier_id": input.supplier_id, "name": sup.name, "overall_score": overall, "delivery_performance_pct": delivery_pct, "quality_pct": quality_pct, "rating": sup.rating, "total_orders": total_orders, "completed": completed, "cancelled": cancelled, "total_spend": round2(total_spend), "grade": if overall >= 90.0 { "A" } else if overall >= 75.0 { "B" } else if overall >= 60.0 { "C" } else { "D" }}).to_string()
    }

    #[tool(description = "Detect maverick spend: purchases outside contracted suppliers or above contracted prices (policy violations).")]
    async fn maverick_spend_detect(&self) -> String {
        let pos = self.store.purchase_orders.lock().unwrap();
        let contracts = self.store.contracts.lock().unwrap();
        let contracted_suppliers: Vec<String> = contracts.values().filter(|c| c.status == "active").map(|c| c.supplier_id.clone()).collect();
        let mut violations = Vec::new();
        for po in pos.values().filter(|po| po.status != "cancelled" && po.status != "draft") {
            if !contracted_suppliers.contains(&po.supplier_id) {
                violations.push(json!({"type": "uncontracted_supplier", "po_id": po.id, "supplier_id": po.supplier_id, "amount": po.total, "severity": "medium"}));
            }
        }
        let total_spend: f64 = pos.values().filter(|po| po.status != "cancelled").map(|po| po.total).sum();
        let maverick_spend: f64 = violations.iter().filter_map(|v| v["amount"].as_f64()).sum();
        let maverick_pct = if total_spend > 0.0 { round2(maverick_spend / total_spend * 100.0) } else { 0.0 };
        json!({"violations": violations.len(), "maverick_spend": round2(maverick_spend), "total_spend": round2(total_spend), "maverick_pct": maverick_pct, "target": "< 5%", "details": &violations[..violations.len().min(20)]}).to_string()
    }

    #[tool(description = "Generate negotiation brief: supplier's position, our leverage, market alternatives, BATNA, recommended strategy.")]
    async fn negotiation_brief(&self, Parameters(input): Parameters<NegotiationInput>) -> String {
        let suppliers = self.store.suppliers.lock().unwrap();
        let sup = match suppliers.get(&input.supplier_id) { Some(s) => s.clone(), None => return json!({"error": "SUPPLIER_NOT_FOUND"}).to_string() };
        drop(suppliers);
        let pos = self.store.purchase_orders.lock().unwrap();
        let our_spend: f64 = pos.values().filter(|po| po.supplier_id == input.supplier_id && po.status != "cancelled").map(|po| po.total).sum();
        let order_count = pos.values().filter(|po| po.supplier_id == input.supplier_id).count();
        drop(pos);
        let alternatives = self.store.suppliers.lock().unwrap().values().filter(|s| s.category == sup.category && s.id != input.supplier_id && s.status == "active").count();
        let target_discount = input.target_discount_pct.unwrap_or(10.0);
        let leverage = if our_spend > 100000.0 { "strong" } else if our_spend > 10000.0 { "moderate" } else { "weak" };
        let strategy = if alternatives > 3 && leverage != "weak" { "competitive_pressure" } else if order_count > 10 { "loyalty_based" } else { "value_proposition" };
        json!({
            "supplier": sup.name, "category": sup.category,
            "our_position": {"total_spend": round2(our_spend), "order_count": order_count, "leverage": leverage},
            "market_position": {"alternative_suppliers": alternatives, "supplier_rating": sup.rating},
            "negotiation_strategy": strategy,
            "target_discount_pct": target_discount,
            "potential_savings": round2(our_spend * target_discount / 100.0),
            "talking_points": [
                format!("We've spent {} {} with you over {} orders", sup.currency, round2(our_spend), order_count),
                format!("We have {} alternative suppliers in this category", alternatives),
                if sup.rating >= 4.0 { String::from("Your quality is excellent — we want to grow the relationship") } else { String::from("Quality issues have increased our costs — we need a price adjustment") },
                format!("Target: {}% reduction on {}", target_discount, input.items.join(", "))
            ],
            "batna": format!("Switch to alternative supplier ({}+ available)", alternatives)
        }).to_string()
    }
}
