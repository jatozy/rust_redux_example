use uuid::Uuid;

use crate::actions::add_new_device::AddNewDevice;
use crate::actions::set_gain::SetGain;
use crate::store::store::Store;
use std::cell::RefCell;
use std::rc::Rc;

pub struct EventHandler {
    store: Rc<RefCell<Store>>,
}

impl EventHandler {
    pub fn new(store: Rc<RefCell<Store>>) -> Self {
        Self { store: store }
    }

    pub fn add_new_device(&self) {
        self.store
            .borrow_mut()
            .execute(Box::new(AddNewDevice { id: Uuid::new_v4() }));
    }

    pub fn set_gain(&self, id: Uuid, gain: f64) {
        self.store
            .borrow_mut()
            .execute(Box::new(SetGain { id: id, gain: gain }));
    }
}
