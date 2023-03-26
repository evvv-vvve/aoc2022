mod day_one;
mod day_two;

fn main() {
    //run_day_one();
    run_day_two();
}

fn run_day_two() {
    match day_two::DayTwo::from_file("inputs\\day_two.txt") {
        Ok(day_two) => {
            println!("Assumed total Score: {}", day_two.get_assumed_total());
            println!("Actual total Score: {}", day_two.get_actual_total());
            
        },
        Err(e) => println!("Error while trying to load day_two: {}", e)
    }
}

fn run_day_one() {
    match day_one::DayOne::from_file("inputs\\day_one.txt") {
        Ok(day_one) => {
            match day_one.most_calories() {
                Some(cals) => println!("Most calories: {}", cals),
                None => println!("Most calories is inconclusive")
            }

            let top_count = 3;

            if let Some(cal_top) = day_one.get_top(top_count) {
                println!("Top 3 calorie counts total: {}", cal_top.iter().sum::<u32>());
            } else {
                println!("There are less than {} calorie totals!",top_count)
            }
        },
        Err(e) => println!("Error while trying to load day_one: {}", e)
    }
}