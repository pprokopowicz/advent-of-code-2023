use crate::{model::UniverseImage, solver};

pub fn solve(input: &UniverseImage) {
    let result = solver::solve(input, 2);
    println!("Part 1 solution: {}", result);
}
