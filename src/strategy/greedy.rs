//! Dumb greedy algorithm.
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use std::fmt;

/// Dumb algorithm.
/// Amongst all possible movements return the one which yields the configuration with the best
/// immediate value.
pub struct Greedy();

impl fmt::Display for Greedy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Greedy")
    }
}

impl Strategy for Greedy {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        /*Pourquoi ce code marche pas?*/
        /*let i_min = state.movements()
                        .map(|m| state.play(&m).skip_play().value())
                        .collect::<Vec<i8>>()
                        .iter()
                        .enumerate()
                        .min_by(|(_, a), (_, b)| a.cmp(b))
                        .map(|(i, _)| i);

        let opt = state.movements()
                    .take(i_min.unwrap())
                    .last();*/

        /*Since the board is 8*8, v_opt is bounded by 64*/
        let mut v_opt: i8 = 64;
        let mut opt: Option<Movement> = None;
        for m in state.movements() {
            let v = state.play(&m).skip_play().value();
            if v < v_opt { v_opt = v; opt = Some(m); }
            //println!("valeur = {}", state.value());
        }
        opt

        //unimplemented!("TODO: algo glouton")

    }
}
