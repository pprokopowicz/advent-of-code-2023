pub fn solve<F>(input: &Vec<Vec<isize>>, result_fn: F) -> isize
where
    F: Fn(Vec<Vec<isize>>) -> isize,
{
    input
        .iter()
        .map(|values| {
            let differences = differences(values);
            result_fn(differences)
        })
        .sum::<isize>()
}

fn is_last_step(values: &Vec<isize>) -> bool {
    values.iter().all(|val| val == &0)
}

fn differences(initial_values: &Vec<isize>) -> Vec<Vec<isize>> {
    let mut output: Vec<Vec<isize>> = vec![initial_values.to_vec()];
    let mut curr_differences_index = 0;

    loop {
        let next_diff = next_steps(&output[curr_differences_index]);
        curr_differences_index += 1;
        output.push(next_diff.to_vec());

        if is_last_step(&next_diff) {
            break;
        }
    }

    output
}

fn next_steps(previous: &Vec<isize>) -> Vec<isize> {
    let mut output = vec![];

    for index in (1..previous.len()).rev() {
        output.push(previous[index] - previous[index - 1]);
    }

    output.reverse();

    output
}
