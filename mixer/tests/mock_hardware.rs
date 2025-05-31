use mixer::hardware_interface::factory::Factory;
use mixer::hardware_interface::hardware::{Hardware, HardwareRef};
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

pub struct MockHardware {
    gain: f64,
    input_level: f64,
    id: Uuid,
}

impl MockHardware {
    pub fn new(id: Uuid) -> HardwareRef {
        Rc::new(RefCell::new(Self {
            gain: 1.0,
            input_level: 0.0,
            id: id,
        }))
    }
}

impl Hardware for MockHardware {
    fn set_gain(&mut self, gain: f64) {
        self.gain = gain;
    }

    fn gain(&self) -> f64 {
        self.gain
    }

    fn input_level(&self) -> f64 {
        self.input_level
    }

    fn output_level(&self) -> f64 {
        self.input_level * self.gain
    }

    fn id(&self) -> Uuid {
        self.id
    }
}

pub struct MockHardwareFactory {
    devices: Vec<HardwareRef>,
}

impl Factory for MockHardwareFactory {
    fn create(&mut self, id: Uuid) -> HardwareRef {
        self.devices.push(MockHardware::new(id));

        self.devices.iter().last().unwrap().clone()
    }
}

impl MockHardwareFactory {
    pub fn new() -> Rc<RefCell<MockHardwareFactory>> {
        Rc::new(RefCell::new(Self {
            devices: Vec::new(),
        }))
    }

    pub fn get_id_of_first_device(&self) -> Uuid {
        self.devices
            .iter()
            .nth(0)
            .expect("There is no hardware device created")
            .borrow()
            .id()
    }

    pub fn get_device(&self, id: Uuid) -> HardwareRef {
        if let Some(device) = self.devices.iter().find(|d| d.borrow().id() == id) {
            device.clone()
        } else {
            panic!("The device does not exist");
        }
    }
}
