use crate::{model::UniverseImage, solver};

pub fn solve(input: &UniverseImage) {
    let result = solver::solve(input, 1000000);
    println!("Part 2 solution: {}", result);
}
