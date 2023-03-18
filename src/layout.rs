use std::{collections::HashMap, fmt::Display, fs::read_to_string};

use itertools::Itertools;
use once_cell::sync::Lazy;

use crate::{score::Score, xgrams::Xgrams};

static EFFORT: Lazy<Vec<f32>> = Lazy::new(|| {
    let effort_raw = read_to_string("./effort.json").expect("Unable to read file");
    serde_json::from_str(&effort_raw).unwrap()
});

static MOVEMENT: Lazy<HashMap<String, HashMap<String, f32>>> = Lazy::new(|| {
    let movement_raw = read_to_string("./movement.toml").expect("Unable to read file");
    let movement = toml::from_str(&movement_raw).unwrap();
    movement
});

#[derive(Debug, Clone)]
pub struct Layout {
    pub line: String,
    pub score: Score,
    left: String,
}

impl Layout {
    pub fn score(&mut self, xgrams: &Xgrams) {
        for (char, times) in &xgrams.x1 {
            let char = Self::convert_len_1_string_to_char(char.to_string());
            if let Some(char) = char {
                self.score.x1 += *times as f32 * self.get_score_for_char(char);

                self.score.counter_left += self.get_left_for_char(char) as u128 * *times as u128;
                self.score.counter_right += self.get_right_for_char(char) as u128 * *times as u128;
            }
        }

        for (chars, times) in &xgrams.x2 {
            let no_dups = chars.chars().into_iter().unique().collect_vec().len() > 1;

            if no_dups {
                self.score.counter_left +=
                    self.is_all_left(chars.to_string()) as u128 * *times as u128;
                self.score.counter_right +=
                    self.is_all_right(chars.to_string()) as u128 * *times as u128;

                let positions: Vec<usize> = chars
                    .chars()
                    .into_iter()
                    .map(|char| self.line.chars().position(|c| c == char))
                    .filter(|pos| pos.is_some())
                    .map(|pos| pos.unwrap())
                    .collect_vec();

                if let Some((first, rest)) = positions.split_first() {
                    for next in rest {
                        let next = next.to_string();
                        let first = first.to_string();
                        if let Some(value) = MOVEMENT.get(&first) {
                            if let Some(penalty) = value.get(&next) {
                                self.score.x2 = *times as f32 * penalty;
                            }
                        }
                    }
                }
            }
        }
    }

    fn is_all_left(&self, chars: String) -> u8 {
        let count: u8 = chars.chars().map(|c| self.get_left_for_char(c)).sum();
        match count == chars.len() as u8 {
            true => 1,
            false => 0,
        }
    }

    fn is_all_right(&self, chars: String) -> u8 {
        let count: u8 = chars.chars().map(|c| self.get_right_for_char(c)).sum();
        match count == chars.len() as u8 {
            true => 1,
            false => 0,
        }
    }

    fn get_score_for_char(&self, char: char) -> f32 {
        let pos = self.line.chars().position(|c| c == char);

        match pos {
            Some(position) => EFFORT[position],
            None => 1.0,
        }
    }

    fn convert_len_1_string_to_char(source: String) -> Option<char> {
        if source.len() > 1 {
            None
        } else {
            Some(*source.chars().into_iter().collect_vec().first().unwrap())
        }
    }

    fn get_left_for_char(&self, char: char) -> u8 {
        let pos = self.left.chars().position(|c| c == char);
        match pos {
            Some(_) => 1,
            None => 0,
        }
    }

    fn get_right_for_char(&self, char: char) -> u8 {
        if Self::get_left_for_char(self, char) == 0 {
            1
        } else {
            0
        }
    }

    pub(crate) fn defend(&mut self, hunter: &Layout) {
        let my = self.score;
        let their = hunter.score;

        if my.x1 < their.x1 {
            self.score.points += 1;
        }

        if my.x2 < their.x2 {
            self.score.points += 1;
        }

        if my.x3 < their.x3 {
            self.score.points += 1;
        }

        if my.x4 < their.x4 {
            self.score.points += 1;
        }

        if my.x5 < their.x5 {
            self.score.points += 1;
        }

        let my_sum = my.counter_left + my.counter_right;
        let my_perc = my.counter_left as f64 / my_sum as f64 * 100.0;
        let their_sum = their.counter_left + my.counter_right;
        let their_perc = their.counter_left as f64 / their_sum as f64 * 100.0;

        if (50.0 - my_perc) < (50.0 - their_perc) {
            self.score.points += 1;
        }
    }
}

impl From<&str> for Layout {
    fn from(line: &str) -> Self {
        Self::from(line.to_string())
    }
}

impl From<&String> for Layout {
    fn from(line: &String) -> Self {
        Self::from(line.to_string())
    }
}

impl From<String> for Layout {
    fn from(line: String) -> Self {
        let mut chs = line.chars().into_iter();
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
        Self {
            line,
            score: Score::default(),
            left,
        }
    }
}

impl Eq for Layout {}

impl PartialEq for Layout {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line && self.score == other.score
    }
}

impl Ord for Layout {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for Layout {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl Display for Layout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = "#####################\n\n".to_string();

        let mut layout = self.line.chars();

        for _x in 0..3 {
            for _y in 0..10 {
                let char = layout.next().unwrap();
                res = format!("{} {}", res, char);
            }
            res = format!("{}\n", res);
        }

        res = format!("{}        ", res);
        for _ in 0..2 {
            let char = layout.next().unwrap();
            res = format!("{} {}", res, char);
        }

        res = format!("{}\n\nPoints: {}", res, self.score.points);
        res = format!("{}\n---------------------", res);
        res = format!("{}\nEffort: {}", res, self.score.x1);
        res = format!("{}\nBigrams: {}", res, self.score.x2);
        res = format!("{}\nTrigrams: {}", res, self.score.x3);
        res = format!("{}\n4-grams: {}", res, self.score.x4);
        res = format!("{}\n5-grams: {}", res, self.score.x5);
        res = format!("{}\nleft: {}", res, self.score.counter_left);
        res = format!("{}\nright: {}", res, self.score.counter_right);
        res = format!("{}\n#####################\n", res);

        write!(f, "{}", res,)
    }
}
