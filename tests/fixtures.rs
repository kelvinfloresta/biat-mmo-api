use biat_mmo_api::entities::player::Player;

pub fn create_player() -> Player {
    Player::new("John", 100, 10)
}
