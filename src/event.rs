#[derive(Copy, Clone)]
pub enum RemoteControllerInputEvent {
    HDMISource1 = 0x01,
    HDMISource2 = 0x02,
    HDMISource3 = 0x03,
    InputSourceApple = 0x04,
    InputSourceTriangle = 0x05,
    InputSourceWimius = 0x06,
    LightOff = 0x07,
    LightOn = 0x08,
    SoundBassDown = 0x09,
    SoundBassUp = 0x10,
    SoundEqualizerReset = 0x11,
    SoundMute = 0x12,
    SoundSource = 0x13,
    SoundTrebleDown = 0x14,
    SoundTrebleUp = 0x15,
    SoundVolumeDown = 0x16,
    SoundVolumeUp = 0x17,
    TelevisionBack = 0x18,
    TelevisionDown = 0x19,
    TelevisionHome = 0x20,
    TelevisionLeft = 0x21,
    TelevisionMenu = 0x22,
    TelevisionPlayPause = 0x23,
    TelevisionRight = 0x24,
    TelevisionUp = 0x25,
}

impl RemoteControllerInputEvent {
    pub fn from_uart_message(message: &[u8]) -> RemoteControllerInputEvent {
        // TODO: convert from UART message.
        RemoteControllerInputEvent::HDMISource1
    }
    pub fn as_uart_message(&self) -> &[u8] {
        // TODO: convert event as UART message.
        b"\n"
    }
}
