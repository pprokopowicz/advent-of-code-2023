#[derive(Debug)]
pub struct Race {
    pub time: usize,
    pub distance: usize,
}

impl Race {
    pub fn new(time: usize, distance: usize) -> Race {
        Race { time, distance }
    }
}
