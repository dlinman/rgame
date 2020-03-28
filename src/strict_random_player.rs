
use rand::prelude::*;
use super::game_form::{Game};
use super::player::Player;

pub fn get_player() -> impl Player {
    StrictRandomPlayer{ rng : rand::thread_rng() }
}

struct StrictRandomPlayer {
    rng : ThreadRng,
}

impl Player for StrictRandomPlayer {
    fn decide_turn<G : Game>(&mut self, game : &G, state : &G::State) -> G::TurnAction {
        let mut turns = game.legal_turns(&state).collect::<Vec<_>>();
        turns.shuffle(&mut self.rng);
        turns.into_iter().nth(0).unwrap()
    }
}
