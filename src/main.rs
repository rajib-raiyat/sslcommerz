use serde_json::to_string_pretty;

use crate::create_payment::Transaction;

mod config;
mod create_payment;

fn main() {
    let mut transaction = Transaction::default();
    transaction.total_amount = 100.00;
    transaction.customer.cus_name = "Adam".to_string();

    // println!("Transaction: {:?}", transaction);
    let json_data = to_string_pretty(&transaction).unwrap();
    println!("Transaction Request JSON:\n{}", json_data);
}
