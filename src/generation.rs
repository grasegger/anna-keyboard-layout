use std::fs::read_to_string;

use chrono::Local;
use once_cell::sync::Lazy;
use rand::thread_rng;
use rayon::prelude::*;

use crate::{layout::Layout, xgrams::Xgrams};
use itertools::Itertools;

use rand::seq::SliceRandom;

type Generation = Vec<Layout>;

#[cfg(debug_assertions)]
static MULT: usize = 100;

#[cfg(not(debug_assertions))]
static MULT: usize = 10_000;

static FULLGEN: Lazy<usize> = Lazy::new(|| {
    let default_parallelism_approx = std::thread::available_parallelism().unwrap().get();
    println!("Got parallelism of {}", default_parallelism_approx);
    default_parallelism_approx * MULT
});

pub fn get_initial_layouts() -> Generation {
    let raw_layouts = read_to_string("layouts.list").unwrap();
    let layouts = raw_layouts.split("\n").map(|raw| raw.into()).collect_vec();

    if layouts.len() > 10 {
        panic!("You are only allowed to provide 10 initial layouts.")
    } else {
        layouts
    }
}

pub fn grow_generation(old: &mut Generation) {
    let start_now = Local::now();
    let upto = *FULLGEN;
    println!("Starting to grow up to {}", upto);
    if old.len() < upto / 10 {
        fill_random(old, upto);
    } else {
        fill_random(old, upto);
    }
    let timeframe = Local::now() - start_now;
    println!(
        "Finished growing to {} after {} seconds.",
        upto,
        timeframe.num_seconds()
    );
}

fn fill_random(old: &mut Generation, upto: usize) {
    let diff = upto - old.len();
    let mut randoms: Generation = (0..diff)
        .into_par_iter()
        .map(|_n| -> Layout {
            let mut base = "abcdefghijklmnopqrstuvwxyzäöüß_#"
                .chars()
                .into_iter()
                .collect_vec();
            base.shuffle(&mut thread_rng());
            let layout: Layout = base.into_iter().collect::<String>().into();
            layout
        })
        .collect();
    old.append(&mut randoms);
}

pub fn score_generation(generation: &mut Generation, xgrams: &Xgrams) {
    let start_now = Local::now();
    println!("Starting to score");

    generation.par_iter_mut().for_each(|layout| {
        layout.score(xgrams);
    });

    for h in 0..generation.len() {
        let hunter = generation.get(h).unwrap().to_owned();
        generation.par_iter_mut().for_each(|gatherer| {
            gatherer.defend(&hunter);
        })
    }

    let timeframe = Local::now() - start_now;
    println!(
        "Finished scoring after {} seconds.",
        timeframe.num_seconds()
    );
}

pub fn shrink_generation(generation: &mut Generation) {
    let keep = generation.len() - *FULLGEN / 10;
    generation.sort();

    generation.drain(..keep);
}
