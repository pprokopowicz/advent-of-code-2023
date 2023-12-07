use reader::{read_file, File};

mod model;
mod parser;
mod part1;
mod part2;
mod solver;

fn main() {
    let contents = read_file(File::Day07);

    part1::solve(&contents);
    part2::solve(&contents);
}
