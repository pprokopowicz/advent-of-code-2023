mod parser;
mod part1;

fn main() {
    let input = parser::parse();

    // input.iter().for_each(|num| println!("{:?}", num));
    part1::solve(&input);
}
