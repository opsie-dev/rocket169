use log::debug;

/// Enumeration of every control available on original remote.
pub enum AppleTVControl {
    ButtonHome,
    ButtonMenu,
    ButtonOK,
    ButtonTransportPlayPause,
    ButtonVoiceAssistant,
    ButtonVolumeDown,
    ButtonVolumeUp,
}

/// Remote controller interface.
pub struct AppleTVRemoteControl {}

impl AppleTVRemoteControl {
    pub fn new() -> AppleTVRemoteControl {
        AppleTVRemoteControl {}
    }

    pub fn trigger(&self, control: AppleTVControl) {
        match control {
            AppleTVControl::ButtonHome => {
                debug!("[AppleTV] Home button triggered");
            }
            AppleTVControl::ButtonMenu => {
                debug!("[AppleTV] Menu button triggered");
            }
            AppleTVControl::ButtonOK => {
                debug!("[AppleTV] OK button triggered");
            }
            AppleTVControl::ButtonTransportPlayPause => {
                debug!("[AppleTV] Play/Pause button triggered");
            }
            AppleTVControl::ButtonVoiceAssistant => {
                debug!("[AppleTV] Siri button triggered");
            }
            AppleTVControl::ButtonVolumeDown => {
                debug!("[AppleTV] Volume down button triggered");
            }
            AppleTVControl::ButtonVolumeUp => {
                debug!("[AppleTV] Volume up button triggered");
            }
        }
    }
}
