pub mod game;
pub mod strategy;

use game::State;

fn first_player_point(state: &State) -> i32 {
    if state.is_lose() {
        if state.is_first_player() {
            -1
        } else {
            1
        }
    } else {
        0
    }
}
pub fn play<F1: Fn(&State) -> u16, F2: Fn(&State) -> u16>(next_action_functions: &(F1, F2)) -> i32 {
    let mut state = State::new();
    loop {
        if state.is_done() {
            break;
        }
        let action = if state.is_first_player() {
            next_action_functions.0(&state)
        } else {
            next_action_functions.1(&state)
        };
        state = state.next(action);
    }
    first_player_point(&state)
}
