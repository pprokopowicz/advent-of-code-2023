use crate::model::{Coordinate, Point};
use pathfinding::prelude::bfs;
use rayon::prelude::*;

pub fn solve(input: &Vec<Vec<Point>>) {
    let mut result = 0;
    let galaxy_coordinates = galaxy_coordinates(input);

    let mut galaxies_to_find = galaxy_coordinates.to_vec();

    for coordinate in galaxy_coordinates {
        galaxies_to_find.remove(0);

        let sum: usize = galaxies_to_find
            .par_iter()
            .filter_map(|to_find| {
                Option::Some(
                    bfs(
                        &coordinate,
                        |coordinate| successors(&coordinate, input),
                        |coordinate| coordinate == to_find,
                    )?
                    .len()
                        - 1,
                )
            })
            .sum();

        result += sum;
    }

    println!("Part 1 solution: {}", result);
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
