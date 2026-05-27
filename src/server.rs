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
}
