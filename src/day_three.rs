use std::{error::Error, fs::File, io::{BufReader, BufRead}, collections::HashSet};

pub struct DayThree {
    rucksacks: Vec<String>
}

impl DayThree {
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut rucksacks = Vec::new();

        for line in reader.lines() {
            match line {
                Ok(rucksack) => {
                    rucksacks.push(rucksack);
                },
                Err(e) => return Err(Box::new(e))
            }
        }

        Ok(Self {
            rucksacks
        })
    }
}

// (String, String)

impl DayThree {
    pub fn get_total_double_priority(&self) -> i32 {
        let mut total_priority = 0;

        for rucksack in &self.rucksacks {
            let compartment1 = &rucksack[0..rucksack.len() / 2];
            let compartment2 = &rucksack[rucksack.len() / 2..rucksack.len()];
            
            let mut should_break = false;

            for char1 in compartment1.chars() {
                if should_break {
                    break;
                }

                for char2 in compartment2.chars() {
                    if char1 == char2 {
                        if let Some(priority) = get_char_priority_value(&char1) {
                            total_priority += priority;
                            should_break = true;

                            break;
                        }
                    }
                }
            }
        }

        total_priority
    }

    pub fn get_total_group_priorities(&self) -> i32 {
        let mut rucksacks = self.rucksacks.clone();
        let mut priority_total = 0;

        for elf_group in rucksacks.chunks_mut(3) {
            let rucksack1 = remove_duplicates(elf_group[0].clone());
            let rucksack2 = remove_duplicates(elf_group[1].clone());
            let rucksack3 = remove_duplicates(elf_group[2].clone());

            let mut dup_check = rucksack1.clone();

            dup_check.retain(|c| {
                rucksack2.contains(c) && rucksack3.contains(c)
            });

            assert!(dup_check.len() == 1);

            if let Some(char) = dup_check.chars().next() {
                if let Some(priority) = get_char_priority_value(&char) {
                    priority_total += priority;
                }
            }
        }

        priority_total
    }
}

fn remove_duplicates(mut s: String) -> String {
    let mut seen = HashSet::new();

    s.retain(|c| {
        let is_first = !seen.contains(&c);
        seen.insert(c);
        is_first
    });

    s
}

fn get_char_priority_value(to_check: &char) -> Option<i32> {
    let mut priority = None;

    if to_check.is_ascii_lowercase() {
        priority = Some(*to_check as i32 - 'a' as i32 + 1);
    }

    if to_check.is_ascii_uppercase() {
        priority = Some(*to_check as i32 - 'A' as i32 + 27);        
    }

    priority
}