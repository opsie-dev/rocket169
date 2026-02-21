/// Enumeration of every control available on original remote.
pub enum WimiusS27Control {
    ButtonPower,
    ButtonMenu,
    ButtonSource,
    ButtonDown,
    ButtonLeft,
    ButtonRight,
    ButtonUp,
    ButtonOK,
    ButtonHome,
    ButtonReturn,
    ButtonVolumeDown,
    ButtonVolumeUp,
    ButtonVolumeMute,
}

/// Remote controller interface.
pub struct WimiusS27RemoteControl {}

impl WimiusS27RemoteControl {
    pub fn new() -> WimiusS27RemoteControl {
        WimiusS27RemoteControl {}
    }

    pub fn trigger(&self, control: WimiusS27Control) {
        // TODO: implement.
    }
}
