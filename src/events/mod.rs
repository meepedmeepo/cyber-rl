use std::{
    collections::VecDeque,
    slice::Iter,
    sync::{LazyLock, Mutex},
};

use hecs::Entity;

use crate::{State, effects::event_message_trigger, equipment::DamageRecieveEffect};

static EVENT_BUS: LazyLock<Mutex<EventManager>> = LazyLock::new(|| {
    Mutex::new(EventManager {
        event_stack: VecDeque::new(),
    })
});

pub struct EventManager {
    event_stack: VecDeque<Event>,
}

#[derive(Debug, Clone, Copy)]
pub struct Event {
    pub event_type: EventType,
    pub source: Entity,
    pub target: Entity,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum EventType {
    DamageRecieved,
    AttackHit,
    Death,
}

pub fn post_event(event: Event) {
    EVENT_BUS.lock().unwrap().event_stack.push_back(event);
}

pub fn poll_events(state: &mut State) {
    loop {
        match EVENT_BUS.lock().unwrap().event_stack.pop_front() {
            Some(e) => {
                process_event(e, state);
            }

            None => break,
        }
    }
}

fn process_event(e: Event, state: &mut State) {
    match e.event_type {
        EventType::DamageRecieved => {
            let listeners = query_listeners::<DamageRecieveEffect>(state, e.target);

            if listeners.len() > 0 {
                notify_listeners(
                    state,
                    NotifyMessage {
                        e,
                        listeners: listeners.into_vec(),
                    },
                );
            }
        }
        _ => unimplemented!("Fatal Err: Event type not implemented"),
    }
}

struct NotifyMessage {
    e: Event,
    listeners: Vec<Entity>,
}

//TODO! Complete this.
fn notify_listeners(state: &mut State, msg: NotifyMessage) {
    for listener in msg.listeners.iter() {
        //TODO change this system so creator can properly be logged.
        event_message_trigger(msg.e, Some(msg.e.source), *listener);
    }
}

pub trait HasOwner {
    fn get_owner(&self) -> Entity;
}

fn query_listeners<T>(mut state: &mut State, source: Entity) -> Box<[Entity]>
where
    T: HasOwner + hecs::Component,
{
    state
        .world
        .query_mut::<&T>()
        .into_iter()
        .filter_map(|(ent, t)| {
            if t.get_owner() == source {
                Some(ent)
            } else {
                None
            }
        })
        .collect()
}
