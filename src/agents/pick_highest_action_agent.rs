
use crate::games::game_form::{Game, HeuristicDescription};
use super::agent::Agent; 

pub fn get_agent( heuristic_description : HeuristicDescription, player : u32 ) -> impl Agent {
    PickHighestActionAgent { heuristic_description, player }
}

struct PickHighestActionAgent {
    heuristic_description : HeuristicDescription,
    player : u32
}

impl Agent for PickHighestActionAgent {
    fn decide_turn<G : Game>(&mut self, game : &G, state : &G::State) -> G::TurnAction {
        let (_, heuristic) = game.heuristics().into_iter().find(|(hd, _)| *hd == self.heuristic_description ).unwrap();
        let highest_action = game.legal_turns(&state)
                          .map(|turn| { 
                                let new_state = game.take_turn(&state, &turn);
                                let score = game.state_score(&new_state, &heuristic, self.player);
                                (score, Some(turn))
                          })
                          .fold((-1, None), |highest, n| {
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
