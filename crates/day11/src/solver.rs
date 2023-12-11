use crate::model::{Coordinate, Point, UniverseImage};

pub fn solve(input: &UniverseImage, expansion_size: usize) -> usize {
    let galaxy_coordinates = galaxy_coordinates(&input.map);
    let mut galaxies_to_left = galaxy_coordinates.to_vec();

    let result = galaxy_coordinates
        .iter()
        .map(|first_galaxy| {
            galaxies_to_left.remove(0);

            galaxies_to_left
                .iter()
                .map(|second_galaxy| distance(&first_galaxy, second_galaxy, input, &expansion_size))
                .sum::<usize>()
        })
        .sum::<usize>();

    result
}

fn distance(
    galaxy0: &Coordinate,
    galaxy1: &Coordinate,
    image: &UniverseImage,
    expansion_size: &usize,
) -> usize {
    let start_y = galaxy0.y.min(galaxy1.y);
    let end_y = galaxy0.y.max(galaxy1.y);
    let start_x = galaxy0.x.min(galaxy1.x);
    let end_x = galaxy0.x.max(galaxy1.x);

    let y_sum = (start_y..end_y)
        .map(|y| {
            if image.horizontal_lines.contains(&y) {
                expansion_size
            } else {
                &1
            }
        })
        .sum::<usize>();
    let x_sum = (start_x..end_x)
        .map(|x| {
            if image.vertical_lines.contains(&x) {
                expansion_size
            } else {
                &1
            }
        })
        .sum::<usize>();

    y_sum + x_sum
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
