
use crate::games::game_form::{Game, GameResult, HeuristicDescription};

pub fn get_game() -> impl Game {
    Ultimate {}
}

struct Ultimate { }

#[derive(Clone, Copy, PartialEq)]
enum Square {
    X,
    O,
    Empty,
}

#[derive(Clone, PartialEq)]
enum MiniBoard {
    X,
    O,
    Board(Vec<Vec<Square>>),
    Draw(Vec<Vec<Square>>),
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
    row : usize,
    col : usize,
    mini_board_row : usize,
    mini_board_col : usize,
    player_turn : u32,
    square : Square,
}

struct Turns {
    s0 : UState,
    row : usize,
    col : usize,
    mini_row : usize,
    mini_col : usize,
}

enum UHeuristic {
    Default,
}

impl Iterator for Turns {
    type Item = UTurn;

    fn next(&mut self) -> Option<UTurn> {
        loop {
            if self.mini_row == 3 {
                self.mini_row = 0;
                self.mini_col = self.mini_col + 1;
            }
            if self.mini_col == 3 {
                self.mini_col = 0;
                self.row = self.row + 1;
            }
            if self.row == 3 {
                self.row = 0;
                self.col = self.col + 1;
            }
            if self.col == 3 {
                return None;
            }
            
            if matches!(self.s0.board[self.row][self.col], MiniBoard::Board(_)) {
                match &self.s0.board[self.row][self.col] {
                    MiniBoard::Board(b) => {
                        if matches!(b[self.mini_row][self.mini_col], Square::Empty) {
                            break;
                        }
                    }
                    _ => panic!("Mini board should be board"),
                }
            }
            self.mini_row = self.mini_row + 1;
        }

        let square = if self.s0.player_turn == 0 {
            Square::X
        }
        else {
            Square::O
        };

        let ret = UTurn{ row: self.row
                       , col: self.col
                       , mini_board_row: self.mini_row
                       , mini_board_col: self.mini_col
                       , square
                       , player_turn: self.s0.player_turn
                       };
        self.row = self.row + 1;
        Some(ret)
    }
}


impl Game for Ultimate {

    type State = UState;
    type TurnAction = UTurn;
    type T = Turns;
    type Heuristic = UHeuristic;

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
        let (r, c) = match state.target_board {
            BoardAllowed::Any => (turn_action.row, turn_action.col),
            BoardAllowed::At{row,col} => (row,col),
        };

        let mut new_state = state.clone();
        let mini_board = match &mut new_state.board[r][c] {
            MiniBoard::Board(b) => b,
            _ => panic!("Attempting to make move on completed board")
        };
        mini_board[turn_action.mini_board_row][turn_action.mini_board_col] = turn_action.square;
        match check_mini_status(mini_board) {
            GameResult::Winner {player, scores: _} if player == 0 => new_state.board[r][c] = MiniBoard::X,
            GameResult::Winner {player: _, scores: _} => new_state.board[r][c] = MiniBoard::O,
            GameResult::Draw {scores: _} => new_state.board[r][c] = MiniBoard::Draw(mini_board.clone()),
            GameResult::NotFinished => (),
        }

        new_state.player_turn = (state.player_turn + 1) % 2;

        new_state.target_board = match new_state.board[turn_action.mini_board_row][turn_action.mini_board_col] {
            MiniBoard::Board(_) => BoardAllowed::At { row: turn_action.mini_board_row, col: turn_action.mini_board_col },
            _ => BoardAllowed::Any,
        };

        new_state
    }

    fn legal_turns(&self, state : &UState) -> Turns {
        Turns { s0: state.clone(), row: 0, col: 0, mini_row: 0, mini_col: 0 }
    }

    fn heuristics(&self) -> Vec<(HeuristicDescription, UHeuristic)> {
        vec![(HeuristicDescription::Default, UHeuristic::Default)

            ]
    } 

    fn state_score(&self, state : &UState, heuristic : &UHeuristic, player : u32) -> i32 {
        // lose penalty 
        // win bonus 
        // target board = any bonus when its your turn
        // target board = any penalty when its your opponents turn
        // target board = at of board that you want to win bonus
        0
    }

    fn players_allowed(&self) -> u32 { 2 }

    fn player_turn(&self, state : &UState) -> u32 {
        state.player_turn
    }

    fn display_turn(&self, turn : &UTurn) -> String {
        fn p(s : Square) -> char {
            match s {
                Square::X => 'X',
                Square::O => 'O',
                Square::Empty => ' ',
            }
        }
        format!( "Player {}: Board Row = {}; Board Col = {}; MiniBoard Row = {}; MiniBoard Col = {}; Symbol = {}\n", 
            turn.player_turn + 1, 
            turn.row, 
            turn.col, 
            turn.mini_board_row,
            turn.mini_board_col,
            p(turn.square) )
    }

    fn display_state(&self, state : &UState) -> String {
        fn sq(s : Square) -> char {
            match s {
                Square::X => 'X',
                Square::O => 'O',
                Square::Empty => ' ',
            }
        }
        fn mini(b : &Vec<Vec<Square>>) -> [String; 5] {
            let mut ret = ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()];
            ret[0] = format!(" {} | {} | {} ", sq(b[0][0]), sq(b[0][1]), sq(b[0][2]));
            ret[1] = format!("-----------");
            ret[2] = format!(" {} | {} | {} ", sq(b[1][0]), sq(b[1][1]), sq(b[1][2]));
            ret[3] = format!("-----------");
            ret[4] = format!(" {} | {} | {} ", sq(b[2][0]), sq(b[2][1]), sq(b[2][2]));
            ret
        }
        fn display(b : &MiniBoard) -> [String; 5] {
            let big_x = [ "           ".to_string()
                        , "    \\ /    ".to_string()
                        , "     .     ".to_string()
                        , "    / \\    ".to_string()
                        , "           ".to_string()
                        ];
            let big_o = [ "   ______  ".to_string()
                        , "   |    |  ".to_string()
                        , "   |    |  ".to_string()
                        , "   ------  ".to_string()
                        , "           ".to_string()
                        ];
            match b {
                MiniBoard::X => big_x,
                MiniBoard::O => big_o,
                MiniBoard::Board(board) => mini(board),
                MiniBoard::Draw(board) => mini(board),
            }
        }

        let mut dis = vec![];
        dis.push("\n".to_string());

        for r in 0..3 {
            let m0 = display(&state.board[r][0]);
            let m1 = display(&state.board[r][1]);
            let m2 = display(&state.board[r][2]);
            dis.push(format!("{}||{}||{}\n", m0[0], m1[0], m2[0]));
            dis.push(format!("{}||{}||{}\n", m0[1], m1[1], m2[1]));
            dis.push(format!("{}||{}||{}\n", m0[2], m1[2], m2[2]));
            dis.push(format!("{}||{}||{}\n", m0[3], m1[3], m2[3]));
            dis.push(format!("{}||{}||{}\n", m0[4], m1[4], m2[4]));
            if r != 2 {
                dis.push(format!("=====================================\n"));
            }
        }

        dis.push("\n".to_string());
        dis.into_iter().collect::<String>()
    }

    fn game_status(&self, state : &UState) -> GameResult {
        use MiniBoard::*;
        use crate::games::game_form::GameResult::*;

        fn row_won(line : &[MiniBoard]) -> Option<GameResult> {
            match line {
                [X, X, X] => Some( Winner { player: 0, scores: vec![] } ),
                [O, O, O] => Some( Winner { player: 1, scores: vec![] } ),
                _ => None,
            }
        }

        fn won(line : &[&MiniBoard]) -> Option<GameResult> {
            match line {
                [X, X, X] => Some( Winner { player: 0, scores: vec![] } ),
                [O, O, O] => Some( Winner { player: 1, scores: vec![] } ),
                _ => None,
            }
        }

        for r in 0..3 {
            let result = row_won(&state.board[r][..]); 
            if matches!(result, Some(_)) {
                return result.unwrap();
            }
        }

        let c0 = vec![ &state.board[0][0], &state.board[1][0], &state.board[2][0] ];
        let result_c0 = won(&c0[..]); 
        if matches!(result_c0, Some(_)) {
            return result_c0.unwrap();
        }

        let c1 = vec![ &state.board[0][1], &state.board[1][1], &state.board[2][1] ];
        let result_c1 = won(&c1); 
        if matches!(result_c1, Some(_)) {
            return result_c1.unwrap();
        }

        let c2 = vec![ &state.board[0][2], &state.board[1][2], &state.board[2][2] ];
        let result_c2 = won(&c2); 
        if matches!(result_c2, Some(_)) {
            return result_c2.unwrap();
        }

        let d0 = vec![ &state.board[0][0], &state.board[1][1], &state.board[2][2] ];
        let result_d0 = won(&d0); 
        if matches!(result_d0, Some(_)) {
            return result_d0.unwrap();
        }

        let d1 = vec![ &state.board[2][0], &state.board[1][1], &state.board[0][2] ];
        let result_d1 = won(&d1); 
        if matches!(result_d1, Some(_)) {
            return result_d1.unwrap();
        }

        if state.board.iter().map(|row| row.iter()).flatten().filter(|s| matches!(s, MiniBoard::Board(_))).count() == 0 {
            GameResult::Draw { scores: vec![] }
        } 
        else {
            GameResult::NotFinished
        }
    }
}


fn check_mini_status(mini : &Vec<Vec<Square>>) -> GameResult {
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
        let result = won(&mini[r][..]); 
        if matches!(result, Some(_)) {
            return result.unwrap();
        }
    }

    let c0 = vec![ mini[0][0], mini[1][0], mini[2][0] ];
    let result_c0 = won(&c0); 
    if matches!(result_c0, Some(_)) {
        return result_c0.unwrap();
    }

    let c1 = vec![ mini[0][1], mini[1][1], mini[2][1] ];
    let result_c1 = won(&c1); 
    if matches!(result_c1, Some(_)) {
        return result_c1.unwrap();
    }

    let c2 = vec![ mini[0][2], mini[1][2], mini[2][2] ];
    let result_c2 = won(&c2); 
    if matches!(result_c2, Some(_)) {
        return result_c2.unwrap();
    }

    let d0 = vec![ mini[0][0], mini[1][1], mini[2][2] ];
    let result_d0 = won(&d0); 
    if matches!(result_d0, Some(_)) {
        return result_d0.unwrap();
    }

    let d1 = vec![ mini[2][0], mini[1][1], mini[0][2] ];
    let result_d1 = won(&d1); 
    if matches!(result_d1, Some(_)) {
        return result_d1.unwrap();
    }

    if mini.iter().map(|row| row.iter()).flatten().filter(|s| matches!(s, Square::Empty)).count() == 0 {
        GameResult::Draw { scores: vec![] }
    } 
    else {
        GameResult::NotFinished
    }
}