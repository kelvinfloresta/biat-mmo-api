use crate::entities::player::Player;
use rand::Rng;


pub fn attack_pvp(attacker: &Player, defender: &mut Player) {
    let (min, max) = attacker.attack();
    let num = rand::thread_rng().gen_range(min..max);
    defender.take_damage(num);
}