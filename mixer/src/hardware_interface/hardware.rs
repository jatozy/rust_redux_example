use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

pub type HardwareRef = Rc<RefCell<dyn Hardware>>;

pub trait Hardware {
    fn set_gain(&mut self, gain: f64);
    fn gain(&self) -> f64;
    fn input_level(&self) -> f64;
    fn output_level(&self) -> f64;
    fn id(&self) -> Uuid;
}
