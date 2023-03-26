use std::{error::Error, fs::File, io::{BufReader, BufRead}};

pub struct DayTwo {
                // elf  player
    turns: Vec<(String, String)>,
}

impl DayTwo {
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut turns = Vec::new();

        for (index, line) in reader.lines().enumerate() {
            match line {
                Ok(str) => {
                    let split = str.split(' ').collect::<Vec<&str>>();

                    if split.len() != 2 {
                        let err = format!("line {} has {} inputs instead of 2.", index, split.len());
                        return Err(err.as_str())?
                    }

                    //          elf                  player
                    turns.push((split[0].to_owned(), split[1].to_owned()));
                },
                Err(e) => return Err(Box::new(e))
            }
        }

        

        Ok(Self { turns })
    }
}

// X = R
// Y = P
// Z = S

// A = R
// B = P
// C = S

impl DayTwo {
    pub fn get_assumed_total(&self) -> u32 {
        let mut total_score = 0;

        for (elf_move, player_move) in &self.turns {
            total_score += calc_points_for_move(elf_move, player_move)
        }

        total_score
    }

    // X = lose (0)
    // Y = draw (3)
    // Z = win (6)

    pub fn get_actual_total(&self) -> u32 {
        let mut actual_total_score = 0;

        for (elf_move, player_action) in &self.turns {
            let player_move = which_to_play(elf_move, player_action);

            actual_total_score += calc_points_for_move(elf_move, &player_move)
        }

        actual_total_score
    }
}

fn which_to_play(elf_move: &String, player_action: &String) -> String {
    let player_move = match player_action.as_str() {
        "X" => match elf_move.as_str() { // lose
            "A" => "Z", // lose vs. rock; scissors
            "B" => "X", // lose vs. paper; rock
            "C" => "Y", // lose vs. scissors; paper
             _  => "L", // invalid
        },
        "Y" => match elf_move.as_str() { // draw
            "A" => "X", // rock vs. rock
            "B" => "Y", // paper vs. paper
            "C" => "Z", // scissors vs. scissors
             _  => "L",
        },
        "Z" => match elf_move.as_str() { // win
            "A" => "Y", // win vs. rock; paper        
            "B" => "Z", // win vs. paper; scissors
            "C" => "X", // win vs. scissors; rock
             _  => "L" // invalid
        },
        _ => "L",
    };

    String::from(player_move)
}

fn calc_points_for_move(elf_move: &String, player_move: &String) -> u32 {
    let mut points = 0;

    match player_move.as_str() {
        "X" => { // rock
            points += 1;

            match elf_move.as_str() {
                "A" => points += 3, // rock vs. rock; tie
                "C" => points += 6, // rock vs. scissors; win
                 _  => points += 0, // rock vs. paper/invalid; lose
            }
        },
        "Y" => { // paper
            points += 2;

            match elf_move.as_str() {
                "A" => points += 6, // paper vs. rock; win
                "B" => points += 3, // paper vs. paper; tie
                 _  => points += 0, // paper vs. scissors/invalid; lose
            }
        },
        "Z" => { // scissors
            points += 3;

            match elf_move.as_str() {
                "B" => points += 6, // scissors vs. paper; win
                "C" => points += 3, // scissors vs. scissors; tie
                 _  => points += 0, // scissors vs. rock/invalid; lose
            }
        },
        _ => points += 0,
    }

    points
}