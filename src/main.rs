
extern crate rand;

mod game_form;
mod tic_tac_toe;
mod agents;

use agents::agent::{Agent};
use agents::strict_random_agent;
use game_form::{Game, GameResult};
use tic_tac_toe::*;


fn main() {
    let ttt = get_game();
    let mut s = ttt.initial_state();
    let mut p1 = strict_random_agent::get_agent();
    let mut p2 = strict_random_agent::get_agent();

    while matches!(ttt.game_status(&s), GameResult::NotFinished) {
        let turn = match ttt.player_turn(&s) {
            0 => p1.decide_turn(&ttt, &s),
            1 => p2.decide_turn(&ttt, &s),
            _ => panic!("unknown player"),
        };

        s = ttt.take_turn(&s, &turn);

        println!("{}\n", ttt.display_state(&s));
    }

    match ttt.game_status(&s) {
        GameResult::NotFinished => panic!("Game not finished"),
        GameResult::Winner{ player, scores: _ } => println!("winner is player {}", player),
        GameResult::Draw{ scores: _ } => println!("draw"),
    }

}
