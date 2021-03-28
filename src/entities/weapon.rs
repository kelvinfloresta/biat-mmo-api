#[derive(Debug)]
pub enum WeaponType {
  Sword
}

pub struct Weapon {
  id: String,
  name: String,
  weapon_type: WeaponType,
  power: u16,
  defense: u16,
}

impl Weapon {
  pub fn new(id: &str, name: &str, power: u16, weapon_type: WeaponType, defense: u16) -> Weapon {
    Weapon {
      id: id.to_string(),
      name: name.to_string(),
      power: power,
      weapon_type: weapon_type,
      defense: defense
    }
  }

  pub fn get_name(&self) -> &String {
    &self.name
  }
}