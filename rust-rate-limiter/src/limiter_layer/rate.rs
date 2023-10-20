use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub struct Rate {
    num: usize,
    interval: Duration,
}

impl Rate {
    pub fn new(num: usize, interval: Duration) -> Rate {
        Rate { num, interval }
    }

    pub(crate) fn num(&self) -> usize {
        self.num
    }

    pub(crate) fn interval(&self) -> Duration {
        self.interval
    }
}
