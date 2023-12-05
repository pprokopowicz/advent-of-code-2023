use crate::model::GardeningMap;

pub fn solve(seeds: &Vec<usize>, maps: &Vec<GardeningMap>) {
    let result = seeds
        .iter()
        .map(|seed| {
            let mut current_value = seed.clone();

            maps.iter().for_each(|map| {
                for (index, range) in map.source_ranges.iter().enumerate() {
                    if range.contains(&current_value) {
                        let destination_start = map.destination_ranges[index].start;
                        let offset = current_value - range.start;

                        current_value = destination_start + offset;
                        break;
                    }
                }
            });

            current_value
        })
        .min()
        .unwrap();

    println!("Part 1 solution: {}", result);
}
