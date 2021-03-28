extern crate biat_mmo_api;

#[cfg(test)]
mod tests {
  use biat_mmo_api::entities::weapon::{WeaponType, Weapon};

  #[test]
  fn should_have_name() {
    let weapon = Weapon::new("1234", "Knife", 5, WeaponType::Sword, 5);
    assert_eq!(weapon.get_name(), "Knife");
  }

}