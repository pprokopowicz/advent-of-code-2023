use std::ops::Range;

#[derive(Debug)]
pub struct GardeningMap {
    pub source_ranges: Vec<Range<usize>>,
    pub destination_ranges: Vec<Range<usize>>,
}

impl GardeningMap {
    pub fn new(
        source_ranges: Vec<Range<usize>>,
        destination_ranges: Vec<Range<usize>>,
    ) -> GardeningMap {
        GardeningMap {
            source_ranges,
            destination_ranges,
        }
    }
}
