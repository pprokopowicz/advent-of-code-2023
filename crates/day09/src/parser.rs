use reader::{read_file, File};

pub fn parse() -> Vec<Vec<isize>> {
    let contents = read_file(File::Day09);

    contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|val| val.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect::<Vec<Vec<isize>>>()
}
