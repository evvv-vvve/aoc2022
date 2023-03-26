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
        "X" => { // lose
            match elf_move.as_str() {
                "A" => { // rock
                    // lose vs. rock; scissors
                    "Z"
                },
                "B" => { // paper
                    // lose vs. paper; rock
                    "X"
                },
                "C" => { // scissors
                    // lose vs. scissors; paper
                    "Y"
                },
                _ => { // invalid
                    "L"
                },
            }
        },
        "Y" => { // draw
            match elf_move.as_str() {
                "A" => "X",
                "B" => "Y",
                "C" => "Z",
                _ => "L",
            }
        },
        "Z" => { // win
            match elf_move.as_str() {
                "A" => { // rock
                    // win vs. rock; paper
                    "Y"
                }
                "B" => { // paper
                    // win vs. paper; scissors
                    "Z"
                },
                "C" => { // scissors
                    // win vs. scissors; rock
                    "X"
                },
                _ => { // invalid
                    "L"
                },
            }
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
                "A" => { // rock
                    // rock vs. rock; tie
                    points += 3;
                },
                "C" => { // scissors
                    // rock vs. scissors; win
                    points += 6;
                },
                _ => { // paper / invalid
                    // rock vs. paper/invalid; lose
                    points += 0;
                },
            }
        },
        "Y" => { // paper
            points += 2;

            match elf_move.as_str() {
                "A" => { // rock
                    // paper vs. rock; win
                    points += 6;
                },
                "B" => { // paper
                    // paper vs. paper; tie
                    points += 3;
                },
                _ => { // scissors / invalid
                    // paper vs. scissors/invalid; lose
                    points += 0;
                },
            }
        },
        "Z" => { // scissors
            points += 3;

            match elf_move.as_str() {
                "B" => { // paper
                    // scissors vs. paper; win
                    points += 6;
                },
                "C" => { // scissors
                    // scissors vs. scissors; tie
                    points += 3;
                },
                _ => { // rock / invalid
                    // scissors vs. rock/invalid; lose
                    points += 0;
                },
            }
        },
        _ => points += 0,
    }

    points
}