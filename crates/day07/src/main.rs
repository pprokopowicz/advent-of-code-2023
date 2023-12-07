use std::{cmp::Ordering, collections::HashMap};

use model::HandType;
use reader::{read_file, File};

mod model;
mod parser;
mod part1;
mod part2;

fn main() {
    let contents = read_file(File::Day07);

    part1::solve(&contents);
    part2::solve(&contents);
}
