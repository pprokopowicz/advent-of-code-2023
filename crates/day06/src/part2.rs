use crate::{model::Race, solver};

pub fn solve(times: &Vec<String>, distances: &Vec<String>) {
    let time = times.concat().parse::<usize>().unwrap();
    let distance: usize = distances.concat().parse::<usize>().unwrap();

    let race = Race::new(time, distance);

    let result: usize = solver::solve(&race);

    println!("Part 2 solution: {}", result);
}
