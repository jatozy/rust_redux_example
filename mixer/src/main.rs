mod timer_hardware;

use mixer::Application;
use std::cell::RefCell;
use std::rc::Rc;
use timer_hardware::TimerHardwareFactory;

fn main() {
    let hardware_factory = Rc::new(RefCell::new(TimerHardwareFactory));
    let mut app = Application::new(hardware_factory.clone());

    app.run();
}
