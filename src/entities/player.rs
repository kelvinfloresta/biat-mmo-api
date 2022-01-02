use rand::{thread_rng, Rng};

pub struct Player {
  name: String,
  life: u32,
  power: u16,
}

impl Player {
  pub fn new(name: &str, life: u32, power: u16) -> Player {
    Player {
      name: name.to_string(),
      power: power,
      life: life,
    }
  }

  pub fn attack(&self) -> u16 {
    let random_value: f64 = thread_rng().gen_range(0.0..1.0);
    let random_attack = self.power as f64 * random_value;
    random_attack as u16
  }

  pub fn receive_damage(&mut self, damage: u32) -> u32 {
    self.life -= damage;
    self.life
  }

  pub fn get_name(&self) -> &String {
    &self.name
  }

  pub fn get_life(&self) -> &u32 {
    &self.life
  }
}
