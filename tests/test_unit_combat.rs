mod fixtures;
extern crate biat_mmo_api;

#[cfg(test)]
mod tests {
    use crate::fixtures;
    use biat_mmo_api::usecases::combat::attack_pvp;

    #[test]
    fn should_take_damage() {
        for _ in 0..1000 {
            let attacker = &fixtures::create_player();
            let defender = &mut fixtures::create_player();
            let initial_life = *defender.get_life();

            attack_pvp(attacker, defender);

            let damaged_life = *defender.get_life();
            assert!(initial_life > damaged_life);
        }
    }
}
