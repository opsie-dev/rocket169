use log::debug;

/// Enumeration of every control available on original remote.
pub enum WimiusS27Control {
    ButtonDown,
    ButtonHome,
    ButtonLeft,
    ButtonMenu,
    ButtonOK,
    ButtonPower,
    ButtonReturn,
    ButtonRight,
    ButtonSource,
    ButtonUp,
    ButtonVolumeDown,
    ButtonVolumeMute,
    ButtonVolumeUp,
}

/// Remote controller interface.
pub struct WimiusS27RemoteControl {}

impl WimiusS27RemoteControl {
    pub fn new() -> WimiusS27RemoteControl {
        WimiusS27RemoteControl {}
    }

    pub fn trigger(&self, control: WimiusS27Control) {
        match control {
            WimiusS27Control::ButtonDown => {
                debug!("[WimiusS27] Down button triggered");
            }
            WimiusS27Control::ButtonHome => {
                debug!("[WimiusS27] Home button triggered");
            }
            WimiusS27Control::ButtonLeft => {
                debug!("[WimiusS27] Left button triggered");
            }
            WimiusS27Control::ButtonMenu => {
                debug!("[WimiusS27] Menu button triggered");
            }
            WimiusS27Control::ButtonOK => {
                debug!("[WimiusS27] OK button triggered");
            }
            WimiusS27Control::ButtonPower => {
                debug!("[WimiusS27] Power button triggered");
            }
            WimiusS27Control::ButtonReturn => {
                debug!("[WimiusS27] Return button triggered");
            }
            WimiusS27Control::ButtonRight => {
                debug!("[WimiusS27] Right button triggered");
            }
            WimiusS27Control::ButtonSource => {
                debug!("[WimiusS27] Source button triggered");
            }
            WimiusS27Control::ButtonUp => {
                debug!("[WimiusS27] Up button triggered");
            }
            WimiusS27Control::ButtonVolumeDown => {
                debug!("[WimiusS27] Volume down button triggered");
            }
            WimiusS27Control::ButtonVolumeMute => {
                debug!("[WimiusS27] Volume mute button triggered");
            }
            WimiusS27Control::ButtonVolumeUp => {
                debug!("[WimiusS27] Volume up button triggered");
            }
        }
    }
}
