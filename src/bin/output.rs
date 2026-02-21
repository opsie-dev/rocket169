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
use rocket169::vendor::apple::{AppleTVControl, AppleTVRemoteControl};
use rocket169::vendor::triangle::{TriangleBoreaBTControl, TriangleBoreaBTRemoteControl};
use rocket169::vendor::ugreen::{UGreenAW504Control, UGreenAW504RemoteControl};
use rocket169::vendor::wimius::{WimiusS27Control, WimiusS27RemoteControl};

esp_bootloader_esp_idf::esp_app_desc!();

const LED_ACTIVATION_DURATION: Duration = Duration::from_secs(5);

#[derive(Copy, Clone)]
enum RemoteControllerSource {
    Apple,
    Triangle,
    Wimius,
}

#[derive(Copy, Clone)]
enum RemoteControllerLED {
    Apple,
    Triangle,
    Wimius,
}

struct RemoteControllerOutputs {
    activated_led: Option<RemoteControllerLED>,
    activated_led_instant: Option<Instant>,
    activated_source: RemoteControllerSource,
    led_apple: Output<'static>,
    led_triangle: Output<'static>,
    led_wimius: Output<'static>,
    remote_control_apple: AppleTVRemoteControl,
    remote_control_triangle: TriangleBoreaBTRemoteControl,
    remote_control_ugreen: UGreenAW504RemoteControl,
    remote_control_wimius: WimiusS27RemoteControl,
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
            activated_source: RemoteControllerSource::Triangle,
            led_apple: led_apple_output,
            led_triangle: led_triangle_output,
            led_wimius: led_wimius_output,
            remote_control_apple: AppleTVRemoteControl::new(),
            remote_control_triangle: TriangleBoreaBTRemoteControl::new(),
            remote_control_ugreen: UGreenAW504RemoteControl::new(),
            remote_control_wimius: WimiusS27RemoteControl::new(),
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
    fn activate_source(&mut self, source: RemoteControllerSource) {
        self.activated_source = source;
        match source {
            RemoteControllerSource::Apple => self.activate_led(RemoteControllerLED::Apple),
            RemoteControllerSource::Triangle => self.activate_led(RemoteControllerLED::Triangle),
            RemoteControllerSource::Wimius => self.activate_led(RemoteControllerLED::Wimius),
        };
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

    fn on_input_event(&mut self, event: RemoteControllerInputEvent) {
        match event {
            // NOTE: Handling HDMI source events.
            RemoteControllerInputEvent::HDMISource1 => {
                self.remote_control_ugreen
                    .trigger(UGreenAW504Control::Button1);
            }
            RemoteControllerInputEvent::HDMISource2 => {
                self.remote_control_ugreen
                    .trigger(UGreenAW504Control::Button2);
            }
            RemoteControllerInputEvent::HDMISource3 => {
                self.remote_control_ugreen
                    .trigger(UGreenAW504Control::Button3);
            }
            // NOTE: Handling input source change events.
            RemoteControllerInputEvent::InputSourceApple => {
                self.activate_source(RemoteControllerSource::Apple);
            }
            RemoteControllerInputEvent::InputSourceTriangle => {
                self.activate_source(RemoteControllerSource::Triangle);
            }
            RemoteControllerInputEvent::InputSourceWimius => {
                self.activate_source(RemoteControllerSource::Wimius);
            }
            // NOTE: Handling light input events.
            RemoteControllerInputEvent::LightOff => {
                // TODO: implements.
            }
            RemoteControllerInputEvent::LightOn => {
                // TODO: implements.
            }
            // NOTE: Triangle sound only events.
            RemoteControllerInputEvent::SoundBassDown => {
                self.activate_source(RemoteControllerSource::Triangle);
                self.remote_control_triangle
                    .trigger(TriangleBoreaBTControl::ButtonBassDown);
            }
            RemoteControllerInputEvent::SoundBassUp => {
                self.activate_source(RemoteControllerSource::Triangle);
                self.remote_control_triangle
                    .trigger(TriangleBoreaBTControl::ButtonBassUp);
            }
            RemoteControllerInputEvent::SoundEqualizerReset => {
                self.activate_source(RemoteControllerSource::Triangle);
                self.remote_control_triangle
                    .trigger(TriangleBoreaBTControl::ButtonEqualizerReset);
            }
            RemoteControllerInputEvent::SoundSource => {
                self.activate_source(RemoteControllerSource::Triangle);
                self.remote_control_triangle
                    .trigger(TriangleBoreaBTControl::ButtonSource);
            }
            RemoteControllerInputEvent::SoundTrebleDown => {
                self.activate_source(RemoteControllerSource::Triangle);
                self.remote_control_triangle
                    .trigger(TriangleBoreaBTControl::ButtonTrebleDown);
            }
            RemoteControllerInputEvent::SoundTrebleUp => {
                self.activate_source(RemoteControllerSource::Triangle);
                self.remote_control_triangle
                    .trigger(TriangleBoreaBTControl::ButtonTrebleUp);
            }
            // NOTE: Multi sound source events.
            // TODO: handle auto switch for source + led reactivation.
            RemoteControllerInputEvent::SoundMute => {
                match self.activated_source {
                    RemoteControllerSource::Triangle => self
                        .remote_control_triangle
                        .trigger(TriangleBoreaBTControl::ButtonVolumeMute),
                    RemoteControllerSource::Wimius => self
                        .remote_control_wimius
                        .trigger(WimiusS27Control::ButtonVolumeMute),
                    _ => {}
                };
            }
            RemoteControllerInputEvent::SoundVolumeDown => {
                match self.activated_source {
                    RemoteControllerSource::Apple => self
                        .remote_control_apple
                        .trigger(AppleTVControl::ButtonVolumeDown),
                    RemoteControllerSource::Triangle => self
                        .remote_control_triangle
                        .trigger(TriangleBoreaBTControl::ButtonVolumeDown),
                    RemoteControllerSource::Wimius => self
                        .remote_control_wimius
                        .trigger(WimiusS27Control::ButtonVolumeDown),
                };
            }
            RemoteControllerInputEvent::SoundVolumeUp => {
                match self.activated_source {
                    RemoteControllerSource::Apple => self
                        .remote_control_apple
                        .trigger(AppleTVControl::ButtonVolumeUp),
                    RemoteControllerSource::Triangle => self
                        .remote_control_triangle
                        .trigger(TriangleBoreaBTControl::ButtonVolumeUp),
                    RemoteControllerSource::Wimius => self
                        .remote_control_wimius
                        .trigger(WimiusS27Control::ButtonVolumeUp),
                };
            }
            RemoteControllerInputEvent::TelevisionBack => {
                self.activate_source(RemoteControllerSource::Wimius);
                self.remote_control_wimius
                    .trigger(WimiusS27Control::ButtonReturn);
            }
            RemoteControllerInputEvent::TelevisionDown => {
                // TODO: add Apple TV.
                match self.activated_source {
                    RemoteControllerSource::Wimius => self
                        .remote_control_wimius
                        .trigger(WimiusS27Control::ButtonDown),
                    _ => (),
                };
            }
            RemoteControllerInputEvent::TelevisionHome => {
                match self.activated_source {
                    RemoteControllerSource::Wimius => self
                        .remote_control_wimius
                        .trigger(WimiusS27Control::ButtonHome),
                    _ => {
                        self.activate_source(RemoteControllerSource::Apple);
                        self.remote_control_apple
                            .trigger(AppleTVControl::ButtonHome);
                    }
                };
            }
            RemoteControllerInputEvent::TelevisionLeft => {
                match self.activated_source {
                    RemoteControllerSource::Triangle => self
                        .remote_control_triangle
                        .trigger(TriangleBoreaBTControl::ButtonTransportBackward),
                    RemoteControllerSource::Wimius => self
                        .remote_control_wimius
                        .trigger(WimiusS27Control::ButtonLeft),
                    _ => (),
                };
            }
            RemoteControllerInputEvent::TelevisionMenu => {
                match self.activated_source {
                    RemoteControllerSource::Wimius => self
                        .remote_control_wimius
                        .trigger(WimiusS27Control::ButtonMenu),
                    _ => {
                        self.activate_source(RemoteControllerSource::Apple);
                        self.remote_control_apple
                            .trigger(AppleTVControl::ButtonMenu);
                    }
                };
            }
            RemoteControllerInputEvent::TelevisionPlayPause => {
                match self.activated_source {
                    RemoteControllerSource::Apple => self
                        .remote_control_apple
                        .trigger(AppleTVControl::ButtonTransportPlayPause),
                    RemoteControllerSource::Triangle => self
                        .remote_control_triangle
                        .trigger(TriangleBoreaBTControl::ButtonTransportPlayPause),
                    RemoteControllerSource::Wimius => self
                        .remote_control_wimius
                        .trigger(WimiusS27Control::ButtonOK),
                };
            }
            RemoteControllerInputEvent::TelevisionRight => {
                match self.activated_source {
                    RemoteControllerSource::Triangle => self
                        .remote_control_triangle
                        .trigger(TriangleBoreaBTControl::ButtonTransportForward),
                    RemoteControllerSource::Wimius => self
                        .remote_control_wimius
                        .trigger(WimiusS27Control::ButtonRight),
                    _ => (),
                };
            }
            RemoteControllerInputEvent::TelevisionUp => {
                // TODO: add Apple TV.
                match self.activated_source {
                    RemoteControllerSource::Wimius => self
                        .remote_control_wimius
                        .trigger(WimiusS27Control::ButtonUp),
                    _ => (),
                };
            }
            _ => {}
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
            outputs.on_input_event(RemoteControllerInputEvent::from_byte(buffer[0]));
        }
        outputs.on_event_loop_iteration();
    }
}
