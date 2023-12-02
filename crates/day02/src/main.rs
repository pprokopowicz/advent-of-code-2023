mod model;
mod parser;
mod part1;

fn main() {
    let games = parser::parse();

    part1::solve(&games);
}
