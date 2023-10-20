use super::rate::Rate;

#[derive(Debug)]
pub struct RateLimit<T> {
    inner: T,
}

impl<T> RateLimit<T> {
    pub fn new(service: T, rate: Rate) -> RateLimit<T> {
        RateLimit { inner: service }
    }
}
