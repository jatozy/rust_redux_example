use super::{Action, ActionType};
use std::any::Any;
use uuid::Uuid;

pub struct SetGain {
    pub id: Uuid,
    pub gain: f64,
}

impl Action for SetGain {
    fn action_type(&self) -> ActionType {
        ActionType::SetGain
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
