use esp_hal::{
    gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull},
    peripherals::Peripherals,
};

use crate::button::RemoteControllerButtons;
use crate::led::RemoteControllerLEDs;

pub struct RemoteControllerInputs {
    buttons: RemoteControllerButtons,
}

impl RemoteControllerInputs {
    pub fn new(peripherals: Peripherals) -> RemoteControllerInputs {
        let input_configuration = InputConfig::default().with_pull(Pull::Up);
        RemoteControllerInputs {
            buttons: RemoteControllerButtons::new(
                Input::new(
                    peripherals.GPIO0,
                    input_configuration,
                ),
                Input::new(
                    peripherals.GPIO1,
                    input_configuration,
                ),
                Input::new(
                    peripherals.GPIO2,
                    input_configuration,
                ),
                Input::new(
                    peripherals.GPIO3,
                    input_configuration,
                ),
                Input::new(
                    peripherals.GPIO4,
                    input_configuration,
                ),
                Input::new(
                    peripherals.GPIO5,
                    input_configuration,
                ),
                Input::new(
                    peripherals.GPIO12,
                    input_configuration,
                ),
                Input::new(
                    peripherals.GPIO13,
                    input_configuration,
                ),
                Input::new(
                    peripherals.GPIO14,
                    input_configuration,
                ),
                Input::new(
                    peripherals.GPIO15,
                    input_configuration,
                ),
                Input::new(
                    peripherals.GPIO16,
                    input_configuration,
                ),
                Input::new(
                    peripherals.GPIO17,
                    input_configuration,
                ),
                Input::new(
                    peripherals.GPIO18,
                    input_configuration,
                ),
                Input::new(
                    peripherals.GPIO19,
                    input_configuration,
                ),
                Input::new(
                    peripherals.GPIO21,
                    input_configuration,
                ),
                Input::new(
                    peripherals.GPIO22,
                    input_configuration,
                ),
                Input::new(
                    peripherals.GPIO23,
                    input_configuration,
                ),
                Input::new(
                    peripherals.GPIO25,
                    input_configuration,
                ),
                Input::new(
                    peripherals.GPIO26,
                    input_configuration,
                ),
                Input::new(
                    peripherals.GPIO27,
                    input_configuration,
                ),
                Input::new(
                    peripherals.GPIO32,
                    input_configuration,
                ),
                Input::new(
                    peripherals.GPIO33,
                    input_configuration,
                ),
                Input::new(
                    peripherals.GPIO34,
                    input_configuration,
                ),
                Input::new(
                    peripherals.GPIO35,
                    input_configuration,
                ),
                Input::new(
                    peripherals.GPIO36,
                    input_configuration,
                ),
            )
        }
    }
    pub fn poll(&self) -> () {
        // TODO: read every button state and returns matches.
    }
}

pub struct RemoteControllerOutputs<'a> {
    leds: RemoteControllerLEDs<'a>,
}

impl<'a> RemoteControllerOutputs<'a> {
    pub fn new(peripherals: Peripherals) -> RemoteControllerOutputs<'a> {
        let output_configuration = OutputConfig::default();
        let output_1 = Output::new(
            peripherals.GPIO0,
            Level::High,
            output_configuration,
        );
        let output_2 = Output::new(
            peripherals.GPIO1,
            Level::High,
            output_configuration,
        );
        let output_3 = Output::new(
            peripherals.GPIO3,
            Level::High,
            output_configuration,
        );
        RemoteControllerOutputs {
            leds: RemoteControllerLEDs::new(
                output_1,
                output_2,
                output_3,
            )
        }
    }
}
