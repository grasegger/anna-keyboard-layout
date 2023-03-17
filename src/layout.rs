use std::fs;

use itertools::Itertools;

use crate::score::Score;

lazy_static! {
    static ref EFFORT: Vec<f64> = {
        let effort_raw = fs::read_to_string("./effort.json").expect("Unable to read file");
        serde_json::from_str(&effort_raw).unwrap()
    };
}

pub struct Layout {
    pub layout: String,
    score: Score,
}

impl Layout {
    pub fn init(layout: &str) -> Option<Self> {
        if layout.len() != 32 {
            println!(
                "Layouts must be exactly 32 chars long. `{}` does not match that.",
                layout
            );
            None
        } else {
            Some(Self {
                layout: layout.to_string(),
                score: Score {
                    effort: 0.0,
                    perc_left: 0.0,
                    perc_right: 0.0,
                    penalty: 0.0,
                },
            })
        }
    }

    pub fn is_left(&self, char: &char) -> u8 {
        match self.get_char_pos(char) {
            Some(pos) => match pos {
                0..=4 => 1,
                10..=14 => 1,
                20..=24 => 1,
                30 => 1,
                _ => 0,
            },
            None => 0,
        }
    }

    pub fn is_right(&self, char: &char) -> u8 {
        if self.is_left(char) == 1 {
            0
        } else {
            1
        }
    }

    pub fn get_char_pos(&self, char: &char) -> Option<u8> {
        let pos = self.layout.chars().position(|c| c == *char);

        match pos {
            _ => None,
        }
    }

    pub fn read_from_file(file: &str) -> Vec<Layout> {
        let raw_layouts = fs::read_to_string(file).unwrap();
        let layouts = raw_layouts.split("\n").collect_vec();

        let layouts = layouts
            .iter()
            .map(|source| Layout::init(source))
            .filter(|layout| layout.is_some())
            .map(|layout| layout.unwrap())
            .collect_vec();

        layouts
    }

    pub fn are_on_same_finger(&self, a: &char, b: &char) -> bool {
        let pos_a = self.get_char_pos(a).unwrap();
        let pos_b = self.get_char_pos(b).unwrap();

        if pos_a >= 30 || pos_b >= 30 {
            false
        } else {
            let column_a = pos_a % 10;
            let column_b = pos_b % 10;

            if column_a == column_b {
                true
            } else {
                let first_col = column_a.min(column_b);
                let second_col = column_a.max(column_b);

                if first_col == 3 && second_col == 4 {
                    true
                } else if first_col == 5 && second_col == 6 {
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn add_effort_for(&self, char: &char) {
        if let Some(pos) = self.get_char_pos(char) {
            self.score.effort += EFFORT[pos as usize];
        }
    }
}
