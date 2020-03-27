

pub trait Game {

    type State : Copy;
    type TurnAction;
    type T : Iterator<Item = Self::TurnAction>;

    fn initial_state(&self) -> Self::State;

    fn take_turn(&self, state : &Self::State, turn_action : Self::TurnAction) -> Self::State;

    fn legal_turns(&self, state : &Self::State) -> Self::T;

    fn state_value(&self, state : &Self::State) -> u32;

    fn players_allowed(&self) -> u32;

    fn player_turn(&self) -> u32;
}
