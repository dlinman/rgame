

use crate::agents::agent::{Agent};
use crate::game_form::{Game, GameResult};

pub fn contest<G : Game, P1 : Agent, P2 : Agent>( game : &G, player_1 : &mut P1, player_2 : &mut P2, game_count : u32 ) {

    let mut p1_win = 0;
    let mut p2_win = 0;
    let mut draw = 0;
    for _ in 0..game_count {

        let mut s = game.initial_state();

        while matches!(game.game_status(&s), GameResult::NotFinished) {
            let turn = match game.player_turn(&s) {
                0 => player_1.decide_turn(game, &s),
                1 => player_2.decide_turn(game, &s),
                _ => panic!("unknown player"),
            };

            s = game.take_turn(&s, &turn);
        }

        match game.game_status(&s) {
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
    }

    println!("Player 1 wins = {}; player 2 wins = {}, draws = {}", p1_win, p2_win, draw);
}