use serde::Serialize;
use uuid::Uuid;

use crate::config::CONFIG;

#[derive(Debug, Serialize)]
pub struct Customer {
    pub(crate) cus_name: String,
    pub(crate) cus_email: String,
    pub(crate) cus_add1: String,
    pub(crate) cus_city: String,
    pub(crate) cus_postcode: String,
    pub(crate) cus_country: String,
    pub(crate) cus_phone: String,
}

#[derive(Debug, Serialize)]
struct CartItem {
    sku: String,
    product: String,
    quantity: u8,
    amount: f64,
}

#[derive(Debug, Serialize)]
struct ProductInformation {
    product_name: String,
    product_category: String,
    product_profile: String,
    cart: Vec<CartItem>,
}

#[derive(Debug, Serialize)]
pub struct Transaction {
    store_id: String,
    store_passwd: String,
    success_url: String,
    fail_url: String,
    cancel_url: String,
    ipn_url: Option<String>,
    emi_option: String,
    shipping_method: String,
    pub tran_id: String,
    pub(crate) total_amount: f64,
    pub(crate) product_amount: f64,
    pub(crate) currency: String,
    #[serde(flatten)]
    pub(crate) customer: Customer,
    #[serde(flatten)]
    pub(crate) product: ProductInformation,
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
            emi_option: "0".to_string(),
            shipping_method: "NO".to_string(),
            total_amount: 0.0,
            product_amount: 0.0,
            currency: "BDT".to_string(),
            tran_id: Uuid::new_v4().to_string(),
            customer: Customer {
                cus_name: String::new(),
                cus_email: String::new(),
                cus_add1: String::new(),
                cus_city: String::new(),
                cus_postcode: String::new(),
                cus_country: String::new(),
                cus_phone: String::new(),
            },
            product: ProductInformation {
                product_category: "Online Donation".to_string(),
                product_name: String::new(),
                product_profile: "general".to_string(),
                cart: vec![],
            },
        }
    }
}
