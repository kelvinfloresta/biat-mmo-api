
extern crate biat_mmo_api;

#[cfg(test)]
mod tests {
  use biat_mmo_api::entities::player::{Player};

  #[test]
  fn should_have_name() {
    let player = Player::new("John", 100, 10);

    assert_eq!("John", player.get_name());
  }

  #[test]
  fn should_have_life() {
    let player = Player::new("John", 100, 10);
    let expected: u32 = 100;
    assert_eq!(&expected, player.get_life());
  }


  #[test]
  fn should_not_panic_when_attack() {
    let player = Player::new("John", 100, 10);
    assert!(player.attack() >= 0 as u16);
  }
}
