use std::{
    collections::VecDeque,
    sync::{LazyLock, Mutex},
};

use hecs::Entity;

use crate::{State, equipment::DamageRecieveEffect};

static EVENT_BUS: LazyLock<Mutex<EventManager>> = LazyLock::new(|| {
    Mutex::new(EventManager {
        event_stack: VecDeque::new(),
    })
});

pub struct EventManager {
    event_stack: VecDeque<Event>,
}

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
            let listeners = state
                .world
                .query_mut::<&DamageRecieveEffect>()
                .into_iter()
                .filter_map(|(ent, dmg)| {
                    if e.target == dmg.owner {
                        Some(ent)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            if listeners.len() > 0 {
                notify_listeners(state, NotifyMessage { e, listeners });
            }
        }
        _ => unimplemented!("Fatal Err: Event type not implemented"),
    }
}

struct NotifyMessage {
    e: Event,
    listeners: Vec<Entity>,
}

fn notify_listeners(state: &mut State, msg: NotifyMessage) {
    for listener in msg.listeners.iter() {
        event_message_trigger
    }
}
