use crate::User;

pub fn _user_info(user: &User) {
  println!("\t------ User Info ------");
  println!("\tname: {}", user.get_name());
  println!("\tbalance: {}", user.get_balance());
  println!("\tuser type: {:?}", user.get_type());
  println!("\tcart: {:#?}", user.get_cart());
}

pub fn _deposit(user: &mut User, value: f32) {
  user.set_balance(user.get_balance() + value);
}

pub fn _withdraw(user: &mut User, value: f32) {
  assert!(
    !(user.get_balance() < value),
    "❌ Saldo Insuficiente! Realize um Deposito"
  );
  user.set_balance(user.get_balance() - value);
  println!("✅ Sucesso");
}
