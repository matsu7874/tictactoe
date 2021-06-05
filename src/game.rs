use crate::strategy::random::random_action;

const WIN_PATTERNS: [u16; 8] = [
    0b111,
    0b111000,
    0b111000000,
    0b1001001,
    0b10010010,
    0b100100100,
    0b100010001,
    0b1010100,
];

#[derive(Clone, Copy)]
pub struct State {
    my_stones: u16,
    opponent_stones: u16,
}
impl State {
    pub fn new() -> Self {
        State {
            my_stones: 0,
            opponent_stones: 0,
        }
    }
    pub fn is_lose(&self) -> bool {
        for &p in WIN_PATTERNS.iter() {
            if self.opponent_stones & p == p {
                return true;
            }
        }
        false
    }
    pub fn is_draw(&self) -> bool {
        self.my_stones.count_ones() + self.opponent_stones.count_ones() == 9
    }
    pub fn is_done(&self) -> bool {
        self.is_lose() || self.is_draw()
    }
    /// 次の状態を生成する。
    pub fn next(&self, action: u16) -> State {
        State {
            my_stones: self.opponent_stones,
            opponent_stones: self.my_stones | action,
        }
    }

    /// 合法手を列挙する。
    pub fn legal_actions(&self) -> Vec<u16> {
        let mut actions = vec![];
        for i in 0..9 {
            if (self.my_stones | self.opponent_stones) & 1 << i == 0 {
                actions.push(1 << i);
            }
        }
        actions
    }
    pub fn is_first_player(&self) -> bool {
        // 既に打たれた石の数が同じ場合は先手番
        self.my_stones.count_ones() == self.opponent_stones.count_ones()
    }

    pub fn playout(state: &State) -> f64 {
        if state.is_lose() {
            return -1.0;
        }
        if state.is_draw() {
            return 0.0;
        }
        -State::playout(&state.next(random_action(state)))
    }
}
impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbols = if self.is_first_player() {
            ("o", "x")
        } else {
            ("x", "o")
        };
        let mut board = vec![];
        for i in 0..9 {
            if self.my_stones & 1 << i != 0 {
                board.push(symbols.0);
            } else if self.opponent_stones & 1 << i != 0 {
                board.push(symbols.1);
            } else {
                board.push("-");
            }
            if i == 2 || i == 5 {
                board.push("\n");
            }
        }
        write!(f, "{}", board.join(""))
    }
}
