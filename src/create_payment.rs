use uuid::Uuid;

use crate::config::CONFIG;

struct Customer {
    name: String,
    email: String,
    address_line1: String,
    address_line2: String,
    city: String,
    state: String,
    postal_code: String,
    country: String,
    phone: String,
}

#[derive(Debug)]
pub struct Transaction {
    store_id: String,
    store_passwd: String,
    success_url: String,
    fail_url: String,
    cancel_url: String,
    ipn_url: Option<String>,
    pub(crate) total_amount: f64,
    pub(crate) currency: String,
    pub tran_id: String,
    pub(crate) product_category: String,
}

impl Default for Transaction {
    fn default() -> Self {
        Transaction {
            store_id: CONFIG.payment_credentials.store_id.clone(),
            store_passwd: CONFIG.payment_credentials.store_passwd.clone(),
            success_url: CONFIG.payment_credentials.success_url.clone(),
            fail_url: CONFIG.payment_credentials.fail_url.clone(),
            cancel_url: CONFIG.payment_credentials.cancel_url.clone(),
            ipn_url: CONFIG.payment_credentials.ipn_url.clone(),
            total_amount: 0.0,
            currency: "BDT".to_string(),
            tran_id: Uuid::new_v4().to_string(),
            product_category: String::new(),
        }
    }
}
