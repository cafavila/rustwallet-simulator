mod domain;
use crate::domain::wallet::Wallet;

fn main() {
    println!("Welcome to the wallet simulator!");
    let wallet = Wallet::new(1, 1);
    println!("Wallet created with ID: {}", wallet.id());
    println!("Wallet created with Owner ID: {}", wallet.owner_id());
}
