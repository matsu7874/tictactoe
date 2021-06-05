use crate::State;
use rand::prelude::*;

pub fn random_action(state: &State) -> u16 {
    let mut rng = rand::thread_rng();
    let legal_actions = state.legal_actions();
    *legal_actions.choose(&mut rng).unwrap()
}
