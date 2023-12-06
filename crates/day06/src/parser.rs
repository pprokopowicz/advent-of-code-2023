use std::str::Lines;

use reader::{read_file, File};

use crate::model::Race;

pub fn parse() -> Vec<Race> {
    let contents = read_file(File::Day06);
    let mut lines = contents.lines();

    let times = parse_next_line(&mut lines);
    let distances = parse_next_line(&mut lines);

    let races = times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| Race::new(time.clone(), distance.clone()))
        .collect::<Vec<Race>>();

    races
}

fn parse_next_line(lines: &mut Lines<'_>) -> Vec<usize> {
    lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|num| num.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}
