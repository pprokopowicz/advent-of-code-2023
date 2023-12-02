use crate::model::Game;

pub fn solve(input: &Vec<Game>) {
    const AVAILABLE_RED: usize = 12;
    const AVAILABLE_GREEN: usize = 13;
    const AVAILABLE_BLUE: usize = 14;

    let result: usize = input
        .iter()
        .filter_map(|game| {
            let number_of_correct_sets = game
                .sets
                .iter()
                .filter(|set| {
                    set.red <= AVAILABLE_RED
                        && set.green <= AVAILABLE_GREEN
                        && set.blue <= AVAILABLE_BLUE
                })
                .count();

            if number_of_correct_sets == game.sets.iter().count() {
                Option::Some(game.id)
            } else {
                Option::None
            }
        })
        .sum();

    println!("Part 1 solution: {}", result);
}
