use crate::actions::add_new_device::AddNewDevice;
use crate::actions::set_gain::SetGain;
use crate::actions::{Action, ActionType};
use crate::hardware_interface::factory::FactoryRef;
use crate::store::state::State;

pub struct Store {
    state: State,
}

impl Store {
    pub fn new(hardware_factory: FactoryRef) -> Self {
        let state = State::new(hardware_factory);
        Self { state }
    }

    pub fn emplace_device(&mut self) {
        self.state.emplace_device(uuid::Uuid::new_v4());
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn execute(&mut self, action: Box<dyn Action>) {
        match action.action_type() {
            ActionType::AddNewDevice => {
                if let Some(add_device) = action.as_any().downcast_ref::<AddNewDevice>() {
                    super::reducers::add_new_device(add_device, &mut self.state);
                }
            }
            ActionType::SetGain => {
                if let Some(set_gain) = action.as_any().downcast_ref::<SetGain>() {
                    super::reducers::set_gain(set_gain, &mut self.state);
                }
            }
        }
    }
}
