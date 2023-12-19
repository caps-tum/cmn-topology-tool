use core_affinity::CoreId;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Barrier};
use std::time::Instant;

pub struct State {
    barrier: Barrier,
    flag: AtomicBool,
}

impl State {
    pub fn new() -> Self {
        Self {
            barrier: Barrier::new(2),
            flag: AtomicBool::new(PING),
        }
    }
}

const PING: bool = false;
const PONG: bool = true;
pub fn run( ping_core: CoreId,
        pong_core: CoreId,
        num_round_trips: u32,
        num_samples: u32 ) -> u128 {
    let state = State::new();
    let r_state = &state;

    crossbeam_utils::thread::scope(|s| {

        let pong = s.spawn(move |_| {
            core_affinity::set_for_current(pong_core);
            r_state.barrier.wait();
            for _ in 0..(num_round_trips*num_samples) {
                while r_state.flag.compare_exchange(PING, PONG, Ordering::Relaxed, Ordering::Relaxed).is_err() {}
            }
        });

        let ping = s.spawn(move |_| {
            core_affinity::set_for_current(ping_core);
            r_state.barrier.wait();

            let start = Instant::now(); // this data will never be used, so use the lower-precision Instant call
            for _ in 0..num_samples {
                for _ in 0..num_round_trips {
                    while r_state.flag.compare_exchange(PONG, PING, Ordering::Relaxed, Ordering::Relaxed).is_err() {}
                }
            }
            let end = Instant::now();
            (end-start).as_micros()
        });

        pong.join().unwrap();
        ping.join().unwrap()
    }).unwrap()
}