use std::{fs::File, error::Error, io::{BufReader, BufRead}, ops::RangeInclusive};

#[derive(Clone, Copy, PartialEq)]
struct Pair {
    elf_a: (i32, i32),
    elf_b: (i32, i32)
}

pub struct DayFour {
    pairs: Vec<Pair>
}

impl DayFour {
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut pairs = Vec::new();

        for line in reader.lines() {
            match line {
                Ok(pair) => {
                    if !pair.is_empty() {
                        let split_pair: Vec<&str> = pair.split(',').collect();

                        let pair_a_range: Vec<&str> = split_pair[0].split('-').collect();
                        let elf_a = (pair_a_range[0].parse::<i32>()?, pair_a_range[1].parse::<i32>()?);

                        let pair_b_range: Vec<&str> = split_pair[1].split('-').collect();
                        let elf_b = (pair_b_range[0].parse::<i32>()?, pair_b_range[1].parse::<i32>()?);

                        pairs.push(Pair {
                            elf_a,
                            elf_b
                        });
                    }
                },
                Err(e) => return Err(Box::new(e))
            }
        }

        Ok(Self {
            pairs
        })
    }

    pub fn get_pairs_that_contains_the_other(&self) -> i32 {
        let mut total = 0;

        for pair in &self.pairs {
            let elf_a = pair.elf_a;
            let elf_b = pair.elf_b;

            if is_in_range(elf_a, elf_b) {
                total += 1;
            }
        }

        total
    }

    pub fn get_total_overlapping(&self) -> i32 {
        let mut overlapping = 0;

        for pair in &self.pairs {
            if pairs_contain_any(pair.elf_a, pair.elf_b) {
                overlapping += 1;
            }
        }

        overlapping
    }
}

fn is_in_range(pair_a: (i32, i32), pair_b: (i32, i32)) -> bool {
    (pair_a.0 >= pair_b.0 && pair_a.1 <= pair_b.1) ||
    (pair_b.0 >= pair_a.0 && pair_b.1 <= pair_a.1)
}

fn pairs_contain_any(pair_a: (i32, i32), pair_b: (i32, i32)) -> bool {
    let mut range_a = pair_a.0..=pair_a.1;
    let range_b = pair_b.0..=pair_b.1;

    range_a.any(|num| range_b.contains(&num))
}