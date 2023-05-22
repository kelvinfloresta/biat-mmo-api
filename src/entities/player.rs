pub struct Player {
  name: String,
  life: u16,
  power: u16,
}

impl Player {
  pub fn new(name: &str, life: u16, power: u16) -> Player {
    Player {
      name: name.to_string(),
      power: power,
      life: life,
    }
  }

  pub fn attack(&self) -> (u16, u16) {
    return (self.power, self.power * 2)
  }

  pub fn take_damage(&mut self, damage: u16) -> &u16 {
    self.life -= damage;
    &self.life
  }

  pub fn get_name(&self) -> &String {
    &self.name
  }

  pub fn get_life(&self) -> &u16 {
    &self.life
  }
}
