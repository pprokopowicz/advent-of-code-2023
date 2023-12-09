use crate::solver;

pub fn solve(input: &Vec<Vec<isize>>) {
    let result = solver::solve(input, result);
    println!("Part 2 solution: {}", result);
}

fn result(differences: Vec<Vec<isize>>) -> isize {
    let len = differences.len();
    let mut previous_first_value = 0;

    for index in (0..(len - 1)).rev() {
        let first = differences[index][0];

        previous_first_value = first - previous_first_value;
    }

    previous_first_value
}
