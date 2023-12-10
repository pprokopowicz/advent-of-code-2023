use crate::model::{Pipe, Point};

pub fn solve(input: &Vec<Vec<Point>>) {
    let mut result = 0;

    for line in input {
        let mut previous_bend: Option<&Pipe> = Option::None;
        let mut is_outside = true;

        for point in line {
            if point.is_in_loop {
                match point.pipe {
                    Pipe::Start => is_outside = !is_outside,
                    Pipe::Vertical => is_outside = !is_outside,
                    Pipe::NorthEast => previous_bend = Option::Some(&Pipe::NorthEast),
                    Pipe::NorthWest => {
                        match previous_bend {
                            Option::Some(bend) => match bend {
                                Pipe::NorthEast => (),
                                Pipe::SouthEast => is_outside = !is_outside,
                                _ => (),
                            },
                            Option::None => (),
                        };
                        previous_bend = Option::None;
                    }
                    Pipe::SouthEast => previous_bend = Option::Some(&Pipe::SouthEast),
                    Pipe::SouthWest => {
                        match previous_bend {
                            Option::Some(bend) => match bend {
                                Pipe::NorthEast => is_outside = !is_outside,
                                Pipe::SouthEast => (),
                                _ => (),
                            },
                            Option::None => (),
                        };
                        previous_bend = Option::None;
                    }
                    _ => (),
                }
            } else {
                if !is_outside {
                    result += 1;
                }
            }
        }
    }

    println!("Part 2 solution: {}", result);
}
