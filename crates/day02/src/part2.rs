use crate::model::Game;

pub fn solve(input: &Vec<Game>) {
    let result: usize = input
        .iter()
        .map(|game| {
            let max_red = game.sets.iter().map(|set| set.red).max().unwrap();
            let max_green = game.sets.iter().map(|set| set.green).max().unwrap();
            let max_blue = game.sets.iter().map(|set| set.blue).max().unwrap();

            max_red * max_green * max_blue
        })
        .sum();

    println!("Part 2 solution: {}", result);
}
