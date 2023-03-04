#[macro_use]
extern crate lazy_static;

use std::fs;

use itertools::Itertools;
use rayon::prelude::*;
use score::Score;

mod ngram;
mod score;

pub fn main() {
    let raw_layouts = fs::read_to_string("layouts.list").unwrap();
    let layouts: Vec<&str> = raw_layouts.split("\n").collect_vec();

    let scores: Vec<Score> = layouts
        .par_iter()
        .map(|layout| Score::calculate(layout))
        .collect();

    for score in scores {
        println!("{}\n", score);
    }
}
