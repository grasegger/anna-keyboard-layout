use std::{collections::HashMap, fs, process};

use toml::Table;

pub struct Ngrams {
    length: usize,
    chars: Vec<char>,
    pub penalties: Vec<f64>,
    movement: HashMap<String, Table>,
    counter: u128,
}

impl Ngrams {
    pub fn new(length: usize) -> Self {
        let mut penalties = vec![];
        for _ in 0..length - 1 {
            penalties.push(0.0);
        }

        let movement_raw = fs::read_to_string("./movement.toml").expect("Unable to read file");
        let movement = toml::from_str(&movement_raw).unwrap();
        Self {
            length,
            chars: vec![],
            penalties,
            movement,
            counter: 0,
        }
    }

    // Returns the penalty for the added char
    pub fn push(&mut self, c: char, layout: &str) {
        if self.is_filled() {
            self.chars.remove(0);
        }

        self.push_penalty(c, layout);
        self.chars.push(c);
    }

    pub fn is_filled(&self) -> bool {
        self.chars.len() >= self.length
    }

    fn push_penalty(&mut self, char: char, layout: &str) {
        self.counter += 1;

        let mut lchars = layout.chars();
        let pos = lchars.position(|c| c == char);

        match pos {
            Some(pos) => {
                let pos = pos.to_string();
                let last = self.chars.last();
                if let Some(last_char) = last {
                    if let Some(last_char_pos) = lchars.position(|c| c == *last_char) {
                        let last_char_pos = last_char_pos.to_string();
                        if let Some(value) = self.movement.get(&pos) {
                            if let Some(penalty) = value.get(&last_char_pos) {
                                if let Some(f_val) = penalty.as_float() {
                                    self.penalties.push(f_val);
                                } else {
                                    println!("{}", penalty);
                                }
                            }
                        }
                    }
                } else if self.counter > 1 {
                    println!(
                        "{} | Cant find last {} in {}, tried {:?} | {:?}",
                        process::id(),
                        char,
                        layout,
                        last,
                        self.counter
                    );
                }
            }
            None => (),
        }
    }

    pub fn get_penalties(&self) -> f64 {
        self.penalties.clone().into_iter().sum()
    }
}
