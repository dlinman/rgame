
extern crate rand;

mod game_form;
mod tic_tac_toe;
mod player;

use rand::prelude::*;
use game_form::{Game, GameResult};
use tic_tac_toe::*;


fn main() {
    let mut rng = rand::thread_rng();
    let ttt = get_game();
    let mut s = ttt.initial_state();

    while matches!(ttt.game_status(&s), GameResult::NotFinished) {
        let mut turns = ttt.legal_turns(&s).collect::<Vec<_>>();
        turns.shuffle(&mut rng);

        s = ttt.take_turn(&s, &turns[0]);

        println!("{}", ttt.display_turn(&turns[0]));
        println!("{}", ttt.display_state(&s));
    }

    match ttt.game_status(&s) {
        GameResult::NotFinished => panic!("Game not finished"),
        GameResult::Winner{ player, scores: _ } => println!("winner is player {}", player),
        GameResult::Draw{ scores: _ } => println!("draw"),
    }

}
