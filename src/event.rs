use super::Time;
use std::cmp::Ordering;

pub struct Event<T> {
    pub time: Time,
    pub generation: u32,
    pub item: T,
}

impl<T> PartialEq for Event<T> {
    fn eq(&self, other: &Event<T>) -> bool {
        self.time == other.time && self.generation == other.generation
    }
}

impl<T> Eq for Event<T> {}

impl<T> Ord for Event<T> {
    fn cmp(&self, other: &Event<T>) -> Ordering {
        other
            .time
            .cmp(&self.time)
            .then_with(|| other.generation.cmp(&self.generation))
    }
}

impl<T> PartialOrd for Event<T> {
    fn partial_cmp(&self, other: &Event<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
