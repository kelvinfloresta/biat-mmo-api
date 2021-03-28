mod entities;

#[cfg(test)]
mod tests {
  use super::*;
  use entities::player::Player;

  #[test]
  fn should_have_life() {
    let player = Player::new("John", 100, 10);

    let expected = 100 as u32;
    assert_eq!(&expected, player.get_life());
  }

  #[test]
  fn should_have_name() {
    let player = Player::new("John", 100, 10);
    assert_eq!("John", player.get_name());
  }
}