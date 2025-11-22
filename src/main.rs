#![no_std]
#![no_main]

use ch32_hal::gpio::{Level, Output, Speed};
use embassy_executor::Spawner;
use embassy_time::Timer;
use panic_halt as _;

#[embassy_executor::main(entry = "ch32_hal::entry")]
async fn main(_spawner: Spawner) -> ! {
    let p = ch32_hal::init(Default::default());

    // Configure PA0 as LED output
    let mut led = Output::new(p.PA0, Level::Low, Speed::default());

    // Initialize EEPROM write control pin (PA1 = high for write-protected)
    let _wc = Output::new(p.PA1, Level::High, Speed::default());

    // NOTE: USB CDC logging requires embassy-usb-driver version alignment
    // Currently blocked - see docs/features/logging-over-usb.md
    //
    // The firmware is fully functional, just without logging capability.
    // Once ch32-hal updates to compatible embassy versions, USB CDC can be added.

    loop {
        led.set_high();
        Timer::after_millis(500).await;

        led.set_low();
        Timer::after_millis(500).await;
    }
}
