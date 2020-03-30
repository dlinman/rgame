

use crate::agents::agent::{Agent};
use crate::game_form::{Game, GameResult};

pub fn play<G : Game, P1 : Agent, P2 : Agent>( game : &G, player_1 : &mut P1, player_2 : &mut P2 ) {

        let mut s = game.initial_state();

        while matches!(game.game_status(&s), GameResult::NotFinished) {
            let turn = match game.player_turn(&s) {
                0 => player_1.decide_turn(game, &s),
                1 => player_2.decide_turn(game, &s),
                _ => panic!("unknown player"),
            };

            println!("============================================================");
            println!("Player {} took the follwing action:\n{}\n\n", game.player_turn(&s) + 1, game.display_turn(&turn));

            s = game.take_turn(&s, &turn);
            println!("{}\n\n", game.display_state(&s));
            println!("============================================================");
        }

        match game.game_status(&s) {
            GameResult::NotFinished => panic!("Game not finished"),
            GameResult::Winner{ player, scores: _ } => {
                if player == 0 {
                    println!("Player 1 won");
                }
                else {
                    println!("Player 2 won");
                }
            },
            GameResult::Draw{ scores: _ } => println!("The game was a draw"),
        }
}
