mod model;
mod parser;
mod part1;
mod part2;

fn main() {
    let games = parser::parse();

    part1::solve(&games);
    part2::solve(&games);
}
