use reader::{read_file, File};

use crate::model::Point;

pub fn parse() -> Vec<Vec<Point>> {
    let contents = read_file(File::Day11);

    let mut mapped = contents
        .lines()
        .map(|line| line.chars().map(|c| Point::new(&c)).collect::<Vec<Point>>())
        .collect::<Vec<Vec<Point>>>();

    expand_vertical_lines(&mut mapped);
    expand_horizontal_lines(&mut mapped);

    mapped
}

fn expand_vertical_lines(map: &mut Vec<Vec<Point>>) {
    let horizontal_len = map[0].len();
    let mut expand_space_indexes: Vec<usize> = vec![];

    for x_index in 0..horizontal_len {
        let is_only_space = (0..map.len()).all(|y_index| map[y_index][x_index] == Point::Space);

        if is_only_space {
            expand_space_indexes.push(x_index);
        }
    }

    for (index, x_index) in expand_space_indexes.iter().enumerate() {
        let modified_index = index + x_index + 1;
        for y_index in 0..map.len() {
            map[y_index].insert(modified_index, Point::Space);
        }
    }
}

fn expand_horizontal_lines(map: &mut Vec<Vec<Point>>) {
    let horizontal_len = map[0].len();
    let mut expand_space_indexes: Vec<usize> = vec![];

    for index in 0..map.len() {
        let is_only_space = map[index].iter().all(|point| point == &Point::Space);

        if is_only_space {
            expand_space_indexes.push(index);
        }
    }

    for (index, y_index) in expand_space_indexes.iter().enumerate() {
        let modified_index = index + y_index + 1;

        map.insert(modified_index, vec![Point::Space; horizontal_len])
    }
}
