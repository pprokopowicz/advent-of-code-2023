use crate::model::{Coordinate, Point};

pub fn solve(input: &Vec<Vec<Point>>) {
    let mut result = 1;
    let mut previous_coordinate = start_coordinate(input);
    let mut current_coordinate = next_coordinate_from_start(&previous_coordinate, input);

    loop {
        let next = next_coordinate(&previous_coordinate, &current_coordinate, input);
        previous_coordinate = current_coordinate;
        current_coordinate = next;
        result += 1;

        if input[current_coordinate.y][current_coordinate.x] == Point::Start {
            break;
        }
    }

    result /= 2;

    println!("Part 1 solution: {}", result);
}

fn start_coordinate(input: &Vec<Vec<Point>>) -> Coordinate {
    for (y, line) in input.iter().enumerate() {
        for (x, point) in line.iter().enumerate() {
            match point {
                Point::Start => return Coordinate::new(x, y),
                _ => (),
            }
        }
    }

    panic!("No starting point!")
}

fn next_coordinate(
    previous_coordinate: &Coordinate,
    current_coordinate: &Coordinate,
    input: &Vec<Vec<Point>>,
) -> Coordinate {
    let current_point = &input[current_coordinate.y][current_coordinate.x];

    match current_point {
        Point::Vertical => {
            let y = if previous_coordinate.y > current_coordinate.y {
                current_coordinate.y - 1
            } else {
                current_coordinate.y + 1
            };
            return Coordinate::new(current_coordinate.x, y);
        }
        Point::Horizontal => {
            let x = if previous_coordinate.x > current_coordinate.x {
                current_coordinate.x - 1
            } else {
                current_coordinate.x + 1
            };
            return Coordinate::new(x, current_coordinate.y);
        }
        Point::NorthEast => {
            let (x, y) = if previous_coordinate.y < current_coordinate.y {
                (current_coordinate.x + 1, current_coordinate.y)
            } else {
                (current_coordinate.x, current_coordinate.y - 1)
            };
            return Coordinate::new(x, y);
        }
        Point::NorthWest => {
            let (x, y) = if previous_coordinate.y < current_coordinate.y {
                (current_coordinate.x - 1, current_coordinate.y)
            } else {
                (current_coordinate.x, current_coordinate.y - 1)
            };
            return Coordinate::new(x, y);
        }
        Point::SouthWest => {
            let (x, y) = if previous_coordinate.y > current_coordinate.y {
                (current_coordinate.x - 1, current_coordinate.y)
            } else {
                (current_coordinate.x, current_coordinate.y + 1)
            };
            return Coordinate::new(x, y);
        }
        Point::SouthEast => {
            let (x, y) = if previous_coordinate.y > current_coordinate.y {
                (current_coordinate.x + 1, current_coordinate.y)
            } else {
                (current_coordinate.x, current_coordinate.y + 1)
            };
            return Coordinate::new(x, y);
        }
        _ => panic!("Bad Point!"),
    }
}

fn next_coordinate_from_start(start: &Coordinate, input: &Vec<Vec<Point>>) -> Coordinate {
    let can_go_west = start.x > 0;
    let can_go_east = start.x < input[0].len() - 1;
    let can_go_south = start.y < input.len() - 1;
    let can_go_north = start.y > 0;

    if can_go_west {
        match input[start.y][start.x - 1] {
            Point::Horizontal => return Coordinate::new(start.x - 1, start.y),
            Point::NorthEast => return Coordinate::new(start.x - 1, start.y),
            Point::SouthEast => return Coordinate::new(start.x - 1, start.y),
            _ => (),
        }
    }

    if can_go_east {
        match input[start.y][start.x + 1] {
            Point::Horizontal => return Coordinate::new(start.x + 1, start.y),
            Point::NorthWest => return Coordinate::new(start.x + 1, start.y),
            Point::SouthWest => return Coordinate::new(start.x + 1, start.y),
            _ => (),
        }
    }

    if can_go_north {
        match input[start.y - 1][start.x] {
            Point::Vertical => return Coordinate::new(start.x, start.y - 1),
            Point::SouthWest => return Coordinate::new(start.x, start.y - 1),
            Point::SouthEast => return Coordinate::new(start.x, start.y - 1),
            _ => (),
        }
    }

    if can_go_south {
        match input[start.y + 1][start.x] {
            Point::Vertical => return Coordinate::new(start.x, start.y + 1),
            Point::NorthEast => return Coordinate::new(start.x, start.y + 1),
            Point::NorthWest => return Coordinate::new(start.x, start.y + 1),
            _ => (),
        }
    }

    panic!("No next point!");
}
