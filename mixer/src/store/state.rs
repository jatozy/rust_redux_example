use crate::hardware_interface::factory::FactoryRef;
use crate::model::device::Device;
use uuid::Uuid;

pub struct State {
    pub devices: Vec<Device>,
    hardware_factory: FactoryRef,
}

impl State {
    pub fn new(hardware_factory: FactoryRef) -> Self {
        Self {
            devices: Vec::new(),
            hardware_factory: hardware_factory,
        }
    }

    pub fn emplace_device(&mut self, id: Uuid) {
        self.devices
            .push(Device::new(id, self.hardware_factory.clone()));
    }

    pub fn devices(&self) -> &Vec<Device> {
        &self.devices
    }

    pub fn number_devices(&self) -> usize {
        self.devices().iter().count()
    }
}
