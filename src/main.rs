mod actions;
mod products;
mod users;

use actions::transfer_type::Transfer;
use products::{product::Product, store::Store};
use users::user::User;

fn main() {
    let mut store: Store = Store::new();

    let product_1: Product = Product::new(String::from("0001"), String::from("Rice"), 8.57, 10);
    let product_2: Product = Product::new(String::from("0002"), String::from("Beans"), 3.5, 100);
    let mut user_1 = User::new(String::from("Francisco"), 1000.00);

    store.add_product(product_1);
    store.add_product(product_2);
    user_1.user_info();
    user_1.update_balace(20000.0, Transfer::WITHDRAW);
    user_1.user_info();
    store.show_store();
}
