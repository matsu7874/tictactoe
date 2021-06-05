use crate::State;
fn alpha_beta(state: &State, alpha: f64, beta: f64) -> f64 {
    if state.is_lose() {
        return -1.0;
    }
    if state.is_draw() {
        return 0.0;
    }
    let mut alpha = alpha;
    for action in state.legal_actions() {
        let score = -alpha_beta(&state.next(action), -beta, -alpha);
        if score > alpha {
            alpha = score;
        }
        if alpha >= beta {
            return alpha;
        }
    }
    alpha
}
pub fn alpha_beta_action(state: &State) -> u16 {
    let mut best_action = 0;
    let mut alpha = f64::MIN;
    for action in state.legal_actions() {
        let score = -alpha_beta(&state.next(action), f64::MIN, -alpha);
        if score > alpha {
            best_action = action;
            alpha = score;
        }
    }
    best_action
}
