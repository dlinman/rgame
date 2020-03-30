
extern crate rand;

mod game_form;
mod tic_tac_toe;
mod agents;
mod two_player_contest;

use agents::agent::{Agent};
use agents::{strict_random_agent, pick_highest_action_agent, self_minimax_agent};
use game_form::{Game, GameResult};
use tic_tac_toe::*;


fn main() {
    let ttt = get_game();
    let mut p1 = self_minimax_agent::get_agent(0, 3); 
    let mut p2 = pick_highest_action_agent::get_agent(1);
    two_player_contest::contest(&ttt, &mut p1, &mut p2, 100);
}
