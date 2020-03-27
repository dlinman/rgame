
mod game_form;
mod tic_tac_toe;
use game_form::{Game};
use tic_tac_toe::*;


fn main() {
    let ttt = get_game();
    let s0 = ttt.initial_state();

    println!("{}", ttt.display_state(&s0)); 
}
