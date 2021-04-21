//! Implementation of the min max algorithm.
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;
use std::fmt;
//use rayon::prelude::*;

/// Min-Max algorithm with a given recursion depth.
pub struct MinMax(pub u8);

fn minmax(state: &Configuration, depth: u8, current: bool) -> (Option<Movement>, i8) {
    if depth == 0 || state.movements().next().is_none() {
        if current {
            (None, state.value())
        }
        else {
            (None, -state.value())
        }
    } else {
        let mut v_opt;
        let mut opt: Option<Movement> = None;
        if current {
            v_opt = 127;
            for m in state.movements() {
                let (_, value) = minmax(&state.play(&m), depth - 1, !current);

                if value < v_opt {
                    v_opt = value;
                    opt = Some(m);
                }
            }
            //println!("{}", v_opt);
        } else {
            v_opt = -128;
            for m in state.movements() {
                let (_, value) = minmax(&state.play(&m), depth - 1, !current);

                if value > v_opt {
                    v_opt = value;
                    opt = Some(m);
                }
            }
        }
        //println!("v_opt = {}", v_opt);
        (opt, v_opt)
    }
}

fn negamax(state: &Configuration, depth: u8) -> (Option<Movement>, i8) {
    if depth == 0 || state.movements().next().is_none() {
        (None, state.value())
    } else {
        let mut v_opt: i8 = 127;
        let mut opt: Option<Movement> = None;
        for m in state.movements() {
            let (_, value) = negamax(&state.play(&m), depth - 1);

            let mut readed_value = -value;

            if readed_value < v_opt {
                v_opt = readed_value;
                opt = Some(m);
            }
        }
        (opt, v_opt)
    }
}

impl Strategy for MinMax {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        println!("Je suis de profondeur {}", self.0);
        let depth = self.0;
        let (opt, _) = minmax(state, depth, true);
        //let (opt, _) = negamax(state, depth);
        opt
        //unimplemented!("TODO: implementer min max")
    }
}



impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Min - Max (max level: {})", self.0)
    }
}

/// Anytime min max algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn min_max_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    for depth in 1..100 {
        movement.store(MinMax(depth).compute_next_move(state));
    }
}
