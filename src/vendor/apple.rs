/// Enumeration of every control available on original remote.
pub enum AppleTVControl {
    ButtonOK,
    ButtonMenu,
    ButtonHome,
    ButtonVoiceAssistant,
    ButtonTransportPlayPause,
    ButtonVolumeUp,
    ButtonVolumeDown,
}

/// Remote controller interface.
pub struct AppleTVRemoteControl {}

impl AppleTVRemoteControl {
    pub fn new() -> AppleTVRemoteControl {
        AppleTVRemoteControl {}
    }

    pub fn trigger(&self, control: AppleTVControl) {
        // TODO: implement.
    }
}
