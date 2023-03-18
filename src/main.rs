use chrono::prelude::*;
use clap::Parser;
use xgrams::Xgrams;

use crate::generation::{
    get_initial_layouts, grow_generation, score_generation, shrink_generation,
};

mod generation;
mod layout;
mod score;
mod xgrams;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory with text files
    #[arg(short, long, default_value = "texts")]
    path: String,

    /// How many generations should the program work on?
    #[arg(short, long, default_value_t = 1)]
    count: u8,

    /// How many layouts should the program print in the end?
    #[arg(short, long, default_value_t = 10)]
    targets: usize,
}

pub fn main() {
    let args = Args::parse();
    let start_now = Local::now();
    println!("Starting to read files into xgrams. This can take a while ...");
    let xgrams = Xgrams::read_xgrams(args.path);
    let timeframe = Local::now() - start_now;
    println!(
        "Finished reading files into xgrams after {} seconds.",
        timeframe.num_seconds()
    );
    let mut generation = get_initial_layouts();

    let mut current_gen = 0;

    while current_gen < args.count {
        let start_now = Local::now();
        println!(
            "Starting to grow, score and shrink generation {} of {}",
            current_gen + 1,
            args.count
        );

        grow_generation(&mut generation);
        score_generation(&mut generation, &xgrams);
        shrink_generation(&mut generation);

        current_gen += 1;
        let timeframe = Local::now() - start_now;
        println!(
            "Finished generation {} after {} seconds.",
            current_gen,
            timeframe.num_seconds()
        );
    }

    generation.sort();

    if args.targets < generation.len() {
        let sorted = generation.iter().rev().take(args.targets);

        for layout in sorted {
            println!("{}", layout);
        }
    } else {
        for layout in generation {
            println!("{}", layout);
        }
    }
}
