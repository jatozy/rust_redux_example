use mixer::hardware_interface::hardware::{Hardware, HardwareRef};
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use uuid::Uuid;

pub struct TimerHardware {
    gain: f64,
    input_level: Arc<Mutex<f64>>,
    id: Uuid,
}

impl TimerHardware {
    pub fn new(id: Uuid) -> HardwareRef {
        let input_level = Arc::new(Mutex::new(0.0));
        let input_level_clone = input_level.clone();

        // Spawn a background thread to update input_level every 200ms
        thread::spawn(move || {
            let mut rng = rand::thread_rng();
            loop {
                let new_value = rng.gen_range(0.0..=1.0);
                if let Ok(mut level) = input_level_clone.lock() {
                    *level = new_value;
                }
                thread::sleep(Duration::from_millis(200));
            }
        });

        Rc::new(RefCell::new(Self {
            gain: 1.0,
            input_level,
            id: id,
        }))
    }
}

impl Hardware for TimerHardware {
    fn set_gain(&mut self, gain: f64) {
        self.gain = gain;
    }

    fn gain(&self) -> f64 {
        self.gain
    }

    fn input_level(&self) -> f64 {
        self.input_level.lock().map(|v| *v).unwrap_or(0.0)
    }

    fn output_level(&self) -> f64 {
        self.input_level() * self.gain
    }

    fn id(&self) -> Uuid {
        self.id
    }
}
