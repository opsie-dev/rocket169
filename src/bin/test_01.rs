// Test #01: three LED + three selector push button.
#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]
use esp_backtrace as _;
use esp_hal::Config as HALConfig;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::main;
use log::debug;

esp_bootloader_esp_idf::esp_app_desc!();

#[derive(Copy, Clone)]
enum RemoteSource {
    Apple,
    Triangle,
    Wimius,
}

struct RemoteInputs {
    button_apple: Input<'static>,
    button_triangle: Input<'static>,
    button_wimius: Input<'static>,
}

impl RemoteInputs {
    fn poll(&self) -> Option<RemoteSource> {
        if self.button_apple.is_high() {
            return Some(RemoteSource::Apple);
        }
        if self.button_triangle.is_high() {
            return Some(RemoteSource::Triangle);
        }
        if self.button_wimius.is_high() {
            return Some(RemoteSource::Wimius);
        }
        None
    }
}

struct RemoteOutputs {
    activated_source: Option<RemoteSource>,
    led_apple: Output<'static>,
    led_triangle: Output<'static>,
    led_wimius: Output<'static>,
}

impl RemoteOutputs {
    fn activate_led(&mut self, source: RemoteSource) {
        match self.activated_source {
            None => {}
            Some(source) => {
                self.deactivate_led(source);
            }
        }
        match source {
            RemoteSource::Apple => self.led_apple.set_high(),
            RemoteSource::Triangle => self.led_triangle.set_high(),
            RemoteSource::Wimius => self.led_wimius.set_high(),
        };
        self.activated_source = Some(source);
    }
    fn deactivate_led(&mut self, source: RemoteSource) {
        match source {
            RemoteSource::Apple => self.led_apple.set_low(),
            RemoteSource::Triangle => self.led_triangle.set_low(),
            RemoteSource::Wimius => self.led_wimius.set_low(),
        };
    }
}

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();
    let hal_configuration = HALConfig::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(hal_configuration);
    let input_configuration = InputConfig::default().with_pull(Pull::Up);
    let output_configuration = OutputConfig::default();
    let inputs = RemoteInputs {
        button_apple: Input::new(peripherals.GPIO36, input_configuration),
        button_triangle: Input::new(peripherals.GPIO39, input_configuration),
        button_wimius: Input::new(peripherals.GPIO34, input_configuration),
    };
    let mut outputs = RemoteOutputs {
        activated_source: None,
        led_apple: Output::new(peripherals.GPIO32, Level::Low, output_configuration),
        led_triangle: Output::new(peripherals.GPIO33, Level::Low, output_configuration),
        led_wimius: Output::new(peripherals.GPIO25, Level::Low, output_configuration),
    };
    loop {
        match inputs.poll() {
            None => {}
            Some(source) => {
                match source {
                    RemoteSource::Apple => {
                        debug!("AppleTV input source selected, activating LED");
                    }
                    RemoteSource::Triangle => {
                        debug!("TriangleBoreaBT input source selected, activating LED");
                    }
                    RemoteSource::Wimius => {
                        debug!("WimiusS27 input source selected, activating LED");
                    }
                }
                outputs.activate_led(source);
            }
        };
    }
}
