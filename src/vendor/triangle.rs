use log::debug;

/// Enumeration of every control available on original remote.
pub enum TriangleBoreaBTControl {
    ButtonBassDown,
    ButtonBassUp,
    ButtonEqualizerReset,
    ButtonPower,
    ButtonSource,
    ButtonTransportBackward,
    ButtonTransportForward,
    ButtonTransportPlayPause,
    ButtonTrebleDown,
    ButtonTrebleUp,
    ButtonVolumeDown,
    ButtonVolumeMute,
    ButtonVolumeUp,
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
        match control {
            TriangleBoreaBTControl::ButtonBassDown => {
                debug!("[TriangleBoreaBT] Bass down button triggered");
            }
            TriangleBoreaBTControl::ButtonBassUp => {
                debug!("[TriangleBoreaBT] Bass up button triggered");
            }
            TriangleBoreaBTControl::ButtonEqualizerReset => {
                debug!("[TriangleBoreaBT] EQ reset button triggered");
            }
            TriangleBoreaBTControl::ButtonPower => {
                debug!("[TriangleBoreaBT] Power button triggered");
            }
            TriangleBoreaBTControl::ButtonSource => {
                debug!("[TriangleBoreaBT] Source button triggered");
            }
            TriangleBoreaBTControl::ButtonTransportBackward => {
                debug!("[TriangleBoreaBT] Transport backward button triggered");
            }
            TriangleBoreaBTControl::ButtonTransportForward => {
                debug!("[TriangleBoreaBT] Transport forward button triggered");
            }
            TriangleBoreaBTControl::ButtonTransportPlayPause => {
                debug!("[TriangleBoreaBT] Transport play/pause button triggered");
            }
            TriangleBoreaBTControl::ButtonTrebleDown => {
                debug!("[TriangleBoreaBT] Treble down button triggered");
            }
            TriangleBoreaBTControl::ButtonTrebleUp => {
                debug!("[TriangleBoreaBT] Treble up button triggered");
            }
            TriangleBoreaBTControl::ButtonVolumeDown => {
                debug!("[TriangleBoreaBT] Volume down button triggered");
            }
            TriangleBoreaBTControl::ButtonVolumeMute => {
                debug!("[TriangleBoreaBT] Volume mute button triggered");
            }
            TriangleBoreaBTControl::ButtonVolumeUp => {
                debug!("[TriangleBoreaBT] Volume up button triggered");
            }
        }
    }
}
