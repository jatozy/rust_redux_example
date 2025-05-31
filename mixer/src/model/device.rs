use crate::hardware_interface::factory::FactoryRef;
use crate::hardware_interface::hardware::HardwareRef;
use uuid::Uuid;

pub struct Device {
    id: Uuid,
    hardware: HardwareRef,
}

impl Device {
    pub fn new(id: Uuid, hardware_factory: FactoryRef) -> Self {
        Self {
            id: id,
            hardware: hardware_factory.borrow_mut().create(id),
        }
    }

    pub fn output_level(&self) -> f64 {
        self.hardware.borrow().input_level() * self.hardware.borrow().gain()
    }

    pub fn set_gain(&mut self, gain: f64) {
        self.hardware.borrow_mut().set_gain(gain);
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }
}
