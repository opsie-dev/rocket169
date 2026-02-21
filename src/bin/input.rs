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
use esp_hal::gpio::{Input, InputConfig, Pull};
use esp_hal::main;
use esp_hal::uart::{Config as UartConfig, Uart};
use log::debug;
use rocket169::event::RemoteControllerInputEvent;

esp_bootloader_esp_idf::esp_app_desc!();

const REMOTE_CONTROLLER_INPUT_SIZE: usize = 25;

struct RemoteControllerInput {
    event: RemoteControllerInputEvent,
    input: Input<'static>,
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
        .with_rx(peripherals.GPIO5)
        .with_tx(peripherals.GPIO4);
    let input_configuration = InputConfig::default().with_pull(Pull::Up);
    let inputs: [RemoteControllerInput; REMOTE_CONTROLLER_INPUT_SIZE] = [
        RemoteControllerInput {
            event: RemoteControllerInputEvent::HDMISource1,
            input: Input::new(peripherals.GPIO1, input_configuration),
        },
        RemoteControllerInput {
            event: RemoteControllerInputEvent::HDMISource2,
            input: Input::new(peripherals.GPIO12, input_configuration),
        },
        RemoteControllerInput {
            event: RemoteControllerInputEvent::HDMISource3,
            input: Input::new(peripherals.GPIO13, input_configuration),
        },
        RemoteControllerInput {
            event: RemoteControllerInputEvent::InputSourceApple,
            input: Input::new(peripherals.GPIO14, input_configuration),
        },
        RemoteControllerInput {
            event: RemoteControllerInputEvent::InputSourceTriangle,
            input: Input::new(peripherals.GPIO15, input_configuration),
        },
        RemoteControllerInput {
            event: RemoteControllerInputEvent::InputSourceWimius,
            input: Input::new(peripherals.GPIO16, input_configuration),
        },
        RemoteControllerInput {
            event: RemoteControllerInputEvent::LightOff,
            input: Input::new(peripherals.GPIO17, input_configuration),
        },
        RemoteControllerInput {
            event: RemoteControllerInputEvent::LightOn,
            input: Input::new(peripherals.GPIO18, input_configuration),
        },
        RemoteControllerInput {
            event: RemoteControllerInputEvent::SoundBassDown,
            input: Input::new(peripherals.GPIO19, input_configuration),
        },
        RemoteControllerInput {
            event: RemoteControllerInputEvent::SoundBassUp,
            input: Input::new(peripherals.GPIO2, input_configuration),
        },
        RemoteControllerInput {
            event: RemoteControllerInputEvent::SoundEqualizerReset,
            input: Input::new(peripherals.GPIO21, input_configuration),
        },
        RemoteControllerInput {
            event: RemoteControllerInputEvent::SoundMute,
            input: Input::new(peripherals.GPIO22, input_configuration),
        },
        RemoteControllerInput {
            event: RemoteControllerInputEvent::SoundSource,
            input: Input::new(peripherals.GPIO23, input_configuration),
        },
        RemoteControllerInput {
            event: RemoteControllerInputEvent::SoundTrebleDown,
            input: Input::new(peripherals.GPIO25, input_configuration),
        },
        RemoteControllerInput {
            event: RemoteControllerInputEvent::SoundTrebleUp,
            input: Input::new(peripherals.GPIO26, input_configuration),
        },
        RemoteControllerInput {
            event: RemoteControllerInputEvent::SoundVolumeDown,
            input: Input::new(peripherals.GPIO27, input_configuration),
        },
        RemoteControllerInput {
            event: RemoteControllerInputEvent::SoundVolumeUp,
            input: Input::new(peripherals.GPIO3, input_configuration),
        },
        RemoteControllerInput {
            event: RemoteControllerInputEvent::TelevisionBack,
            input: Input::new(peripherals.GPIO32, input_configuration),
        },
        RemoteControllerInput {
            event: RemoteControllerInputEvent::TelevisionDown,
            input: Input::new(peripherals.GPIO33, input_configuration),
        },
        RemoteControllerInput {
            event: RemoteControllerInputEvent::TelevisionHome,
            input: Input::new(peripherals.GPIO34, input_configuration),
        },
        RemoteControllerInput {
            event: RemoteControllerInputEvent::TelevisionLeft,
            input: Input::new(peripherals.GPIO35, input_configuration),
        },
        RemoteControllerInput {
            event: RemoteControllerInputEvent::TelevisionMenu,
            input: Input::new(peripherals.GPIO36, input_configuration),
        },
        RemoteControllerInput {
            event: RemoteControllerInputEvent::TelevisionPlayPause,
            input: Input::new(peripherals.GPIO37, input_configuration),
        },
        RemoteControllerInput {
            event: RemoteControllerInputEvent::TelevisionRight,
            input: Input::new(peripherals.GPIO38, input_configuration),
        },
        RemoteControllerInput {
            event: RemoteControllerInputEvent::TelevisionUp,
            input: Input::new(peripherals.GPIO39, input_configuration),
        },
    ];
    loop {
        let n = match inputs
            .iter()
            .filter(|x| x.input.is_low())
            .map(|x| x.event)
            .next()
        {
            Some(event) => match uart.write(event.to_byte()) {
                Err(error) => {
                    debug!("Error while writing message to UART {}", error);
                    0
                }
                Ok(i) => i,
            },
            None => 0,
        };
        debug!("{} byte(s) written to UART0", n);
    }
}
