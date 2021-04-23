//! Alpha - Beta algorithm.
use std::fmt;

use super::Strategy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;
use std::collections::HashMap;
use rayon::prelude::*;
use std::sync::atomic::{AtomicI8, Ordering};
//use std::sync::atomic::{AtomicI;

/// Anytime alpha beta algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn alpha_beta_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    for depth in 1..100 {
        let chosen_movement = AlphaBeta(depth).compute_next_move(state);
        movement.store(chosen_movement);
    }
}

fn alpha_beta(state: &Configuration, depth: u8, mut alpha: i8, mut beta: i8, current: bool) -> (Option<Movement>, i8) {
    if depth == 0 || state.movements().next().is_none() {
        if current {(None, state.value())}
        else {(None, -state.value())}
    }
    else {
        let mut v_opt: i8;
        let mut opt: Option<Movement> = None;
        if current {
            v_opt = 127;
            for m in state.movements() {
                let (_, value) = alpha_beta(&state.play(&m), depth - 1, alpha, beta, !current);
                if value < v_opt {
                    v_opt = value;
                    opt = Some(m);
                }
                if alpha >= v_opt {
                    break;
                }

                if v_opt < beta {
                    beta = v_opt;
                }
            }
        } else {
            v_opt = -128;
            for m in state.movements() {
                let (_, value) = alpha_beta(&state.play(&m), depth - 1, alpha, beta, !current);

                if value > v_opt {
                    v_opt = value;
                    opt = Some(m);
                }

                if beta <= v_opt {
                    break;
                }

                if v_opt > alpha {
                    alpha = v_opt;
                }
            }
        }
        (opt, v_opt)
    }
}



fn negamax(state: &Configuration, depth: u8, mut alpha: i8, mut beta: i8) -> (Option<Movement>, i8) {
    if depth == 0 || state.movements().next().is_none() {
        (None, state.value())
    } else {
        let mut v_opt: i8 = 127;
        let mut opt: Option<Movement> = None;
        for m in state.movements() {
            let (_, value) = negamax(&state.play(&m), depth - 1, -beta, -alpha);

            let mut readed_value = -value;

            if readed_value < v_opt {
                v_opt = readed_value;
                opt = Some(m);
            }

            if readed_value < beta {
                beta = readed_value;
            }

            if alpha >= beta {
                break;
            }
        }
        (opt, v_opt)
    }
}
/*Young Brothers Wait Concept*/
// fn negamax_par(state: &Configuration, depth: u8, alpha: &AtomicI8, beta: &AtomicI8) -> (Option<Movement>, i8) {
    // if depth == 0 || state.movements().next().is_none() {
    //     (None, state.value())
    // } else {
    //     let v_opt: AtomicI8 = AtomicI8::new(127);
    //     let mut opt: AtomicMove = AtomicMove::new().unwrap();

    //     /*Premier mouvement en séquentiel*/
    //     let mut iter_m = state.movements();
    //     let mut mov = iter_m.next();
    //     let (_, value) = negamax_par(&state.play(&mov.unwrap()), depth - 1, &(AtomicI8::new(- beta.load(Ordering::SeqCst))), &(AtomicI8::new(- alpha.load(Ordering::SeqCst))));

    //     let readed_value = -value;

    //     if readed_value < v_opt.load(Ordering::SeqCst) {
    //         v_opt.store(readed_value, Ordering::SeqCst);
    //         opt.store(mov);
    //     }

    //     if readed_value < beta.load(Ordering::SeqCst) {
    //         beta.store(readed_value,Ordering::SeqCst);
    //     }

    //     /*Suite en parallèle*/

    //     let par_movements = iter_m
    //                         .par_bridge()
    //                         .try_for_each(|mov|
    //                             negamax_par_aux(state, depth, &alpha, &beta, mov, &v_opt, &opt)
    //                         );


    //     (opt.load(), v_opt.load(Ordering::SeqCst))
    // }
// }

// fn negamax_par_aux(state: &Configuration, depth: u8, mut alpha: &AtomicI8, mut beta: &AtomicI8, mov: Movement, mut v_opt: &AtomicI8, mut opt: &AtomicMove) {
//     let (_, val) = negamax_par(&state.play(mov), depth - 1, AtomicI8::new(- beta.load(Ordering::SeqCst)), AtomicI8::new(- alpha.load(Ordering::SeqCst)));

//     let mut read_value = -val;

//     if read_value < v_opt.load(Ordering::SeqCst) {
//         v_opt.store(read_value, Ordering::SeqCst);
//         opt.store(Some(mov));
//     }

//     if read_value < beta.load(Ordering::SeqCst) {
//         beta.store(read_value,Ordering::SeqCst);
//     }

//     if alpha.load(Ordering::SeqCst) >= beta.load(Ordering::SeqCst) {
//         return ();
//     }
// }

fn negamax_mem(state: &Configuration, saving_states: &mut HashMap<String, (Option<Movement>, i8)>, depth: u8, mut alpha: i8, mut beta: i8) -> (Option<Movement>, i8) {
    if depth == 0 || state.movements().next().is_none() {
        (None, state.value())
    } else {
        let v = saving_states.get(&state.serialize());
        let res: (Option<Movement>, i8) = match v {
            Some(x) => *x,
            None => {
                let mut v_opt: i8 = 127;
                let mut opt: Option<Movement> = None;

                for m in state.movements() {
                    let (_, value) = negamax_mem(&state.play(&m), saving_states, depth - 1, -beta, -alpha);

                    let mut readed_value = -value;
                    saving_states.insert(state.serialize(), (Some(m), readed_value));

                    if readed_value < v_opt {
                        v_opt = readed_value;
                        opt = Some(m);
                    }

                    if readed_value < beta {
                        beta = readed_value;
                    }

                    if alpha >= beta {
                        //println!("COUPURE");
                        break;
                    }
                }
                (opt, v_opt)
            },
        };
        res
    }
}

fn pvs(state: &Configuration, depth: u8, mut alpha: i8, mut beta: i8) -> (Option<Movement>, i8) {
    if depth == 0 || state.movements().next().is_none() {
        (None, state.value())
    } else {
        let mut v_opt: i8 = 127;
        let mut opt: Option<Movement> = None;

        let mut iter_m = state.movements();
        let mut mov = iter_m.next();
        let (_, value) = pvs(&state.play(&mov.unwrap()), depth - 1, -beta, -alpha);

        let mut readed_value = -value;

        if readed_value < v_opt {
            v_opt = readed_value;
            opt = Some(mov.unwrap());
        }

        if readed_value < beta {
            beta = readed_value;
        }

        for m in iter_m {
            let (_, value) = pvs(&state.play(&m), depth - 1, -alpha - 1, -alpha);

            let mut readed_value = -value;

            if readed_value < v_opt {
                v_opt = readed_value;
                opt = Some(m);
            }

            if alpha < readed_value && readed_value < beta {
                let (_, value) = pvs(&state.play(&m), depth - 1, -beta, -readed_value);

                let mut readed_value = -value;

                if readed_value < v_opt {
                    v_opt = readed_value;
                    opt = Some(m);
                }
            }

            if readed_value < beta {
                beta = readed_value;
            }

            if alpha >= beta {
                break;
            }

        }

        (opt, v_opt)
    }
}



/// Alpha - Beta algorithm with given maximum number of recursions.
pub struct AlphaBeta(pub u8);

impl fmt::Display for AlphaBeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Alpha - Beta (max level: {})", self.0)
    }
}

impl Strategy for AlphaBeta {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        let depth = self.0;

        /*Minmax alpha-beta*/
        /*let alpha = -127;
        let beta = 127;*/
        //let (opt, _) = alpha_beta(state, depth, alpha, beta, true, cmp_coup);
        //println!("cmp_coup = {}", cmp_coup);

        /*Negamin alpha-beta*/
        let alpha = -127;
        let beta = 127;
        let (opt, _) = negamax(state, depth, alpha, beta);

        /*Negamin parallèle alpha-beta*/
        // let alpha = AtomicI8::new(-127);
        // let beta = AtomicI8::new(127);
        // let (opt, _) = negamax_par(state, depth, &alpha, &beta);

        /*Negamax mem*/
        //let mut saving_states: HashMap<String, (Option<Movement>, i8)> = HashMap::new();
        //let (opt, _) = negamax_mem(state, &mut saving_states, depth, alpha, beta);

        /*pvs*/
        //let mut cmp_coup = 0;
        //let alpha = 65;
        //let beta = -65;
        //let (opt, _) = pvs(state, depth, alpha, beta);
        //println!("cmp_coup = {}", cmp_coup);

        /*Negamax alpha-beta par*/
        /*let alpha = 127;
        let beta = -127;
        let (opt, _) = negamax(state, depth, alpha, beta);*/
        //println!("alpha = {}, beta = {}", alpha, beta);

        opt
    }
}
