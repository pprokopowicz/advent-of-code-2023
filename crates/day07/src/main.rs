mod model;
mod parser;
mod part1;

fn main() {
    let input = parser::parse();

    part1::solve(&input);
}
