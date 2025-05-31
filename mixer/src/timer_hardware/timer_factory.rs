use super::timer_impl::TimerHardware;
use mixer::hardware_interface::factory::Factory;
use mixer::hardware_interface::hardware::HardwareRef;
use uuid::Uuid;

pub struct TimerHardwareFactory;

impl Factory for TimerHardwareFactory {
    fn create(&mut self, id: Uuid) -> HardwareRef {
        TimerHardware::new(id)
    }
}
