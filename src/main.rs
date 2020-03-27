
mod game_form;
mod tic_tac_toe;
use game_form::{Game};
use tic_tac_toe::*;


fn main() {
    let ttt = get_game();
    let s0 = ttt.initial_state();

    println!("{}", ttt.display_state(&s0)); 

    let turns = ttt.legal_turns(&s0);
    for turn in turns {
        println!("{}", ttt.display_turn(&turn));
    }
}
