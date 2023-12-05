use std::ops::Range;

use reader::{read_file, File};

use crate::model::GardeningMap;

pub fn parse() -> (Vec<usize>, Vec<GardeningMap>) {
    let contents = read_file(File::Day05);

    let mut split = contents.split("\n\n");

    let seeds = split
        .nth(0)
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|seed| seed.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let maps = split
        .map(|map| {
            let mut source_ranges: Vec<Range<usize>> = vec![];
            let mut destination_ranges: Vec<Range<usize>> = vec![];

            map.lines().skip(1).for_each(|line| {
                let nums = line
                    .split_whitespace()
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                source_ranges.push(nums[1]..(nums[1] + nums[2]));
                destination_ranges.push(nums[0]..(nums[0] + nums[2]));
            });

            GardeningMap::new(source_ranges, destination_ranges)
        })
        .collect::<Vec<GardeningMap>>();

    (seeds, maps)
}
