use crate::create_payment::Transaction;

mod config;
mod create_payment;

fn main() {
    let mut transaction = Transaction::default();
    transaction.total_amount = 100.00;

    println!("Transaction: {:?}", transaction);
}
