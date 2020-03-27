
use crate::game_form::{Game, GameResult};

struct TicTacToeGame {

}

#[derive(Clone, Copy)]
enum Square {
    X,
    O,
    Empty,
}

#[derive(Clone)]
struct TicTacToeState {
    board : Vec<Vec<Square>>,
    player_turn : u32,
}

struct TicTacToeTurn {
    row : usize,
    col : usize,
    square : Square,
    player_turn : u32,
}

struct Turns {
    s0 : TicTacToeState,
    row : usize,
    col : usize,
}

impl Iterator for Turns {
    type Item = TicTacToeTurn;

    fn next(&mut self) -> Option<TicTacToeTurn> {
        loop {
            if self.col == 3 {
                return None;
            }
            if self.row == 3 {
                self.row = 0;
            }
            if matches!(self.s0.board[self.row][self.col], Square::Empty) {
                break;
            }
            self.col = self.col + 1;
            self.row = self.row + 1;
        }

        let square = if self.s0.player_turn == 0 {
            Square::X
        }
        else {
            Square::O
        };

        Some(TicTacToeTurn{row: self.row, col: self.col, square, player_turn: self.s0.player_turn})
    }
}

impl Game for TicTacToeGame {
    type State = TicTacToeState;
    type TurnAction = TicTacToeTurn;
    type T = Turns; 

    fn initial_state(&self) -> TicTacToeState {
        let mut board = vec![];
        for r in 0..3 {
            board.push(vec![]);
            for _ in 0..3 {
                board[r].push(Square::Empty);
            }
        }
        TicTacToeState { board, player_turn: 0 }
    }

    fn take_turn(&self, state : &TicTacToeState, turn_action : TicTacToeTurn) -> TicTacToeState {
        let mut new_state = state.clone();
        new_state.board[turn_action.row][turn_action.col] = turn_action.square;
        new_state.player_turn = (state.player_turn + 1) % 2;
        new_state
    }

    fn legal_turns(&self, state : &TicTacToeState) -> Turns {
        Turns { s0: state.clone(), row: 0, col: 0 }
    }

    fn state_score(&self, state : &TicTacToeState) -> i32 {
        0
    }

    fn players_allowed(&self) -> u32 { 2 }

    fn player_turn(&self, state : &TicTacToeState) -> u32 {
        state.player_turn
    }

    fn display_turn(&self, turn : &TicTacToeTurn) -> String {
        fn p(s : Square) -> char {
            match s {
                Square::X => 'X',
                Square::O => 'O',
                Square::Empty => ' ',
            }
        }
        format!( "Player {}: Row = {}; Col = {}; Symbol = {}", turn.player_turn, turn.row, turn.col, p(turn.square) )
    }

    fn display_state(&self, state : &TicTacToeState) -> String {
        fn p(s : Square) -> char {
            match s {
                Square::X => 'X',
                Square::O => 'O',
                Square::Empty => ' ',
            }
        }
        let mut dis = vec![];
        for r in 0..3 {
            dis.push(format!("{} | {} | {}", p(state.board[r][0]), p(state.board[r][1]), p(state.board[r][2])));
            if r != 3 {
                    dis.push("------------".to_string());
            }
        }
        dis.push("".to_string());
        dis.push(format!("Player {}'s turn", state.player_turn));
        dis.into_iter().collect::<String>()
    }

    fn game_status(&self, state : &Self::State) -> GameResult {
        GameResult::NotFinished
    }
}
