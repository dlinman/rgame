
use crate::games::game_form::{Game, GameResult};

pub fn get_game() -> impl Game {
    TicTacToeGame {}
}

struct TicTacToeGame { }

#[derive(Clone, Copy, PartialEq, Eq)]
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
            if self.row == 3 {
                self.row = 0;
                self.col = self.col + 1;
            }
            if self.col == 3 {
                return None;
            }
            if matches!(self.s0.board[self.row][self.col], Square::Empty) {
                break;
            }
            self.row = self.row + 1;
        }

        let square = if self.s0.player_turn == 0 {
            Square::X
        }
        else {
            Square::O
        };

        let ret = TicTacToeTurn{row: self.row, col: self.col, square, player_turn: self.s0.player_turn};
        self.row = self.row + 1;
        Some(ret)
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

    fn take_turn(&self, state : &TicTacToeState, turn_action : &TicTacToeTurn) -> TicTacToeState {
        let mut new_state = state.clone();
        new_state.board[turn_action.row][turn_action.col] = turn_action.square;
        new_state.player_turn = (state.player_turn + 1) % 2;
        new_state
    }

    fn legal_turns(&self, state : &TicTacToeState) -> Turns {
        Turns { s0: state.clone(), row: 0, col: 0 }
    }

    fn state_score(&self, state : &TicTacToeState, player : u32) -> i32 {
        fn score(line : Vec<Square>, s : Square) -> i32 {
            let mut home_count = 0;
            let mut empty_count = 0;
            let mut enemy_count = 0;
            for i in 0..3 {
                if Square::Empty == line[i] {
                    empty_count = empty_count + 1;
                }
                else if s == line[i] {
                    home_count = home_count + 1;
                }
                else { 
                    enemy_count = enemy_count + 1;
                }
            }

            match (home_count, empty_count, enemy_count) {
                // X X X
                (3, _, _) => 10000,
                // O O X
                (1, _, 2) => 1000,
                // X X E
                (2, 1, _) => 100,
                // O X E
                (1, 1, 1) => 10,
                // X E E
                (1, 2, _) => 1,

                // O O O
                (_, _, 3) => -10000,
                // O O X
                /*(2, _, 1) => -2000,
                // X X E
                (_, 1, 2) => -200,
                // X E E
                (_, 2, 1) => -2,*/
                _ => 0,
            }
        }

        let square = if player == 0 {
            Square::X
        }
        else {
            Square::O
        };

        let r0 = vec![state.board[0][1], state.board[0][1], state.board[0][2]];
        let r1 = vec![state.board[1][1], state.board[1][1], state.board[1][2]];
        let r2 = vec![state.board[2][1], state.board[2][1], state.board[2][2]];

        let c0 = vec![state.board[0][0], state.board[1][0], state.board[2][0]];
        let c1 = vec![state.board[0][1], state.board[1][1], state.board[2][1]];
        let c2 = vec![state.board[0][2], state.board[1][2], state.board[2][2]];

        let d0 = vec![state.board[0][0], state.board[1][1], state.board[2][2]];
        let d1 = vec![state.board[2][0], state.board[1][1], state.board[0][2]];

        score( r0, square ) 
        + score( r1, square )
        + score( r2, square )
        + score( c0, square )
        + score( c1, square )
        + score( c2, square )
        + score( d0, square )
        + score( d1, square )
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
        format!( "Player {}: Row = {}; Col = {}; Symbol = {}\n", turn.player_turn + 1, turn.row, turn.col, p(turn.square) )
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
        println!("");
        for r in 0..3 {
            dis.push(format!(" {} | {} | {}\n", p(state.board[r][0]), p(state.board[r][1]), p(state.board[r][2])));
            if r != 2 {
                    dis.push("-----------\n".to_string());
            }
        }
        dis.push("".to_string());
        dis.into_iter().collect::<String>()
    }

    fn game_status(&self, state : &Self::State) -> GameResult {
        use Square::*;
        use crate::games::game_form::GameResult::*;
        fn won(line : &[Square]) -> Option<GameResult> {
            match line {
                [X, X, X] => Some( Winner { player: 0, scores: vec![] } ),
                [O, O, O] => Some( Winner { player: 1, scores: vec![] } ),
                _ => None,
            }
        }

        for r in 0..3 {
            let result = won(&state.board[r][..]); 
            if matches!(result, Some(_)) {
                return result.unwrap();
            }
        }

        let c0 = vec![ state.board[0][0], state.board[1][0], state.board[2][0] ];
        let result_c0 = won(&c0); 
        if matches!(result_c0, Some(_)) {
            return result_c0.unwrap();
        }

        let c1 = vec![ state.board[0][1], state.board[1][1], state.board[2][1] ];
        let result_c1 = won(&c1); 
        if matches!(result_c1, Some(_)) {
            return result_c1.unwrap();
        }

        let c2 = vec![ state.board[0][2], state.board[1][2], state.board[2][2] ];
        let result_c2 = won(&c2); 
        if matches!(result_c2, Some(_)) {
            return result_c2.unwrap();
        }

        let d0 = vec![ state.board[0][0], state.board[1][1], state.board[2][2] ];
        let result_d0 = won(&d0); 
        if matches!(result_d0, Some(_)) {
            return result_d0.unwrap();
        }

        let d1 = vec![ state.board[2][0], state.board[1][1], state.board[0][2] ];
        let result_d1 = won(&d1); 
        if matches!(result_d1, Some(_)) {
            return result_d1.unwrap();
        }

        if state.board.iter().map(|row| row.iter()).flatten().filter(|s| matches!(s, Square::Empty)).count() == 0 {
            GameResult::Draw { scores: vec![] }
        } 
        else {
            GameResult::NotFinished
        }
    }
}