
pub enum GameResult {
    NotFinished,
    Winner { player: u32, scores : Vec<(u32, i32)> },
    Draw { scores : Vec<(u32, i32)> },
}

pub trait Game {

    type State : Clone;
    type TurnAction;
    type T : Iterator<Item = Self::TurnAction>;

    fn initial_state(&self) -> Self::State;

    fn take_turn(&self, state : &Self::State, turn_action : &Self::TurnAction) -> Self::State;

    fn legal_turns(&self, state : &Self::State) -> Self::T;

    fn state_score(&self, state : &Self::State, player : u32) -> i32;

    fn players_allowed(&self) -> u32;

    fn player_turn(&self, state : &Self::State) -> u32;

    fn display_turn(&self, turn : &Self::TurnAction) -> String;

    fn display_state(&self, state : &Self::State) -> String;

    fn game_status(&self, state : &Self::State) -> GameResult;
}
