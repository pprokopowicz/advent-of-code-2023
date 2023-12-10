use reader::{read_file, File};

use crate::model::{Pipe, Point};

pub fn parse() -> Vec<Vec<Point>> {
    let contents = read_file(File::Day10);

    contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Point::new(Pipe::new(&c), false))
                .collect::<Vec<Point>>()
        })
        .collect::<Vec<Vec<Point>>>()
}
