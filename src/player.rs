
use super::game_form::{Game};

pub trait Player {
    fn decide_turn<G : Game>(&self, game : &G, state : &G::State) -> G::TurnAction;
}