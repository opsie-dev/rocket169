use esp_hal::gpio::{
    Input,
    InputConfig,
    Pull,
};
use esp_hal::peripherals::Peripherals;
use super::event::RemoteControllerInputEvent;

const REMOTE_CONTROLLER_INPUT_SIZE: usize = 25;

struct RemoteControllerInput {
    event: RemoteControllerInputEvent,
    input: Input<'static>,
}

pub struct RemoteControllerInputs {
    inputs: [RemoteControllerInput; REMOTE_CONTROLLER_INPUT_SIZE],
}

impl RemoteControllerInputs {
    pub fn new(peripherals: Peripherals) -> RemoteControllerInputs {
        let configuration = InputConfig::default().with_pull(Pull::Up);
        return RemoteControllerInputs {
            inputs: [
                RemoteControllerInput {
                    event: RemoteControllerInputEvent::HDMISource1,
                    input: Input::new(
                        peripherals.GPIO1,
                        configuration,
                    ),
                },
                RemoteControllerInput {
                    event: RemoteControllerInputEvent::HDMISource2,
                    input: Input::new(
                        peripherals.GPIO12,
                        configuration,
                    ),
                },
                RemoteControllerInput {
                    event: RemoteControllerInputEvent::HDMISource3,
                    input: Input::new(
                        peripherals.GPIO13,
                        configuration,
                    ),
                },
                RemoteControllerInput {
                    event: RemoteControllerInputEvent::InputSourceApple,
                    input: Input::new(
                        peripherals.GPIO14,
                        configuration,
                    ),
                },
                RemoteControllerInput {
                    event: RemoteControllerInputEvent::InputSourceTriangle,
                    input: Input::new(
                        peripherals.GPIO15,
                        configuration,
                    ),
                },
                RemoteControllerInput {
                    event: RemoteControllerInputEvent::InputSourceWimius,
                    input: Input::new(
                        peripherals.GPIO16,
                        configuration,
                    ),
                },
                RemoteControllerInput {
                    event: RemoteControllerInputEvent::LightOff,
                    input: Input::new(
                        peripherals.GPIO17,
                        configuration,
                    ),
                },
                RemoteControllerInput {
                    event: RemoteControllerInputEvent::LightOn,
                    input: Input::new(
                        peripherals.GPIO18,
                        configuration,
                    ),
                },
                RemoteControllerInput {
                    event: RemoteControllerInputEvent::SoundBassDown,
                    input: Input::new(
                        peripherals.GPIO19,
                        configuration,
                    ),
                },
                RemoteControllerInput {
                    event: RemoteControllerInputEvent::SoundBassUp,
                    input: Input::new(
                        peripherals.GPIO2,
                        configuration,
                    ),
                },
                RemoteControllerInput {
                    event: RemoteControllerInputEvent::SoundEqualizerReset,
                    input: Input::new(
                        peripherals.GPIO21,
                        configuration,
                    ),
                },
                RemoteControllerInput {
                    event: RemoteControllerInputEvent::SoundMute,
                    input: Input::new(
                        peripherals.GPIO22,
                        configuration,
                    ),
                },
                RemoteControllerInput {
                    event: RemoteControllerInputEvent::SoundSource,
                    input: Input::new(
                        peripherals.GPIO23,
                        configuration,
                    ),
                },
                RemoteControllerInput {
                    event: RemoteControllerInputEvent::SoundTrebleDown,
                    input: Input::new(
                        peripherals.GPIO25,
                        configuration,
                    ),
                },
                RemoteControllerInput {
                    event: RemoteControllerInputEvent::SoundTrebleUp,
                    input: Input::new(
                        peripherals.GPIO26,
                        configuration,
                    ),
                },
                RemoteControllerInput {
                    event: RemoteControllerInputEvent::SoundVolumeDown,
                    input: Input::new(
                        peripherals.GPIO27,
                        configuration,
                    ),
                },
                RemoteControllerInput {
                    event: RemoteControllerInputEvent::SoundVolumeUp,
                    input: Input::new(
                        peripherals.GPIO3,
                        configuration,
                    ),
                },
                RemoteControllerInput {
                    event: RemoteControllerInputEvent::TelevisionBack,
                    input: Input::new(
                        peripherals.GPIO32,
                        configuration,
                    ),
                },
                RemoteControllerInput {
                    event: RemoteControllerInputEvent::TelevisionDown,
                    input: Input::new(
                        peripherals.GPIO33,
                        configuration,
                    ),
                },
                RemoteControllerInput {
                    event: RemoteControllerInputEvent::TelevisionHome,
                    input: Input::new(
                        peripherals.GPIO34,
                        configuration,
                    ),
                },
                RemoteControllerInput {
                    event: RemoteControllerInputEvent::TelevisionLeft,
                    input: Input::new(
                        peripherals.GPIO35,
                        configuration,
                    ),
                },
                RemoteControllerInput {
                    event: RemoteControllerInputEvent::TelevisionMenu,
                    input: Input::new(
                        peripherals.GPIO36,
                        configuration,
                    ),
                },
                RemoteControllerInput {
                    event: RemoteControllerInputEvent::TelevisionPlayPause,
                    input: Input::new(
                        peripherals.GPIO37,
                        configuration,
                    ),
                },
                RemoteControllerInput {
                    event: RemoteControllerInputEvent::TelevisionRight,
                    input: Input::new(
                        peripherals.GPIO38,
                        configuration,
                    ),
                },
                RemoteControllerInput {
                    event: RemoteControllerInputEvent::TelevisionUp,
                    input: Input::new(
                        peripherals.GPIO39,
                        configuration,
                    ),
                },
            ],
        }
    }
    pub fn poll(&self) -> Option<RemoteControllerInputEvent> {
        for i in 0..REMOTE_CONTROLLER_INPUT_SIZE {
            if self.inputs[i].input.is_low() {
                return Some(self.inputs[i].event)
            }
        }
        None
    }
}
