use esp_hal::gpio::{
    Level,
    Output,
    OutputConfig,
};
use esp_hal::peripherals::Peripherals;

pub enum RemoteControllerLED {
    InputSourceApple,
    InputSourceTriangle,
    InputSourceWimius,
}

pub struct RemoteControllerOutputs {
    led_apple: Output<'static>,
    led_triangle: Output<'static>,
    led_wimius: Output<'static>,
}

impl RemoteControllerOutputs {
    pub fn new(peripherals: Peripherals) -> RemoteControllerOutputs {
        let configuration = OutputConfig::default();
        RemoteControllerOutputs {
            led_apple: Output::new(
                peripherals.GPIO0,
                Level::High,
                configuration,
            ),
            led_triangle: Output::new(
                peripherals.GPIO1,
                Level::High,
                configuration,
            ),
            led_wimius: Output::new(
                peripherals.GPIO2,
                Level::High,
                configuration,
            ),
        }
    }
    pub fn activate(&mut self, led: RemoteControllerLED) -> () {
        let mut led_output = match led {
            RemoteControllerLED::InputSourceApple => self.led_apple,
            RemoteControllerLED::InputSourceTriangle => &self.led_triangle,
            RemoteControllerLED::InputSourceWimius => &self.led_wimius,
        };
        led_output.set_high();       
    }
}
