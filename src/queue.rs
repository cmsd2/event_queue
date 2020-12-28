use std::collections::BinaryHeap;
use super::{Event,Time};

pub struct EventQueue<T> {
    generation: u32,
    queue: BinaryHeap<Event<T>>,
}

impl <T> Default for EventQueue<T> {
    fn default() -> Self {
        EventQueue::new()
    }
}

impl <T> EventQueue<T> {
    pub fn new() -> Self {
        EventQueue {
            generation: 0,
            queue: BinaryHeap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn add(&mut self, when: Time, item: T) {
        self.generation = self.generation.checked_add(1).expect("event queue wrap around");
        self.queue.push(Event { time: when, generation: self.generation, item: item })
    }

    pub fn next(&mut self) -> Option<(Time, T)> {
        self.queue
            .pop()
            .map(|event| (event.time, event.item))
    }

    pub fn peek<'a>(&'a self) -> Option<(&'a Time, &'a T)> {
        self.queue
            .peek()
            .map(|event| (&event.time, &event.item))
    }

    pub fn has_next(&self) -> bool {
        !self.queue.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::Time;

    #[test]
    pub fn test_queue() {
        let mut q = EventQueue::new();
        q.add(Time::default() + 1, "b");
        q.add(Time::default(), "a");
        q.add(Time::default(), "a2");

        assert!(q.has_next());
        assert_eq!(q.len(), 3);
        assert_eq!(q.peek(), Some((&Time::default(), &"a")));
        assert_eq!(q.next(), Some((Time::default(), "a")));
        assert_eq!(q.next(), Some((Time::default(), "a2")));
        assert_eq!(q.next(), Some((Time::default() + 1, "b")));
        assert_eq!(q.next(), None);
        assert_eq!(q.has_next(), false);
    }
}
