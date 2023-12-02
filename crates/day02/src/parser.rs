use reader::{read_file, File};

use crate::model::{Game, Set};

pub fn parse() -> Vec<Game> {
    let contents = read_file(File::Day02);

    contents
        .lines()
        .map(|line| {
            let sliced: &str = &line[5..line.len()];
            let split_by_colon = sliced.split(": ").collect::<Vec<&str>>();
            let id = split_by_colon[0].parse::<usize>().unwrap();

            let sets = split_by_colon[1]
                .split("; ")
                .map(|set_str| {
                    let mut red: usize = 0;
                    let mut green: usize = 0;
                    let mut blue: usize = 0;

                    set_str.split(", ").for_each(|cube_str| {
                        let split_by_whitespace =
                            cube_str.split_whitespace().collect::<Vec<&str>>();

                        let number = split_by_whitespace[0].parse::<usize>().unwrap();
                        match split_by_whitespace[1] {
                            "red" => red = number,
                            "green" => green = number,
                            "blue" => blue = number,
                            _ => {}
                        }
                    });

                    Set::new(red, green, blue)
                })
                .collect::<Vec<Set>>();

            Game::new(id, sets)
        })
        .collect::<Vec<Game>>()
}
