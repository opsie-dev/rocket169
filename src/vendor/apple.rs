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
pub struct AppleTVRemoteControl {
    // TODO: add IR controller.
}

impl AppleTVRemoteControl {
    pub fn new() -> AppleTVRemoteControl {
        AppleTVRemoteControl {
            // TODO: add attr if provided.
        }
    }

    pub fn trigger(&self, control: AppleTVControl) {
        // TODO: implement.
    }
}
