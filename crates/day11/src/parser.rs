use reader::{read_file, File};

use crate::model::{Point, UniverseImage};

pub fn parse() -> UniverseImage {
    let contents = read_file(File::Day11);

    let map = contents
        .lines()
        .map(|line| line.chars().map(|c| Point::new(&c)).collect::<Vec<Point>>())
        .collect::<Vec<Vec<Point>>>();

    let vertical_lines = vertical_lines(&map);
    let horizontal_lines = horizontal_lines(&map);

    UniverseImage {
        map,
        horizontal_lines,
        vertical_lines,
    }
}

fn vertical_lines(map: &Vec<Vec<Point>>) -> Vec<usize> {
    let horizontal_len = map[0].len();

    (0..horizontal_len)
        .filter_map(|x_index| {
            let is_only_space = (0..map.len()).all(|y_index| map[y_index][x_index] == Point::Space);

            if is_only_space {
                Option::Some(x_index)
            } else {
                Option::None
            }
        })
        .collect()
}

fn horizontal_lines(map: &Vec<Vec<Point>>) -> Vec<usize> {
    (0..map.len())
        .filter_map(|index| {
            let is_only_space = map[index].iter().all(|point| point == &Point::Space);

            if is_only_space {
                Option::Some(index)
            } else {
                Option::None
            }
        })
        .collect()
}
