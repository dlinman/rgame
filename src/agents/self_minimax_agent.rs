
use std::cmp;

use crate::games::game_form::{Game, GameResult};
use super::agent::Agent; 

pub fn get_agent( player : u32, depth : u32 ) -> impl Agent {
    SelfMinimaxAgent { player, depth }
}

struct SelfMinimaxAgent {
    player : u32,
    depth : u32,
}

impl Agent for SelfMinimaxAgent {
    fn decide_turn<G : Game>(&mut self, game : &G, state : &G::State) -> G::TurnAction {
        // TODO extend to handle more than 2 player games
        fn minimax<G : Game>( game : &G, state : &G::State, player : u32, depth : u32, max : bool ) -> i32 {
            if depth == 0 || !matches!(game.game_status(state), GameResult::NotFinished) {
                return game.state_score(state, player);
            }
            if max {
                let mut value = std::i32::MIN;
                for new_state in game.legal_turns(&state).map(|turn| game.take_turn(&state, &turn)) {
                    value = cmp::max(value, minimax(game, &new_state, player, depth - 1, false));
                }
                value
            } 
            else {
                let mut value = std::i32::MAX;
                for new_state in game.legal_turns(&state).map(|turn| game.take_turn(&state, &turn)) {
                    value = cmp::min(value, minimax(game, &new_state, player, depth - 1, true));
                }
                value
            }
        }
        let highest_action = game.legal_turns(&state)
                          .map(|turn| { 
                                let new_state = game.take_turn(&state, &turn);
                                let score = minimax(game, &new_state, self.player, self.depth, false);
                                (score, Some(turn))
                          })
                          .fold((std::i32::MIN, None), |highest, n| {
                              if matches!(highest, (_, None)) {
                                  return n;
                              }
                              let (cur_v, cur_t) = highest;
                              let (n_v, n_t) = n;
                              if cur_v > n_v {
                                  (cur_v, cur_t) 
                              }
                              else {
                                  (n_v, n_t) 
                              }
                          });

        let (_, turn) = highest_action;
        turn.unwrap()
    }
}
