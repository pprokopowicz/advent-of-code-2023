use crate::{model::GardeningMap, solver};

pub fn solve(seeds: &Vec<usize>, maps: &Vec<GardeningMap>) {
    let result = solver::solve(seeds, maps);

    println!("Part 1 solution: {}", result);
}
