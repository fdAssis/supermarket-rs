mod products;
mod transactions;
mod users;

use products::{product::Product, store::Store};
use transactions::user_transactions;
use users::{user::Admin, user::User};

fn main() {
    let mut store: Store = Store::new();

    let mut products: Vec<Product> = Vec::new();

    let product_1: Product = Product::new(String::from("0001"), String::from("Rice"), 8.57, 10);
    let product_2: Product = Product::new(String::from("0002"), String::from("Beans"), 3.5, 100);

    // products.push(product_1);

    // products[0].set_price(10.9);

    let mut user_1 = User::new(String::from("Francisco"), 1000.00);
    let mut admin_1 = Admin::new(String::from("Francisco"));

    // user_1.add_product_in_cart(&products[0], 20);
    // user_1.add_product_in_cart(&products[0], 30);
    // user_1.user_info();
    // user_1.add_product_in_cart(&products[0], 30);
    // user_1.user_info();

    //user_1.pay();

    // user_transactions::pay(&mut user_1);
    // user_1.user_info();

    store.add_product(product_1);
    store.add_product(product_2);
    store.show_store();
}
