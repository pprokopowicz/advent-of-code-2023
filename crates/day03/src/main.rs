mod part1;
mod part2;

use reader::{read_file, File};

fn main() {
    let input = read_file(File::Day03);

    part1::solve(&input);
    part2::solve(&input);
}
