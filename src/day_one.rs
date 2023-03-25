use std::{fs::File, error::Error, io::{BufReader, BufRead}};

// normally id probably keep the original order or
// make it possible to edit the original list l8r if needed, lol

pub struct DayOne {
    calorie_data: Vec<u32>
}

impl DayOne {
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut calorie_data = Vec::new();

        calorie_data.push(0);

        for line in reader.lines() {
            match line {
                Ok(str) => {
                    if str.is_empty() {
                        calorie_data.push(0)
                    } else {
                        if let Some(cals) = calorie_data.last_mut() {
                            *cals += str.parse::<u32>()?
                        }
                    }
                },
                Err(e) => return Err(Box::new(e))
            }
        }

        // sort by top
        calorie_data.sort();
        calorie_data.reverse();

        Ok(Self {
            calorie_data
        })
    }
}

impl DayOne {
    pub fn most_calories(&self) -> Option<&u32> {
        self.calorie_data.first()
    }

    pub fn get_top(&self, top_count: usize) -> Option<Vec<u32>> {
        if top_count > self.calorie_data.len() {
            None
        } else {
            Some(self.calorie_data[0..top_count].to_vec())
        }
    }
}