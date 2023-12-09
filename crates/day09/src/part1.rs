use crate::solver;

pub fn solve(input: &Vec<Vec<isize>>) {
    let result = solver::solve(input, result);
    println!("Part 1 solution: {}", result);
}

fn result(differences: Vec<Vec<isize>>) -> isize {
    differences
        .iter()
        .map(|values| values.last().unwrap())
        .sum()
}
