
extern crate rand;

mod agents;
mod two_player_contest;
mod single_game;
mod games;

use games::game_form::{HeuristicDescription};
use agents::{strict_random_agent, pick_highest_action_agent, self_minimax_agent, alpha_beta_agent, human_player};


fn main() {
    let ttt = games::ultimate_tic_tac_toe::get_game();
    let mut p1 = alpha_beta_agent::get_agent(HeuristicDescription::Default, 0, 3); 
    let mut p2 = alpha_beta_agent::get_agent(HeuristicDescription::Default, 1, 4);
    single_game::play(&ttt, &mut p1, &mut p2);
    /*let ttt = games::tic_tac_toe::get_game();
    let mut p1 = self_minimax_agent::get_agent(HeuristicDescription::Default, 0, 3); 
    let mut p2 = strict_random_agent::get_agent();
    two_player_contest::contest(&ttt, &mut p1, &mut p2, 100);*/
}
