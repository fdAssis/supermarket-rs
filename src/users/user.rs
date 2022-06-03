use super::types;
use crate::product::Product;

#[derive(Debug)]
pub struct User {
  user_type: types::Types,
  name: String,
  balance: f32,
  cart: Vec<Product>,
}

impl User {
  pub fn new(user_type: types::Types, name: String, balance: f32) -> User {
    User {
      user_type,
      name,
      balance,
      cart: Vec::new(),
    }
  }

  pub fn get_balance(&self) -> f32 {
    self.balance
  }

  pub fn set_balance(&mut self, value: f32) {
    self.balance = value;
  }

  pub fn get_cart(&self) -> Vec<Product> {
    self.cart.clone()
  }

  pub fn user_info(&self) {
    println!("\t------ User Info ------");
    println!("\tname: {}", self.name);
    println!("\tbalance: {}", self.balance);
    println!("\tuser type: {:?}", self.user_type);
    println!("\tcart: {:#?}", self.cart);
  }

  pub fn add_product_in_cart(&mut self, product: &Product, quantity: u32) {
    for i in 0..self.cart.len() {
      if self.cart[i].get_id().eq(&product.get_id()) {
        let old_quantity: u32 = self.cart[i].get_quantity();
        self.cart[i].set_quantity(old_quantity + quantity);
        return;
      }
    }

    let product = Product::new(
      product.get_id(),
      product.get_name(),
      product.get_price(),
      quantity,
    );

    self.cart.push(product);
  }
}
