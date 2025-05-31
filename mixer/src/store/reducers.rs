use super::state::State;
use crate::actions::{add_new_device::AddNewDevice, set_gain::SetGain};

pub fn add_new_device(action: &AddNewDevice, state: &mut State) {
    state.emplace_device(action.id);
}

pub fn set_gain(action: &SetGain, state: &mut State) {
    if let Some(device) = state.devices.iter_mut().find(|d| *d.id() == action.id) {
        device.set_gain(action.gain);
    }
}
