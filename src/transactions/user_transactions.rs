use crate::User;

pub fn pay(user: &mut User) {
  let mut total: f32 = 0.0;

  for i in 0..user.get_cart().len() {
    total += user.get_cart()[i].get_price() * (user.get_cart()[i].get_quantity() as f32);
  }

  assert!(!(user.get_balance() < total), "❌ Saldo Insuficiente");

  user.set_balance(user.get_balance() - total);

  println!("✅ Compra realizada com sucesso");
}
