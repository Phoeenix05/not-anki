use std::sync::Arc;

use druid::{Data, Lens};

use crate::event::LoggedEvent;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub counter: u32,
    pub events: Arc<Vec<LoggedEvent>>,
}
