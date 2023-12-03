use std::collections::HashMap;

#[derive(PartialEq, Hash, Eq, Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

pub fn solve(input: &String) {
    let parsed_input = parse(input);
    let result = parsed_input.iter().sum::<usize>();
    println!("Part 2 solution: {}", result);
}

fn parse(input: &String) -> Vec<usize> {
    const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let contents = input;
    let lines = &contents.lines().collect::<Vec<&str>>();

    let char_vec = &contents
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let max_y = lines.iter().count() - 1;
    let max_x = lines[0].chars().count() - 1;

    let mut map: HashMap<Coordinate, Vec<usize>> = HashMap::new();

    for y_index in 0..lines.iter().count() {
        let line = lines[y_index];
        let mut it = line.char_indices();

        while let Some((index, c)) = it.next() {
            if !DIGITS.contains(&c) {
                continue;
            }

            let start_index = index;
            let mut end_index = index + 1;

            let mut stop = true;

            while stop {
                let next = it.next();
                match next {
                    Option::Some((next_index, next_c)) => {
                        if DIGITS.contains(&next_c) {
                            end_index = next_index;
                        } else {
                            end_index = next_index;
                            stop = false;
                        }
                    }
                    Option::None => {
                        end_index += 1;
                        stop = false;
                    }
                }
            }

            if start_index > 0 && char_vec[y_index][start_index - 1] == '*' {
                let num = line[start_index..end_index].parse::<usize>().unwrap();
                let coordinate = Coordinate {
                    x: start_index - 1,
                    y: y_index,
                };
                let mut values = match map.get(&coordinate) {
                    Option::Some(nums) => nums.clone(),
                    Option::None => vec![],
                };
                values.push(num);
                map.insert(coordinate, values);
            }

            if end_index < max_x && char_vec[y_index][end_index] == '*' {
                let num = line[start_index..end_index].parse::<usize>().unwrap();
                let coordinate = Coordinate {
                    x: end_index,
                    y: y_index,
                };
                let mut values = match map.get(&coordinate) {
                    Option::Some(nums) => nums.clone(),
                    Option::None => vec![],
                };
                values.push(num);
                map.insert(coordinate, values);
            }

            if y_index > 0 {
                let start = if start_index == 0 { 0 } else { start_index - 1 };
                let end = if end_index >= max_x {
                    max_x
                } else {
                    end_index + 1
                };

                for index in start..end {
                    if char_vec[y_index - 1][index] == '*' {
                        let num = line[start_index..end_index].parse::<usize>().unwrap();
                        let coordinate = Coordinate {
                            x: index,
                            y: y_index - 1,
                        };
                        let mut values = match map.get(&coordinate) {
                            Option::Some(nums) => nums.clone(),
                            Option::None => vec![],
                        };
                        values.push(num);
                        map.insert(coordinate, values);
                    }
                }
            }

            if y_index < max_y {
                let start = if start_index == 0 { 0 } else { start_index - 1 };
                let end = if end_index >= max_x {
                    max_x
                } else {
                    end_index + 1
                };

                for index in start..end {
                    if char_vec[y_index + 1][index] == '*' {
                        let num = line[start_index..end_index].parse::<usize>().unwrap();
                        let coordinate = Coordinate {
                            x: index,
                            y: y_index + 1,
                        };
                        let mut values = match map.get(&coordinate) {
                            Option::Some(nums) => nums.clone(),
                            Option::None => vec![],
                        };
                        values.push(num);
                        map.insert(coordinate, values);
                    }
                }
            }
        }
    }

    map.iter()
        .filter_map(|(_, values)| {
            if values.len() == 2 {
                return Option::Some(values[0] * values[1]);
            }
            Option::None
        })
        .collect::<Vec<usize>>()
}
