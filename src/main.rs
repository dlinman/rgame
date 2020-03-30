
extern crate rand;

mod game_form;
mod tic_tac_toe;
mod agents;

use agents::agent::{Agent};
use agents::{strict_random_agent, pick_highest_action_agent, self_minimax_agent};
use game_form::{Game, GameResult};
use tic_tac_toe::*;


fn main() {
    play_tic_tac_toe();
}

fn play_tic_tac_toe() {
    let ttt = get_game();
    let mut p1 = self_minimax_agent::get_agent(0, 3); 
    let mut p2 = pick_highest_action_agent::get_agent(1);

    let mut p1_win = 0;
    let mut p2_win = 0;
    let mut draw = 0;
    for _ in 0..100 {

        let mut s = ttt.initial_state();

        while matches!(ttt.game_status(&s), GameResult::NotFinished) {
            let turn = match ttt.player_turn(&s) {
                0 => p1.decide_turn(&ttt, &s),
                1 => p2.decide_turn(&ttt, &s),
                _ => panic!("unknown player"),
            };

            s = ttt.take_turn(&s, &turn);

            //println!("{}\n", ttt.display_state(&s));
        }

        match ttt.game_status(&s) {
            GameResult::NotFinished => panic!("Game not finished"),
            GameResult::Winner{ player, scores: _ } => {
                if player == 0 {
                    p1_win = p1_win + 1;
                }
                else {
                    p2_win = p2_win + 1;
                }
            },
            GameResult::Draw{ scores: _ } => draw = draw + 1,
        }
        /*match ttt.game_status(&s) {
            GameResult::NotFinished => panic!("Game not finished"),
            GameResult::Winner{ player, scores: _ } => println!("winner is player {}", player),
            GameResult::Draw{ scores: _ } => println!("draw"),
        }*/
    }

    println!("Player 1 wins = {}; player 2 wins = {}, draws = {}", p1_win, p2_win, draw);
}