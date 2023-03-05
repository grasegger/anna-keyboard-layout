#[macro_use]
extern crate lazy_static;

use std::fs;

use itertools::Itertools;
use rayon::prelude::*;
use score::Score;

mod ngram;
mod score;

pub fn main() {
    let scores = read_and_calc_layouts();

    for score in scores {
        println!("{}\n", score);
    }
}

pub fn read_and_calc_layouts() -> Vec<Score> {
    let raw_layouts = fs::read_to_string("layouts.list").unwrap();
    let layouts: Vec<&str> = raw_layouts.split("\n").collect_vec();

    layouts
        .par_iter()
        .map(|layout| Score::calculate(layout))
        .collect()
}
