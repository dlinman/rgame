
use crate::games::game_form::{Game};

pub trait Agent {
    fn decide_turn<G : Game>(&mut self, game : &G, state : &G::State) -> G::TurnAction;
}