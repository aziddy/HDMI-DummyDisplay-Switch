#![no_std]
#![no_main]

use ch32_hal::gpio::{Level, Output, Speed};
use embassy_executor::Spawner;
use embassy_time::Timer;
use panic_halt as _;

#[embassy_executor::main(entry = "ch32_hal::entry")]
async fn main(_spawner: Spawner) -> ! {
    // Initialize peripherals with default config
    let p = ch32_hal::init(Default::default());

    // Configure PA0 as output (LED blink example)
    let mut led = Output::new(p.PA0, Level::Low, Speed::default());

    // Initialize I2C pins for EEPROM (defined but not used yet)
    // PB6 = SCL, PB7 = SDA, PA1 = WC (Write Control)
    // let _wc = Output::new(p.PA1, Level::High, Speed::default()); // WC high = write protected

    loop {
        led.set_high();
        Timer::after_millis(500).await;
        led.set_low();
        Timer::after_millis(500).await;
    }
}
