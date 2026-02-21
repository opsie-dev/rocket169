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

impl RemoteControllerInput {
    fn new(event: RemoteControllerInputEvent, input: Input<'static>) -> RemoteControllerInput {
        RemoteControllerInput { event, input }
    }
}

struct RemoteControllerInputs {
    inputs: [RemoteControllerInput; REMOTE_CONTROLLER_INPUT_SIZE],
}

impl RemoteControllerInputs {
    fn new(
        hdmi_source_1_input: Input<'static>,
        hdmi_source_2_input: Input<'static>,
        hdmi_source_3_input: Input<'static>,
        input_source_apple_input: Input<'static>,
        input_source_triangle_input: Input<'static>,
        input_source_wimius_input: Input<'static>,
        light_off_input: Input<'static>,
        light_on_input: Input<'static>,
        sound_bass_down_input: Input<'static>,
        sound_bass_up_input: Input<'static>,
        sound_equalizer_reset_input: Input<'static>,
        sound_mute_input: Input<'static>,
        sound_source_input: Input<'static>,
        sound_treble_down_input: Input<'static>,
        sound_treble_up_input: Input<'static>,
        sound_volume_down_input: Input<'static>,
        sound_volume_up_input: Input<'static>,
        television_back_input: Input<'static>,
        television_down_input: Input<'static>,
        television_home_input: Input<'static>,
        television_left_input: Input<'static>,
        television_menu_input: Input<'static>,
        television_play_pause_input: Input<'static>,
        television_right_input: Input<'static>,
        television_up_input: Input<'static>,
    ) -> RemoteControllerInputs {
        RemoteControllerInputs {
            inputs: [
                RemoteControllerInput::new(
                    RemoteControllerInputEvent::HDMISource1,
                    hdmi_source_1_input,
                ),
                RemoteControllerInput::new(
                    RemoteControllerInputEvent::HDMISource2,
                    hdmi_source_2_input,
                ),
                RemoteControllerInput::new(
                    RemoteControllerInputEvent::HDMISource3,
                    hdmi_source_3_input,
                ),
                RemoteControllerInput::new(
                    RemoteControllerInputEvent::InputSourceApple,
                    input_source_apple_input,
                ),
                RemoteControllerInput::new(
                    RemoteControllerInputEvent::InputSourceTriangle,
                    input_source_triangle_input,
                ),
                RemoteControllerInput::new(
                    RemoteControllerInputEvent::InputSourceWimius,
                    input_source_wimius_input,
                ),
                RemoteControllerInput::new(RemoteControllerInputEvent::LightOff, light_off_input),
                RemoteControllerInput::new(RemoteControllerInputEvent::LightOn, light_on_input),
                RemoteControllerInput::new(
                    RemoteControllerInputEvent::SoundBassDown,
                    sound_bass_down_input,
                ),
                RemoteControllerInput::new(
                    RemoteControllerInputEvent::SoundBassUp,
                    sound_bass_up_input,
                ),
                RemoteControllerInput::new(
                    RemoteControllerInputEvent::SoundEqualizerReset,
                    sound_equalizer_reset_input,
                ),
                RemoteControllerInput::new(RemoteControllerInputEvent::SoundMute, sound_mute_input),
                RemoteControllerInput::new(
                    RemoteControllerInputEvent::SoundSource,
                    sound_source_input,
                ),
                RemoteControllerInput::new(
                    RemoteControllerInputEvent::SoundTrebleDown,
                    sound_treble_down_input,
                ),
                RemoteControllerInput::new(
                    RemoteControllerInputEvent::SoundTrebleUp,
                    sound_treble_up_input,
                ),
                RemoteControllerInput::new(
                    RemoteControllerInputEvent::SoundVolumeDown,
                    sound_volume_down_input,
                ),
                RemoteControllerInput::new(
                    RemoteControllerInputEvent::SoundVolumeUp,
                    sound_volume_up_input,
                ),
                RemoteControllerInput::new(
                    RemoteControllerInputEvent::TelevisionBack,
                    television_back_input,
                ),
                RemoteControllerInput::new(
                    RemoteControllerInputEvent::TelevisionDown,
                    television_down_input,
                ),
                RemoteControllerInput::new(
                    RemoteControllerInputEvent::TelevisionHome,
                    television_home_input,
                ),
                RemoteControllerInput::new(
                    RemoteControllerInputEvent::TelevisionLeft,
                    television_left_input,
                ),
                RemoteControllerInput::new(
                    RemoteControllerInputEvent::TelevisionMenu,
                    television_menu_input,
                ),
                RemoteControllerInput::new(
                    RemoteControllerInputEvent::TelevisionPlayPause,
                    television_play_pause_input,
                ),
                RemoteControllerInput::new(
                    RemoteControllerInputEvent::TelevisionRight,
                    television_right_input,
                ),
                RemoteControllerInput::new(
                    RemoteControllerInputEvent::TelevisionUp,
                    television_up_input,
                ),
            ],
        }
    }
    fn poll(&self) -> Option<RemoteControllerInputEvent> {
        self.inputs
            .iter()
            .filter(|x| x.input.is_low())
            .map(|x| x.event)
            .next()
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
        .with_rx(peripherals.GPIO5)
        .with_tx(peripherals.GPIO4);
    let input_configuration = InputConfig::default().with_pull(Pull::Up);
    let inputs = RemoteControllerInputs::new(
        Input::new(peripherals.GPIO1, input_configuration),
        Input::new(peripherals.GPIO2, input_configuration),
        Input::new(peripherals.GPIO3, input_configuration),
        Input::new(peripherals.GPIO12, input_configuration),
        Input::new(peripherals.GPIO13, input_configuration),
        Input::new(peripherals.GPIO14, input_configuration),
        Input::new(peripherals.GPIO15, input_configuration),
        Input::new(peripherals.GPIO16, input_configuration),
        Input::new(peripherals.GPIO17, input_configuration),
        Input::new(peripherals.GPIO18, input_configuration),
        Input::new(peripherals.GPIO19, input_configuration),
        Input::new(peripherals.GPIO21, input_configuration),
        Input::new(peripherals.GPIO22, input_configuration),
        Input::new(peripherals.GPIO23, input_configuration),
        Input::new(peripherals.GPIO25, input_configuration),
        Input::new(peripherals.GPIO26, input_configuration),
        Input::new(peripherals.GPIO27, input_configuration),
        Input::new(peripherals.GPIO32, input_configuration),
        Input::new(peripherals.GPIO33, input_configuration),
        Input::new(peripherals.GPIO34, input_configuration),
        Input::new(peripherals.GPIO35, input_configuration),
        Input::new(peripherals.GPIO36, input_configuration),
        Input::new(peripherals.GPIO37, input_configuration),
        Input::new(peripherals.GPIO38, input_configuration),
        Input::new(peripherals.GPIO39, input_configuration),
    );
    loop {
        let n = match inputs.poll() {
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
