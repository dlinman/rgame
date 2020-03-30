
use crate::game_form::{Game};
use super::agent::Agent; 

use std::io::{self, Read};

pub fn get_agent() -> impl Agent {
    HumanAgent {  }
}

struct HumanAgent {}

impl Agent for HumanAgent {
    fn decide_turn<G : Game>(&mut self, game : &G, state : &G::State) -> G::TurnAction {
        for (id, turn) in game.legal_turns(state).enumerate() {
            println!("================================================\n\n");
            println!("Turn ID = {}\n\n", id);
            println!("{}\n\n", game.display_turn(&turn));

            let new_state = game.take_turn(state, &turn);

            println!("{}\n\n", game.display_state(&new_state));
        }

        // TODO check to ensure that we get a number and that it returns something from nth
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();
        let choice = buffer.parse::<usize>().unwrap();

        game.legal_turns(state).nth(choice).unwrap()
    }
}
