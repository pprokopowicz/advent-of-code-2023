use crate::model::Race;

pub fn solve(input: &Vec<Race>) {
    let result: usize = input
        .iter()
        .map(|race| {
            let num_of_winning_combinations = (0..=race.time)
                .map(|time_holding_button| {
                    let remaining_time = race.time - time_holding_button;
                    remaining_time * time_holding_button
                })
                .filter(|distance| distance > &race.distance)
                .count();

            num_of_winning_combinations
        })
        .product();

    println!("Part 1 solution: {}", result);
}
