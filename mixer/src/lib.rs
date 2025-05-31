pub mod hardware_interface;

mod actions;
mod event_handler;
mod model;
mod store;

use event_handler::event_handler::EventHandler;
use hardware_interface::factory::FactoryRef;
use std::cell::RefCell;
use std::rc::Rc;
use store::store::Store;

pub struct Application {
    pub store: Rc<RefCell<Store>>,
    pub event_handler: EventHandler,
}

impl Application {
    pub fn new(hardware_factory: FactoryRef) -> Self {
        let store = Rc::new(RefCell::new(Store::new(hardware_factory)));
        let event_handler = EventHandler::new(Rc::clone(&store));
        Application {
            store,
            event_handler,
        }
    }

    pub fn run(&mut self) {
        println!(
            "Hello store with {} elements",
            self.store.borrow().state().number_devices()
        );
        self.store.borrow_mut().emplace_device();
        println!(
            "Hello store with {} elements",
            self.store.borrow().state().number_devices()
        );
        self.event_handler.add_new_device();
        println!(
            "Hello store with {} elements",
            self.store.borrow().state().number_devices()
        );
    }
}
