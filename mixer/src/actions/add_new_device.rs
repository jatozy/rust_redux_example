use super::{Action, ActionType};
use std::any::Any;
use uuid::Uuid;

pub struct AddNewDevice {
    pub id: Uuid,
}

impl Action for AddNewDevice {
    fn action_type(&self) -> ActionType {
        ActionType::AddNewDevice
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
