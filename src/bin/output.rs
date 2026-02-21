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
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
use esp_hal::uart::{Config as UartConfig, Uart};
use log::debug;
use rocket169::event::RemoteControllerInputEvent;

esp_bootloader_esp_idf::esp_app_desc!();

const LED_ACTIVATION_DURATION: Duration = Duration::from_secs(5);

#[derive(Copy, Clone)]
enum RemoteControllerLED {
    Apple,
    Triangle,
    Wimius,
}

struct RemoteControllerOutputs {
    activated_led: Option<RemoteControllerLED>,
    activated_led_instant: Option<Instant>,
    led_apple: Output<'static>,
    led_triangle: Output<'static>,
    led_wimius: Output<'static>,
    uptime: Instant,
}

impl RemoteControllerOutputs {
    fn new(
        led_apple_output: Output<'static>,
        led_triangle_output: Output<'static>,
        led_wimius_output: Output<'static>,
    ) -> RemoteControllerOutputs {
        RemoteControllerOutputs {
            activated_led: None,
            activated_led_instant: None,
            led_apple: led_apple_output,
            led_triangle: led_triangle_output,
            led_wimius: led_wimius_output,
            uptime: Instant::now(),
        }
    }
    fn activate_led(&mut self, led: RemoteControllerLED) {
        match self.activated_led {
            None => {}
            Some(led) => {
                self.deactivate_led(led);
            }
        }
        match led {
            RemoteControllerLED::Apple => self.led_apple.set_high(),
            RemoteControllerLED::Triangle => self.led_triangle.set_high(),
            RemoteControllerLED::Wimius => self.led_wimius.set_high(),
        };
        self.activated_led = Some(led);
        self.activated_led_instant = Some(Instant::now());
    }
    fn deactivate_led(&mut self, led: RemoteControllerLED) {
        match led {
            RemoteControllerLED::Apple => self.led_apple.set_low(),
            RemoteControllerLED::Triangle => self.led_triangle.set_low(),
            RemoteControllerLED::Wimius => self.led_wimius.set_low(),
        };
        self.activated_led = None;
        self.activated_led_instant = None;
    }
    fn on_event_loop_iteration(&mut self) {
        match self.activated_led {
            None => {}
            Some(led) => {
                if self.activated_led_instant.unwrap_or(self.uptime).elapsed()
                    >= LED_ACTIVATION_DURATION
                {
                    self.deactivate_led(led);
                }
            }
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
    let mut uart = Uart::new(peripherals.UART0, UartConfig::default())
        .unwrap()
        .with_rx(peripherals.GPIO1)
        .with_tx(peripherals.GPIO2);
    let output_configuration = OutputConfig::default();
    let mut outputs = RemoteControllerOutputs::new(
        Output::new(peripherals.GPIO12, Level::High, output_configuration),
        Output::new(peripherals.GPIO13, Level::High, output_configuration),
        Output::new(peripherals.GPIO14, Level::High, output_configuration),
    );
    let mut buffer = [0x00; 1];
    loop {
        let n = match uart.read(&mut buffer[..]) {
            Err(error) => {
                debug!("Error while reading message from UART {}", error);
                0
            }
            Ok(n) => n,
        };
        if n > 0 {
            match RemoteControllerInputEvent::from_byte(buffer[0]) {
                RemoteControllerInputEvent::InputSourceApple => {
                    outputs.activate_led(RemoteControllerLED::Apple);
                }
                RemoteControllerInputEvent::InputSourceTriangle => {
                    outputs.activate_led(RemoteControllerLED::Triangle);
                }
                RemoteControllerInputEvent::InputSourceWimius => {
                    outputs.activate_led(RemoteControllerLED::Wimius);
                }
                _ => {}
            };
        }
        outputs.on_event_loop_iteration();
    }
}
