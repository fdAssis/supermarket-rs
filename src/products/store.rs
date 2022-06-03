use crate::Product;

pub struct Store {
  products: Vec<Product>,
  cashier: f32,
}

impl Store {
  pub fn new() -> Self {
    Self {
      products: Vec::new(),
      cashier: 0.0,
    }
  }

  pub fn get_products(&self) -> Vec<Product> {
    self.products.clone()
  }

  pub fn show_store(&self) {
    println!("\n\t\t------ Store Products ------");
    for i in 0..self.products.len() {
      print!("\t[{}] - ", i);
      print!("Product: {} - ", self.products[i].get_name());
      print!("Value: {} - ", self.products[i].get_price());
      print!("Quantity: {}\n", self.products[i].get_quantity());
    }
    println!("\t-------------------------------------------------");
  }

  pub fn add_product(&mut self, product: Product) {
    self.products.push(product);
  }
}
