use super::{Event, Time};
use std::collections::BinaryHeap;

pub struct EventQueue<T> {
    generation: u32,
    queue: BinaryHeap<Event<T>>,
}

impl<T> Default for EventQueue<T> {
    fn default() -> Self {
        EventQueue::new()
    }
}

impl<T> EventQueue<T> {
    pub fn new() -> Self {
        EventQueue {
            generation: 0,
            queue: BinaryHeap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn peek(&self) -> Option<(&Time, &T)> {
        self.queue.peek().map(|event| (&event.time, &event.item))
    }

    pub fn add(&mut self, when: Time, item: T) {
        self.generation = self
            .generation
            .checked_add(1)
            .expect("event queue wrap around");
        self.queue.push(Event {
            time: when,
            generation: self.generation,
            item,
        })
    }

    pub fn has_next(&self) -> bool {
        !self.queue.is_empty()
    }
}

impl<T> Iterator for EventQueue<T> {
    type Item = (Time, T);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.queue.pop().map(|event| (event.time, event.item))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::Time;

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
