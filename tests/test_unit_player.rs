extern crate biat_mmo_api;

#[cfg(test)]
mod tests {
  use biat_mmo_api::entities::player::Player;

  #[test]
  fn should_have_name() {
    let player = Player::new("John", 100, 10);

    assert_eq!("John", player.get_name());
  }

  #[test]
  fn should_have_life() {
    let player = Player::new("John", 100, 10);
    let expected: u16 = 100;
    assert_eq!(&expected, player.get_life());
  }

  #[test]
  fn should_calculate_attack_correctly() {
    let player = Player::new("John", 100, 10);
    let (min, max) = player.attack();
    assert!(min == 10);
    assert!(max  == 10 * 2);
  }

  #[test]
  fn should_receive_damage() {
    let mut player = Player::new("John 2", 100, 100);
    let new_life = *player.take_damage(10);

    assert_eq!(90, new_life);
  }
}
