use reader::{read_file, File};

pub fn parse() -> Vec<usize> {
    const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    const NOT_SYMBOLS: [char; 11] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'];

    let contents = read_file(File::Day03);
    let lines = &contents.lines().collect::<Vec<&str>>();

    let char_vec = &contents
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let max_y = lines.iter().count() - 1;
    let max_x = lines[0].chars().count() - 1;

    let mut output: Vec<usize> = vec![];

    for y_index in 0..lines.iter().count() {
        let line = lines[y_index];
        let mut it = line.char_indices();

        'outer: while let Some((index, c)) = it.next() {
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

            if start_index > 0 && !NOT_SYMBOLS.contains(&char_vec[y_index][start_index - 1]) {
                let num = line[start_index..end_index].parse::<usize>().unwrap();
                output.push(num);
                continue 'outer;
            }

            if end_index < max_x && !NOT_SYMBOLS.contains(&char_vec[y_index][end_index]) {
                let num = line[start_index..end_index].parse::<usize>().unwrap();
                output.push(num);
                continue 'outer;
            }

              if y_index > 0 {
                  let start = if start_index == 0 { 0 } else { start_index - 1 };
                  let end = if end_index >= max_x { max_x } else { end_index + 1 };

                  for index in start..end {
                      if !NOT_SYMBOLS.contains(&char_vec[y_index - 1][index]) {
                          let num = line[start_index..end_index].parse::<usize>().unwrap();
                          output.push(num);
                          continue 'outer;
                      }
                  }
              }

              if y_index < max_y {
                  let start = if start_index == 0 { 0 } else { start_index - 1 };
                  let end = if end_index >= max_x { max_x } else { end_index + 1 };

                  for index in start..end {
                      if !NOT_SYMBOLS.contains(&char_vec[y_index + 1][index]) {
                          let num = line[start_index..end_index].parse::<usize>().unwrap();
                          output.push(num);
                          continue 'outer;
                      }
                  }
              }
        }
    }

    output
}
