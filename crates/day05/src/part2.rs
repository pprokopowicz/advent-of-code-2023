use std::ops::Range;

use rayon::prelude::*;

use crate::{model::GardeningMap, solver};

pub fn solve(seeds: &Vec<usize>, maps: &Vec<GardeningMap>) {
    let mut seed_ranges: Vec<Range<usize>> = vec![];
    for i in (0..seeds.len()).step_by(2) {
        seed_ranges.push(seeds[i]..(seeds[i] + seeds[i + 1]));
    }

    let result = seed_ranges
        .par_iter()
        .map(|seed_range| {
            let seed_vec = seed_range.clone().collect::<Vec<usize>>();
            solver::solve(&seed_vec, maps)
        })
        .min()
        .unwrap();

    println!("Part 2 solution: {}", result);
}
