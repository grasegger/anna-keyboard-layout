use std::{collections::HashMap, fs};

use toml::Table;

pub struct Ngrams {
    length: usize,
    chars: Vec<char>,
    pub penalties: Vec<f64>,
}

lazy_static! {
    #[derive(Debug)]
    static ref MOVEMENT : HashMap<String, Table> = {
        let movement_raw = fs::read_to_string("./movement.toml")
        .expect("Unable to read file");
        toml::from_str(&movement_raw).unwrap()
    };
}

impl Ngrams {
    pub fn new(length: usize) -> Self {
        format!("{:?}", MOVEMENT.clone());
        let mut penalties = vec![];
        for _ in 0..length - 1 {
            penalties.push(0.0);
        }
        Self {
            length,
            chars: vec![],
            penalties,
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
        let pos = layout.chars().position(|c| c == char).unwrap().to_string();

        if let Some(last_char) = self.chars.last() {
            let last_char_pos = layout
                .chars()
                .position(|c| c == *last_char)
                .unwrap()
                .to_string();
            if let Some(value) = MOVEMENT.get(&pos) {
                if let Some(penalty) = value.get(&last_char_pos) {
                    

                    if let Some(f_val) = penalty.as_float() {
                        self.penalties.push(f_val);
                    } else {
                        println!("{}", penalty);
                    }
                }
            } else {
                println!("WARN: No movement for index {}", pos);
            }
        }
    }

    pub fn get_penalties(&self) -> f64 {
        self.penalties.clone().into_iter().sum()
    }
}
