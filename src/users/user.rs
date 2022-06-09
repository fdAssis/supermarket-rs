use super::types::Types;
use crate::actions::transfer_type::Transfer::{DEPOSIT, WITHDRAW};
use crate::actions::{transfer_type::Transfer, user_actions};
use crate::Product;

#[derive(Debug, Clone)]
pub struct User {
  user_type: Types,
  name: String,
  balance: f32,
  cart: Vec<Product>,
}

impl User {
  pub fn new(name: String, balance: f32) -> Self {
    User {
      user_type: Types::USER,
      name,
      balance,
      cart: Vec::new(),
    }
  }

  pub fn get_name(&self) -> String {
    self.name.clone()
  }
  pub fn get_type(&self) -> Types {
    self.user_type
  }
  pub fn get_cart(&self) -> Vec<Product> {
    self.cart.clone()
  }
  pub fn get_balance(&self) -> f32 {
    self.balance
  }

  pub fn set_balance(&mut self, value: f32) {
    self.balance = value;
  }

  pub fn update_balace(&mut self, value: f32, transfer_type: Transfer) {
    match transfer_type {
      DEPOSIT => user_actions::_deposit(self, value),
      WITHDRAW => user_actions::_withdraw(self, value),
    }
  }

  pub fn user_info(&self) {
    user_actions::_user_info(self);
  }
}
