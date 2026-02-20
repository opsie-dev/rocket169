use rocket169::device::RemoteController;

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
pub struct AppleTVRemoteControl<C>
where
    C: IRController,
{
    ir_controller: &C,
}

impl AppleTVRemoteControl<C>
where
    C: IRController
{
    pub fn new(
        ir_controller: &C,
    ) -> AppleTVRemoteControl<C> {
        AppleTVRemoteControl {
            ir_controller: ir_controller,
        }
    }

    pub fn trigger(&self, control: AppleTVControl) {
        // TODO: implement.
    }
}
