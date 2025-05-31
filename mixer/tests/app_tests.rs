use std::{cell::RefCell, rc::Rc};

use mixer::Application;

mod mock_hardware;
use mock_hardware::MockHardwareFactory;

struct Runtime {
    pub hardware: Rc<RefCell<MockHardwareFactory>>,
    pub dut: Application,
}

impl Runtime {
    pub fn new() -> Runtime {
        let hardware = MockHardwareFactory::new();
        let dut = Application::new(hardware.clone());

        Self {
            hardware: hardware.clone(),
            dut: dut,
        }
    }
}

#[test]
fn test_event_handler_integration() {
    let runtime = Runtime::new();

    runtime.dut.event_handler.add_new_device();
    runtime.dut.event_handler.add_new_device();
    runtime.dut.event_handler.add_new_device();

    const EXPECTED_NUMBER_DEVICES: usize = 3;
    assert_eq!(
        runtime.dut.store.borrow().state().number_devices(),
        EXPECTED_NUMBER_DEVICES
    );
}

#[test]
fn test_set_gain_of_device() {
    let runtime = Runtime::new();

    runtime.dut.event_handler.add_new_device();

    let device_id = runtime.hardware.borrow().get_id_of_first_device();

    const NEW_GAIN: f64 = 1.5;

    runtime.dut.event_handler.set_gain(device_id, NEW_GAIN);

    let device = runtime.hardware.borrow().get_device(device_id);

    assert_eq!(device.borrow().gain(), NEW_GAIN);
}
