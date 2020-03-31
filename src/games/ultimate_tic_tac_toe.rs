
use crate::games::game_form::{Game, GameResult};

pub fn get_game() -> impl Game {
    Ultimate {}
}

struct Ultimate { }

#[derive(Clone)]
enum Square {
    X,
    O,
    Empty,
}

#[derive(Clone)]
enum MiniBoard {
    X,
    O,
    Board(Vec<Vec<Square>>),
}

#[derive(Clone)]
enum BoardAllowed {
    Any,
    At{ row : usize, col : usize } 
}

#[derive(Clone)]
struct UState {
    player_turn : u32,
    target_board : BoardAllowed,
    board : Vec<Vec<MiniBoard>>,
}

struct UTurn {

}

struct Turns {

}

impl Iterator for Turns {
    type Item = UTurn;

    fn next(&mut self) -> Option<UTurn> {
        None
    }
}


impl Game for Ultimate {

    type State = UState;
    type TurnAction = UTurn;
    type T = Turns;

    fn initial_state(&self) -> UState {
        fn mini_board() -> MiniBoard {
            let mut board = vec![];
            for r in 0..3 {
                board.push(vec![]);
                for _ in 0..3 {
                    board[r].push(Square::Empty);
                }
            }
            MiniBoard::Board(board)
        }

        let mut board = vec![]; 
        for r in 0..3 {
            board.push(vec![]);
            for _ in 0..3 {
                board[r].push(mini_board());
            }
        }

        UState {
            player_turn: 0,
            target_board: BoardAllowed::Any,
            board
        }
    }

    fn take_turn(&self, state : &UState, turn_action : &UTurn) -> UState {
        state.clone()
    }

    fn legal_turns(&self, state : &UState) -> Turns {
        Turns {}
    }

    fn state_score(&self, state : &UState, player : u32) -> i32 {
        0
    }

    fn players_allowed(&self) -> u32 { 2 }

    fn player_turn(&self, state : &UState) -> u32 {
        state.player_turn
    }

    fn display_turn(&self, turn : &UTurn) -> String {
        "".to_string()
    }

    fn display_state(&self, state : &UState) -> String {
        "".to_string()
    }

    fn game_status(&self, state : &UState) -> GameResult {
        GameResult::NotFinished
    }
}

