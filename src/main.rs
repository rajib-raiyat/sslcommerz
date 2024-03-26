use serde_json::to_string_pretty;

use crate::create_payment::{CartItem, Transaction};

mod config;
mod create_payment;

fn main() {
    let sample_data = r#"
  [
    {
      "sku": "REF00001",
      "product": "DHK TO BRS AC A1",
      "quantity": 1,
      "amount": 200.00
    },
    {
      "sku": "REF00002",
      "product": "DHK TO BRS AC A2",
      "quantity": 1,
      "amount": 200.00
    }
  ]
  "#;

    let mut transaction = Transaction::default();
    transaction.total_amount = 100.00;
    transaction.product_amount = transaction.total_amount;

    transaction.customer.cus_name = "Jeffrey A Monroe".to_string();
    transaction.customer.cus_email = "ova1976@hotmail.com".to_string();
    transaction.customer.cus_add1 = "2586 Conaway Street, Birdseye, Indiana(IN), 47513".to_string();
    transaction.customer.cus_city = "Birdseye".to_string();
    transaction.customer.cus_postcode = "47513".to_string();
    transaction.customer.cus_country = "Bangladesh".to_string();
    transaction.customer.cus_phone = "812-389-4243".to_string();

    let cart_items: Vec<CartItem> = serde_json::from_str(sample_data).unwrap();
    transaction.product.cart = cart_items;

    let json_data = to_string_pretty(&transaction).unwrap();
    // println!("Transaction: {:?}", transaction);
    println!("Transaction Request JSON:\n{}", json_data);
}
