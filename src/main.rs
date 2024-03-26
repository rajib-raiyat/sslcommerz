mod config;

use config::CONFIG;

fn main() {
    let base_url = &CONFIG.payment_credentials.base_url;
    let store_id = &CONFIG.payment_credentials.store_id;

    println!("Base URL: {}", base_url);
    println!("Store ID: {}", store_id);
}
