use reader::{read_file, File};

pub fn parse_part1() -> Vec<u32> {
    const RADIX: u32 = 10;
    let contents = read_file(File::Day01);

    contents
        .lines()
        .map(|val| {
            let mut first_num: Option<u32> = Option::None;
            let mut last_num: Option<u32> = Option::None;

            val.chars().into_iter().for_each(|c| {
                let result = c.to_digit(RADIX);
                match result {
                    Some(x) => {
                        if first_num == Option::None {
                            first_num = Option::Some(x);
                        }
                        last_num = Option::Some(x);
                    }
                    None => (),
                };
            });

            first_num.unwrap() * 10 + last_num.unwrap()
        })
        .collect::<Vec<u32>>()
}

static POSSIBLE_DIGIT_VALUES: &'static [(&str, usize)] = &[
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];
static POSSIBLE_STR_VALUES: &'static [(&str, usize)] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

pub fn parse_part2() -> Vec<u32> {
    let contents = read_file(File::Day01);

    contents
        .lines()
        .map(|val| {
            let mut available_first_digits = POSSIBLE_DIGIT_VALUES
                .iter()
                .filter_map(|digit| {
                    let index = val.find(digit.0)?;
                    Option::Some((index, digit.1))
                })
                .collect::<Vec<(usize, usize)>>();

            let mut str_first_digits = POSSIBLE_STR_VALUES
                .iter()
                .filter_map(|digit| {
                    let index = val.find(digit.0)?;
                    Option::Some((index, digit.1))
                })
                .collect::<Vec<(usize, usize)>>();

            let mut available_last_digits = POSSIBLE_DIGIT_VALUES
                .iter()
                .filter_map(|digit| {
                    let index = val.rfind(digit.0)?;
                    Option::Some((index, digit.1))
                })
                .collect::<Vec<(usize, usize)>>();

            let mut str_last_digits = POSSIBLE_STR_VALUES
                .iter()
                .filter_map(|digit| {
                    let index = val.rfind(digit.0)?;
                    Option::Some((index, digit.1))
                })
                .collect::<Vec<(usize, usize)>>();

            available_first_digits.append(&mut str_first_digits);
            available_last_digits.append(&mut str_last_digits);

            let first_num = available_first_digits
                .iter()
                .min_by(|digit1, digit2| digit1.0.cmp(&digit2.0))
                .unwrap()
                .1;
            let last_num = available_last_digits
                .iter()
                .max_by(|digit1, digit2| digit1.0.cmp(&digit2.0))
                .unwrap()
                .1;

            u32::try_from(first_num * 10 + last_num).unwrap()
        })
        .collect::<Vec<u32>>()
}
