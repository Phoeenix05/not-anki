#![allow(dead_code)]

use druid::{Data, Event, KeyEvent, Modifiers};

#[derive(Debug, Clone, Copy, Data, PartialEq)]
pub enum EventType {
    KeyDown,
    KeyUp,
}

#[derive(Clone, Data)]
pub struct LoggedEvent {
    typ: EventType,
    #[data(ignore)]
    key: Option<KeyEvent>,
}

impl LoggedEvent {
    pub fn try_from_event(event: &Event) -> Option<Self> {
        let to_log = match event {
            Event::KeyDown(key) => Some((EventType::KeyDown, Some(key.clone()))),
            _ => None,
        };

        to_log.map(|(typ, key)| LoggedEvent { typ, key })
    }

    pub fn key(&self) -> String {
        self.key
            .as_ref()
            .map(|k| k.key.to_string())
            .unwrap_or_default()
    }

    pub fn code(&self) -> String {
        self.key
            .as_ref()
            .map(|k| k.code.to_string())
            .unwrap_or_default()
    }

    pub fn modifiers(&self) -> String {
        self.key
            .as_ref()
            .map(|k| modifiers_string(k.mods))
            // .or_else(|| /* mouse events */)
            .unwrap_or_default()
    }
}

macro_rules! mods {
    ($f:expr, $result:expr, $name:literal) => {
        if $f {
            $result.push_str($name)
        }
    };
}

fn modifiers_string(mods: Modifiers) -> String {
    let mut result = String::new();

    mods!(mods.shift(), result, "Shift ");
    mods!(mods.ctrl(), result, "Ctrl ");
    mods!(mods.alt(), result, "Alt ");
    mods!(mods.meta(), result, "Meta ");
    mods!(mods.is_empty(), result, "None");

    result
}
