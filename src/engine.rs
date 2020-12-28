use super::EventQueue;
use super::Time;
use std::fmt;

pub struct Engine<Actor, Action, Event> {
    pub stop: bool,
    pub turn: Option<Actor>,
    pub event_queue: EventQueue<Event>,
    pub action_queue: Vec<(Actor, Action)>,
    pub time: Time,
}

impl<Actor, Action, Event> Default for Engine<Actor, Action, Event>
where
    Actor: fmt::Debug + Clone,
    Action: fmt::Debug,
    Event: fmt::Debug,
{
    fn default() -> Self {
        Engine::new()
    }
}

impl<Actor, Action, Event> Engine<Actor, Action, Event>
where
    Actor: fmt::Debug + Clone,
    Action: fmt::Debug,
    Event: fmt::Debug,
{
    pub fn new() -> Self {
        Engine {
            stop: false,
            turn: None,
            event_queue: EventQueue::default(),
            action_queue: Vec::default(),
            time: Time::default(),
        }
    }

    pub fn end_turn(&mut self, actor: Actor) {
        self.turn = None;
        self.action_queue.clear();
        debug!("[{:?}] end turn for {:?}", self.time, actor);
    }

    pub fn current_turn(&self) -> &Option<Actor> {
        &self.turn
    }

    pub fn new_turn(&mut self, actor: Actor) {
        debug!("[{:?}] new turn for: {:?}", self.time, actor);

        self.turn = Some(actor);
    }

    pub fn finish(&mut self) {
        debug!("[{:?}] stop", self.time);
        self.stop = true;
    }

    pub fn is_finished(&self) -> bool {
        self.stop
    }

    pub fn action(&mut self, actor: Actor, action: Action) {
        self.action_queue.push((actor, action));
    }

    pub fn next_action(&mut self) -> Option<(Time, Actor, Action)> {
        if self.action_queue.is_empty() {
            None
        } else {
            let (actor, action) = self.action_queue.remove(0);

            debug!("[{:?}] next action: {:?} {:?}", self.time, actor, action);

            Some((self.time, actor, action))
        }
    }

    pub fn event(&mut self, event: Event) {
        self.event_after(Time::default(), event);
    }

    pub fn event_after(&mut self, delay: Time, event: Event) {
        self.event_at(self.time + delay, event);
    }

    pub fn event_at(&mut self, at: Time, event: Event) {
        debug!("[{:?}] schedule at {}: {:?}", self.time, at, event);
        self.event_queue.add(at, event);
    }

    pub fn next_event(&mut self) -> Option<(Time, Event)> {
        if let Some((time, event)) = self.event_queue.next() {
            self.time = time;

            debug!("[{:?}] next event: {:?}", self.time, event);

            Some((time, event))
        } else {
            debug!("[{:?}] no events", self.time);

            None
        }
    }
}
