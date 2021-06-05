use crate::State;
const EXPAND_THRESHOLD: usize = 10;
const MCTS_SIMULATION_COUNT: usize = 1000;
struct Node {
    state: State,
    w: f64,
    n: usize,
    child_nodes: Vec<Node>,
}
impl Node {
    fn new(state: State) -> Self {
        Node {
            state,
            w: 0.0,
            n: 0,
            child_nodes: vec![],
        }
    }
    fn evaluate(&mut self) -> f64 {
        if self.state.is_done() {
            let value = if self.state.is_lose() { -1.0 } else { 0.0 };
            self.w += value;
            self.n += 1;
            value
        } else if self.child_nodes.is_empty() {
            let value = State::playout(&self.state);
            self.w += value;
            self.n += 1;
            if self.n == EXPAND_THRESHOLD {
                self.expand();
            }
            value
        } else {
            let next_node_index = self.next_child_node_index();
            let value = -self.child_nodes[next_node_index].evaluate();
            self.w += value;
            self.n += 1;
            value
        }
    }
    fn expand(&mut self) {
        let legal_actions = self.state.legal_actions();
        for action in legal_actions {
            self.child_nodes.push(Node::new(self.state.next(action)));
        }
    }
    fn next_child_node_index(&self) -> usize {
        for i in 0..self.child_nodes.len() {
            if self.child_nodes[i].n == 0 {
                return i;
            }
        }
        let mut t = 0;
        for c in self.child_nodes.iter() {
            t += c.n;
        }
        let mut max_value = f64::MIN;
        let mut best_node = 0;
        for i in 0..self.child_nodes.len() {
            let ucb1 = self.child_nodes[i].ucb1_value(t);
            if ucb1 > max_value {
                max_value = ucb1;
                best_node = i;
            }
        }
        best_node
    }
    pub fn ucb1_value(&self, total: usize) -> f64 {
        if self.n == 0 {
            f64::MAX
        } else {
            -self.w / self.n as f64 + (2.0 * (total as f64).ln() / self.n as f64).sqrt()
        }
    }
}

pub fn mcts_action_count(state: &State) -> u16 {
    let mut root_node = Node::new(state.clone());
    root_node.expand();

    for _ in 0..MCTS_SIMULATION_COUNT {
        root_node.evaluate();
    }

    // legal_actionsが常に同じ順番でactionを返すことを期待している。
    let mut best_action = 0;
    let mut best_score = usize::MIN;
    let legal_actions = state.legal_actions();
    for i in 0..legal_actions.len() {
        let score = root_node.child_nodes[i].n;
        if score > best_score {
            best_action = legal_actions[i];
            best_score = score;
        }
    }
    best_action
}

pub fn mcts_action_value(state: &State) -> u16 {
    let mut root_node = Node::new(state.clone());
    root_node.expand();

    for _ in 0..MCTS_SIMULATION_COUNT {
        root_node.evaluate();
    }

    // legal_actionsが常に同じ順番でactionを返すことを期待している。
    let mut best_action = 0;
    let mut best_score = f64::MIN;
    let legal_actions = state.legal_actions();
    for i in 0..legal_actions.len() {
        let score = -root_node.child_nodes[i].w;
        if score > best_score {
            best_action = legal_actions[i];
            best_score = score;
        }
    }
    best_action
}
