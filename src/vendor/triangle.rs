/// Enumeration of every control available on original remote.
pub enum TriangleBoreaBTControl {
    ButtonPower,
    ButtonVolumeMute,
    ButtonVolumeUp,
    ButtonVolumeDown,
    ButtonTransportBackward,
    ButtonTransportForward,
    ButtonTransportPlayPause,
    ButtonSource,
    ButtonEqualizerReset,
    ButtonTrebleUp,
    ButtonTrebleDown,
    ButtonBassUp,
    ButtonBassDown,
}

/// Remote controller interface.
pub struct TriangleBoreaBTRemoteControl {
    // TODO: add IR controller.
}

impl TriangleBoreaBTRemoteControl {
    pub fn new() -> TriangleBoreaBTRemoteControl {
        TriangleBoreaBTRemoteControl {
            // TODO: add attr if provided.
        }
    }

    pub fn trigger(&self, control: TriangleBoreaBTControl) {
        // TODO: implement.
    }
}
