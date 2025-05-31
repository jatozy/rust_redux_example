// actions/mod.rs
pub mod add_new_device;
pub mod set_gain;

use std::any::Any;

#[derive(Debug)]
pub enum ActionType {
    AddNewDevice,
    SetGain,
}

pub trait Action {
    fn action_type(&self) -> ActionType;
    fn as_any(&self) -> &dyn Any;
}
