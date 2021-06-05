use tictactoe::{
    game::State,
    play,
    strategy::{
        alpha_beta::alpha_beta_action,
        mcs::mcs_action,
        mcts::{mcts_action_count, mcts_action_value},
        random::random_action,
    },
};
const EP_GAME_COUNT: usize = 1000;

fn evaluate_algorithm_of<F1: Fn(&State) -> u16 + Clone, F2: Fn(&State) -> u16 + Clone>(
    labels: (&str, &str),
    next_action_functions: &(F1, F2),
) {
    let mut result_count = [0, 0, 0];
    for _ in 0..EP_GAME_COUNT {
        let result = play(&next_action_functions);
        if result == -1 {
            result_count[2] += 1;
        } else if result == 0 {
            result_count[1] += 1;
        } else {
            result_count[0] += 1;
        }
    }
    println!(
        "{},{},{},{},{}",
        labels.0, labels.1, result_count[0], result_count[1], result_count[2],
    );
}

fn main() {
    println!(
        "me,opponent,win,drow,lose,MCS_SIMULATION_COUNT,MCTS_SIMULATION_COUNT,EXPAND_THRESHOLD"
    );
    let next_action_functions = (random_action, random_action);
    evaluate_algorithm_of(("random", "random"), &next_action_functions);
    let next_action_functions = (random_action, alpha_beta_action);
    evaluate_algorithm_of(("random", "alpha_beta"), &next_action_functions);
    let next_action_functions = (alpha_beta_action, alpha_beta_action);
    evaluate_algorithm_of(("alpha_beta", "alpha_beta"), &next_action_functions);
    let next_action_functions = (random_action, mcs_action);
    evaluate_algorithm_of(("random", "mcs"), &next_action_functions);
    let next_action_functions = (alpha_beta_action, mcs_action);
    evaluate_algorithm_of(("alpha_beta", "mcs"), &next_action_functions);
    let next_action_functions = (mcs_action, mcs_action);
    evaluate_algorithm_of(("mcs", "mcs"), &next_action_functions);
    let next_action_functions = (random_action, mcts_action_count);
    evaluate_algorithm_of(("random", "mcts_count"), &next_action_functions);

    let next_action_functions = (alpha_beta_action, mcts_action_count);
    evaluate_algorithm_of(("alpha_beta", "mcts_count"), &next_action_functions);

    let next_action_functions = (mcs_action, mcts_action_count);
    evaluate_algorithm_of(("mcs", "mcts_count"), &next_action_functions);

    let next_action_functions = (mcts_action_count, mcs_action);
    evaluate_algorithm_of(("mcts_count", "mcs"), &next_action_functions);

    let next_action_functions = (mcts_action_count, mcts_action_count);
    evaluate_algorithm_of(("mcts_count", "mcts_count"), &next_action_functions);

    let next_action_functions = (mcts_action_count, mcts_action_value);
    evaluate_algorithm_of(("mcts_count", "mcts_value"), &next_action_functions);

    let next_action_functions = (mcts_action_value, mcts_action_count);
    evaluate_algorithm_of(("mcts_value", "mcts_count"), &next_action_functions);
}
