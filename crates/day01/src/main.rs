mod parser;
mod solver;

fn main() {
    let input1 = parser::parse_part1();
    let input2 = parser::parse_part2();

    println!("Part 1 solution: {}", solver::solve(&input1));
    println!("Part 2 solution: {}", solver::solve(&input2));
}
