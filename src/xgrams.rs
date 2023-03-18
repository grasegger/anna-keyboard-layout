use std::{
    collections::{HashMap, VecDeque},
    fs::{read_dir, File},
    iter::Sum,
    path::Path,
};

use rayon::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use utf8_read::Reader;

type XgramMap = HashMap<String, u32>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Xgrams {
    pub x1: XgramMap,
    pub x2: XgramMap,
    pub x3: XgramMap,
    pub x4: XgramMap,
    pub x5: XgramMap,
    stack: VecDeque<char>,
}

impl Default for Xgrams {
    fn default() -> Self {
        Self {
            x1: XgramMap::new(),
            x2: XgramMap::new(),
            x3: XgramMap::new(),
            x4: XgramMap::new(),
            x5: XgramMap::new(),
            stack: VecDeque::new(),
        }
    }
}

impl Sum for Xgrams {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::default(), |a, b| -> Self {
            let mut out = Self::default();

            for x in [a, b] {
                for (k, v) in x.x1 {
                    *out.x1.entry(k).or_insert(0) += v;
                }

                for (k, v) in x.x2 {
                    *out.x2.entry(k).or_insert(0) += v;
                }

                for (k, v) in x.x3 {
                    *out.x3.entry(k).or_insert(0) += v;
                }

                for (k, v) in x.x4 {
                    *out.x4.entry(k).or_insert(0) += v;
                }

                for (k, v) in x.x5 {
                    *out.x5.entry(k).or_insert(0) += v;
                }
            }

            out
        })
    }
}

impl Xgrams {
    pub fn add_char(&mut self, c: char) {
        self.stack.push_front(c);
        if self.stack.len() > 5 {
            self.stack.pop_back();
        };

        self.add_one();
        self.add_two();
        self.add_three();
        self.add_four();
        self.add_five();
    }

    fn add_one(&mut self) {
        let key = format!("{}", self.stack.get(0).unwrap());
        *self.x1.entry(key).or_insert(1) += 1;
    }

    fn add_two(&mut self) {
        if self.stack.len() >= 2 {
            let key = format!(
                "{}{}",
                self.stack.get(0).unwrap(),
                self.stack.get(1).unwrap(),
            );

            *self.x2.entry(key).or_insert(1) += 1;
        }
    }

    fn add_three(&mut self) {
        if self.stack.len() >= 3 {
            let key = format!(
                "{}{}{}",
                self.stack.get(0).unwrap(),
                self.stack.get(1).unwrap(),
                self.stack.get(2).unwrap(),
            );

            *self.x3.entry(key).or_insert(1) += 1;
        }
    }

    fn add_four(&mut self) {
        if self.stack.len() >= 4 {
            let key = format!(
                "{}{}{}{}",
                self.stack.get(0).unwrap(),
                self.stack.get(1).unwrap(),
                self.stack.get(2).unwrap(),
                self.stack.get(3).unwrap(),
            );

            *self.x4.entry(key).or_insert(1) += 1;
        }
    }

    fn add_five(&mut self) {
        if self.stack.len() == 5 {
            let key = format!(
                "{}{}{}{}{}",
                self.stack.get(0).unwrap(),
                self.stack.get(1).unwrap(),
                self.stack.get(2).unwrap(),
                self.stack.get(3).unwrap(),
                self.stack.get(4).unwrap(),
            );

            *self.x5.entry(key).or_insert(1) += 1;
        }
    }
    pub fn read_xgrams(path: String) -> Xgrams {
        let paths = read_dir(&path).unwrap();

        let mut par_path = vec![];

        for path in paths {
            if let Ok(entry) = path {
                par_path.push(entry.path());
            }
        }

        let size = fs_extra::dir::get_size(&path).unwrap().to_string();
        let hash_base = par_path
            .iter()
            .fold("".to_string(), |acc, p| {
                format!("{}{}", acc, p.to_str().unwrap().to_string())
            })
            .to_string()
            + &path
            + &size;

        let checksum = crc32fast::hash(hash_base.as_bytes()).to_string();

        let saved_xgrams = Path::new(&checksum);

        match Path::exists(saved_xgrams) {
            true => {
                println!("Loaded saved xgrams");
                let f = File::open(saved_xgrams).expect("Unable to open file");
                bincode::deserialize_from(f).unwrap()
            }
            false => {
                println!("Parsing new xgrams");
                let xgrams: Xgrams = par_path
                    .par_iter()
                    .map(|path| {
                        let mut xgram = Xgrams::default();
                        let in_file = File::open(path).unwrap();
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
                                xgram.add_char(c);
                            }
                        }
                        xgram
                    })
                    .sum();

                let f = File::create(saved_xgrams).expect("Unable to open file");
                bincode::serialize_into(f, &xgrams).expect("Error writing to file.");
                xgrams
            }
        }
    }
}
