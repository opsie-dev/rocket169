use esp_hal::gpio::Output;

struct RemoteControllerLED<'a> {
    output: Output<'a>,
}

impl<'a> RemoteControllerLED<'a> {
    fn new(output: Output<'a>) -> RemoteControllerLED<'a> {
        RemoteControllerLED {
            output: output,
        }
    }
    fn trigger(&mut self) -> () {
        if self.output.is_set_high() {
            self.output.set_low();
        }
        // TODO: add delay for shutdown.
    }
}

pub struct RemoteControllerLEDs<'a> {
    apple: RemoteControllerLED<'a>,
    triangle: RemoteControllerLED<'a>,
    wimius: RemoteControllerLED<'a>,
}

impl<'a> RemoteControllerLEDs<'a> {
    pub fn new(
        output_apple: Output<'a>,
        output_triangle: Output<'a>,
        output_wimius: Output<'a>,
    ) -> RemoteControllerLEDs<'a> {
        RemoteControllerLEDs {
            apple: RemoteControllerLED::new(output_apple),
            triangle: RemoteControllerLED::new(output_triangle),
            wimius: RemoteControllerLED::new(output_wimius),
        }
    }
}
