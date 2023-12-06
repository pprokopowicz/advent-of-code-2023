use std::str::Lines;

use reader::{read_file, File};

pub fn parse<'a>() -> (Vec<String>, Vec<String>) {
    let contents = read_file(File::Day06);
    let mut lines = contents.lines();

    let times = parse_next_line(&mut lines);
    let distances = parse_next_line(&mut lines);

    (times, distances)
}

fn parse_next_line(lines: &mut Lines<'_>) -> Vec<String> {
    lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|num| num.to_owned())
        .collect::<Vec<String>>()
}
