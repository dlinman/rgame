
mod game_form;
use game_form::{Game};

fn z<G:Game>( g : G ) {
    let s1 = g.initial_state();
    let turns = g.legal_turns(&s1);

    let x : Vec<G::State> = turns.map( |turn| g.take_turn(&s1, turn)).collect();
}

fn main() {
    
    println!("Hello, world!");
}
