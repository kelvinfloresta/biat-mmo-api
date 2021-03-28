#[derive(Debug)]
pub enum WeaponType {
  Sword
}

pub struct WeaponCTOR {
  pub id: String,
  pub name: String,
  pub weapon_type: WeaponType,
  pub power: u16,
  pub defense: u16,
}

pub struct Weapon {
  id: String,
  name: String,
  weapon_type: WeaponType,
  power: u16,
  defense: u16,
}

impl Weapon {
  pub fn new(params: WeaponCTOR) -> Weapon {
    Weapon {
      id: params.id,
      name: params.name,
      power: params.power,
      weapon_type: params.weapon_type,
      defense: params.defense
    }
  }

  pub fn get_name(&self) -> &String {
    &self.name
  }
}