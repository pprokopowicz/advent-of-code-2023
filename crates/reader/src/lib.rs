use std::fs;

pub enum File {
    Day01 = 1,
    Day02,
    Day03,
    Day04,
    Day05,
    Day06,
}

fn file_name(file: File) -> String {
    let index = file as u32;
    if index > 9 {
        format!("inputs/day{}.txt", index)
    } else {
        format!("inputs/day0{}.txt", index)
    }
}

pub fn read_file(file: File) -> String {
    fs::read_to_string(file_name(file)).unwrap()
}
