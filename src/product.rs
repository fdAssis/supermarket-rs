#[derive(Debug)]
pub struct Product {
  id: String,
  name: String,
  price: f32,
  quantity: u32,
}

impl Product {
  pub fn new(id: String, name: String, price: f32, quantity: u32) -> Product {
    Product {
      id,
      name,
      price,
      quantity,
    }
  }

  pub fn product_info(&self) {
    println!("\t------ Product Info -----");
    println!("\tid: {} ", self.id);
    println!("\tname: {} ", self.name);
    println!("\tprice: {} ", self.price);
    println!("\tquantity: {} ", self.quantity);
  }

  pub fn get_name(&self) -> String {
    self.name.clone()
  }

  pub fn get_id(&self) -> String {
    self.id.clone()
  }

  pub fn get_price(&self) -> f32 {
    self.price
  }

  pub fn get_quantity(&self) -> u32 {
    self.quantity
  }

  pub fn set_price(&mut self, new_price: f32) {
    self.price = new_price;
  }

  pub fn set_quantity(&mut self, new_quantity: u32) {
    self.quantity = new_quantity;
  }
}
