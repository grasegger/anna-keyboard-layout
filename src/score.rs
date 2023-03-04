use ngram::Ngrams;
use std::{fmt, fs};

use utf8_read::Reader;

use crate::ngram;

lazy_static! {
    static ref EFFORT: Vec<f64> = {
        let effort_raw = fs::read_to_string("./effort.json").expect("Unable to read file");
        serde_json::from_str(&effort_raw).unwrap()
    };
}

pub struct Score {
    pub effort: f64,
    pub perc_left: f64,
    pub perc_right: f64,
    pub penalty: f64,
    pub layout: String,
    pub left: String,
    pub right: String,
}

impl Score {
    pub fn calculate(layout: &str) -> Self {
        let layout = layout.to_lowercase();
        let mut chs = layout.chars().into_iter();
        let mut left = "".to_string();
        let mut right = "".to_string();

        for _ in 0..3 {
            for _ in 0..5 {
                let char = chs.next().unwrap();
                let new = format!("{}{}", left, char);
                left = new;
            }

            for _ in 0..5 {
                let char = chs.next().unwrap();
                let new = format!("{}{}", right, char);
                right = new;
            }
        }

        let char = chs.next().unwrap();
        let new = format!("{}{}", left, char);
        left = new;

        let char = chs.next().unwrap();
        let new = format!("{}{}", right, char);
        right = new;

        let mut scores = Self::calculate_scores(&layout, &left, &right).into_iter();
        Self {
            effort: scores.next().unwrap(),
            perc_left: scores.next().unwrap(),
            perc_right: scores.next().unwrap(),
            penalty: scores.next().unwrap(),
            layout,
            left: left.into(),
            right: right.into(),
        }
    }

    fn calculate_scores(layout: &str, left_side: &str, _: &str) -> Vec<f64> {
        let mut effort = 0.0;
        let mut left = 0.0;
        let mut right = 0.0;
        let mut ngrams = Ngrams::new(3);

        let paths = fs::read_dir("./texts").unwrap();

        for path in paths {
            match path {
                Ok(entry) => {
                    let in_file = std::fs::File::open(entry.path()).unwrap();
                    let mut reader = Reader::new(&in_file);
                    for x in reader.into_iter() {
                        let mut char = x.unwrap();
                        let mut clean_char_stack = vec![];

                        if char == ' ' {
                            char = '_';
                        } else if !char.is_alphanumeric() || char.is_numeric() {
                            continue;
                        }

                        if char.is_uppercase() || char.is_ascii_uppercase() {
                            clean_char_stack.push('#');

                            for tc in char.to_lowercase().into_iter() {
                                clean_char_stack.push(tc);
                            }
                        } else {
                            clean_char_stack.push(char);
                        }

                        for c in clean_char_stack {
                            effort += Self::get_score_for_char(layout, c);
                            left += Self::get_left_for_char(left_side, c);
                            right += Self::get_right_for_char(left_side, c);
                            ngrams.push(c, &layout);
                        }
                    }
                }
                Err(_) => (),
            }
        }

        let perc_left = (100.0 / (left + right)) * left;
        let perc_right = (100.0 / (left + right)) * right;

        vec![
            effort,
            perc_left as f64,
            perc_right as f64,
            ngrams.get_penalties(),
        ]
    }

    fn get_left_for_char(left: &str, char: char) -> f64 {
        let pos = left.chars().position(|c| c == char);
        match pos {
            Some(_) => 1.0,
            None => 0.0,
        }
    }

    fn get_right_for_char(left: &str, char: char) -> f64 {
        if Self::get_left_for_char(left, char) == 0.0 {
            1.0
        } else {
            0.0
        }
    }

    fn get_score_for_char(layout: &str, char: char) -> f64 {
        let pos = layout.chars().position(|c| c == char);

        match pos {
            Some(position) => EFFORT[position],
            None => 0.0,
        }
    }
}

impl std::fmt::Debug for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Score")
            .field("effort", &self.effort)
            .field("layout", &self.layout)
            .field("left", &self.left)
            .field("right", &self.right)
            .finish()
    }
}

impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut lyt = self.layout.chars().into_iter();
        let mut res = "".to_string();

        for _ in 0..10 {
            let char = lyt.next().unwrap();
            let new = format!("{}{}", res, char);
            res = new;
        }

        res = format!("{} | effort: {:.2}\n", res, self.effort);

        for _ in 0..10 {
            let char = lyt.next().unwrap();
            let new = format!("{}{}", res, char);
            res = new;
        }

        res = format!("{} | movement: {:.2}\n", res, self.penalty);

        for _ in 0..10 {
            let char = lyt.next().unwrap();
            let new = format!("{}{}", res, char);
            res = new;
        }

        res = format!(
            "{} | left/right: {:.2}% / {:.2}%\n",
            res, self.perc_left, self.perc_right
        );

        res = format!("{}    ", res);
        for _ in 0..2 {
            let char = lyt.next().unwrap();
            let new = format!("{}{}", res, char);
            res = new;
        }

        write!(f, "{}     |", res)
    }
}
