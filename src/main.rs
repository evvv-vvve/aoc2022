mod day_one;

fn main() {
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
