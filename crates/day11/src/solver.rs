use crate::model::{Coordinate, Point, UniverseImage};
use pathfinding::prelude::bfs;
use rayon::prelude::*;

pub fn solve(input: &UniverseImage, expansion_size: usize) -> usize {
    let mut result = 0;
    let galaxy_coordinates = galaxy_coordinates(&input.map);

    let mut galaxies_to_find = galaxy_coordinates.to_vec();

    for coordinate in galaxy_coordinates {
        galaxies_to_find.remove(0);

        let sum: usize = galaxies_to_find
            .par_iter()
            .filter_map(|to_find| {
                let path = bfs(
                    &coordinate,
                    |coordinate| successors(&coordinate, &input.map),
                    |coordinate| coordinate == to_find,
                )?;

                let sum = path
                    .iter()
                    .map(|coordinate| {
                        let mut step = 0;

                        if input.horizontal_lines.contains(&coordinate.y) {
                            step += expansion_size - 1;
                        }

                        if input.vertical_lines.contains(&coordinate.x) {
                            step += expansion_size - 1;
                        }

                        step
                    })
                    .sum::<usize>()
                    + path.len()
                    - 1;

                Option::Some(sum)
            })
            .sum();

        result += sum;
    }

    result
}

fn successors(coordiante: &Coordinate, input: &Vec<Vec<Point>>) -> Vec<Coordinate> {
    let mut output = vec![];

    if coordiante.y > 0 {
        output.push(Coordinate::new(coordiante.x, coordiante.y - 1));
    }

    if coordiante.y < input.len() - 1 {
        output.push(Coordinate::new(coordiante.x, coordiante.y + 1));
    }

    if coordiante.x > 0 {
        output.push(Coordinate::new(coordiante.x - 1, coordiante.y));
    }

    if coordiante.x < input[coordiante.y].len() - 1 {
        output.push(Coordinate::new(coordiante.x + 1, coordiante.y));
    }

    output
}

fn galaxy_coordinates(input: &Vec<Vec<Point>>) -> Vec<Coordinate> {
    let mut galaxy_coordinates: Vec<Coordinate> = vec![];

    for (y_index, line) in input.iter().enumerate() {
        for (x_index, point) in line.iter().enumerate() {
            if point == &Point::Galaxy {
                galaxy_coordinates.push(Coordinate::new(x_index, y_index));
            }
        }
    }

    galaxy_coordinates
}
