use crate::{model::Race, solver};

pub fn solve(times: &Vec<String>, distances: &Vec<String>) {
    let races = times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| {
            Race::new(
                time.parse::<usize>().unwrap(),
                distance.parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<Race>>();

    let result: usize = races.iter().map(|race| solver::solve(race)).product();

    println!("Part 1 solution: {}", result);
}
