mod product;
mod user;
fn main() {
    let mut products: Vec<product::Product> = Vec::new();

    let product_1: product::Product =
        product::Product::new(String::from("0001"), String::from("Rice"), 8.57, 10);

    products.push(product_1);

    products[0].set_price(10.9);

    let mut user_1 = user::User::new(user::Types::USER, String::from("Francisco"), 1000.00);

    user_1.add_product_in_cart(&products[0], 20);
    user_1.add_product_in_cart(&products[0], 30);
    user_1.user_info();
    user_1.add_product_in_cart(&products[0], 30);
    user_1.user_info();
}
