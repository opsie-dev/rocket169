#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::Config as HALConfig;
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
use esp_hal::uart::{Config as UartConfig, Uart};
use log::debug;
use log::info;
use rocket169::input::RemoteControllerInputs;

esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();
    let configuration = HALConfig::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(configuration);
    let mut uart = Uart::new(
        peripherals.UART0,
        UartConfig::default(),
    )?.with_tx(peripherals.GPIO0);
    let inputs = RemoteControllerInputs::new(peripherals);
    loop {
        let n = match inputs.poll() {
            Some(event) => match uart.write(event.as_uart_message()) {
                Err(error) => {
                    // TODO: log error
                    // TODO: blink uart error led.
                    0
                }
                Ok(i) => i,
            }
            None => 0
        };
        debug!("{} byte(s) written to UART0", n);
        // TODO: check if waiting is necessary ?
        // let delay_start = Instant::now();
        //while delay_start.elapsed() < Duration::from_millis(500) {}
    }
}
