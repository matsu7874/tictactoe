use crate::State;
const MCS_SIMULATION_COUNT: usize = 10000;
pub fn mcs_action(state: &State) -> u16 {
    let mut best_action = 0;
    let mut best_score = f64::MIN;
    for action in state.legal_actions() {
        let mut score = 0.0;
        for _ in 0..MCS_SIMULATION_COUNT {
            score += -State::playout(&state.next(action));
        }
        if score > best_score {
            best_action = action;
            best_score = score;
        }
    }
    best_action
}
