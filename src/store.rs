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
}

impl Store {
    pub fn new() -> Self {
        Self {
            suppliers: Arc::new(Mutex::new(HashMap::new())),
            purchase_orders: Arc::new(Mutex::new(HashMap::new())),
            rfqs: Arc::new(Mutex::new(HashMap::new())),
            receipts: Arc::new(Mutex::new(Vec::new())),
        }
    }
    pub fn new_id(prefix: &str) -> String { format!("{}_{}", prefix, uid()) }
}
