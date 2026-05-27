use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::types::*;

fn uid() -> String { uuid::Uuid::new_v4().to_string()[..8].to_string() }

#[derive(Clone)]
pub struct Store {
    pub suppliers: Arc<Mutex<HashMap<String, Supplier>>>,
    pub purchase_orders: Arc<Mutex<HashMap<String, PurchaseOrder>>>,
    pub rfqs: Arc<Mutex<HashMap<String, Rfq>>>,
    pub receipts: Arc<Mutex<Vec<GoodsReceipt>>>,
    pub contracts: Arc<Mutex<HashMap<String, Contract>>>,
    pub invoices: Arc<Mutex<Vec<Invoice>>>,
    pub budgets: Arc<Mutex<Vec<Budget>>>,
    pub diversity: Arc<Mutex<Vec<SupplierDiversity>>>,
    pub catalogs: Arc<Mutex<Vec<CatalogItem>>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            suppliers: Arc::new(Mutex::new(HashMap::new())),
            purchase_orders: Arc::new(Mutex::new(HashMap::new())),
            rfqs: Arc::new(Mutex::new(HashMap::new())),
            receipts: Arc::new(Mutex::new(Vec::new())),
            contracts: Arc::new(Mutex::new(HashMap::new())),
            invoices: Arc::new(Mutex::new(Vec::new())),
            budgets: Arc::new(Mutex::new(Vec::new())),
            diversity: Arc::new(Mutex::new(Vec::new())),
            catalogs: Arc::new(Mutex::new(Vec::new())),
        }
    }
    pub fn new_id(prefix: &str) -> String { format!("{}_{}", prefix, uid()) }
}
