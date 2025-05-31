use uuid::Uuid;

use crate::hardware_interface::hardware::HardwareRef;
use std::cell::RefCell;
use std::rc::Rc;

pub type FactoryRef = Rc<RefCell<dyn Factory>>;

pub trait Factory {
    fn create(&mut self, id: Uuid) -> HardwareRef;
}
