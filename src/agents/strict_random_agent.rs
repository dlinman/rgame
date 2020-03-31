
use rand::prelude::*;
use crate::games::game_form::{Game};
use super::agent::Agent; 

pub fn get_agent() -> impl Agent {
    StrictRandomAgent{ rng : rand::thread_rng() }
}

struct StrictRandomAgent {
    rng : ThreadRng,
}

impl Agent for StrictRandomAgent {
    fn decide_turn<G : Game>(&mut self, game : &G, state : &G::State) -> G::TurnAction {
        let mut turns = game.legal_turns(&state).collect::<Vec<_>>();
        turns.shuffle(&mut self.rng);
        turns.into_iter().nth(0).unwrap()
    }
}
