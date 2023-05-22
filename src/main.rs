mod entities;
use entities::player::Player;

fn log(player: &Player) {
  println!("Player");
  println!("name: {}", player.get_name());
  println!("life: {}", player.get_life());
}

fn main() {
  let john = Player::new("John", 100, 10);
  john.attack();
  log(&john);
}
