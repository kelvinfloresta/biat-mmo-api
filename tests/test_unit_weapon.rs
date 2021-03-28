extern crate biat_mmo_api;

#[cfg(test)]
mod tests {
  use biat_mmo_api::entities::weapon::{WeaponType, Weapon, WeaponCTOR};

  #[test]
  fn should_have_name() {
    let weapon = Weapon::new(WeaponCTOR {
      id: String::from("1234"),
      name: String::from("Knife"),
      power: 5,
      defense: 5,
      weapon_type: WeaponType::Sword
    });
    assert_eq!(weapon.get_name(), "Knife");
  }

}