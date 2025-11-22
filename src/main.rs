#![no_std]
#![no_main]

use ch32_hal as hal;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_time::Timer;
use embassy_usb::class::cdc_acm::{CdcAcmClass, State};
use embassy_usb::driver::EndpointError;
use embassy_usb::Builder;
use hal::gpio::{Level, Output, Speed};
use hal::usbd::Driver;
use hal::{bind_interrupts, println};
use panic_halt as _;

bind_interrupts!(struct Irqs {
    USB_LP_CAN1_RX0 => hal::usbd::InterruptHandler<hal::peripherals::USBD>;
});

#[embassy_executor::task]
async fn usb_task(mut class: CdcAcmClass<'static, Driver<'static, hal::peripherals::USBD>>) -> ! {
    loop {
        class.wait_connection().await;
        // println!("USB CDC Connected!");

        let _ = class
            .write_packet(b"HDMI Dummy Display Switch - USB CDC Ready!\r\n")
            .await;

        let _ = echo(&mut class).await;

        // println!("USB CDC Disconnected");
    }
}

async fn echo(
    class: &mut CdcAcmClass<'static, Driver<'static, hal::peripherals::USBD>>,
) -> Result<(), EndpointError> {
    let mut buf = [0; 64];
    loop {
        let n = class.read_packet(&mut buf).await?;
        let data = &buf[..n];

        // Echo back what we received
        class.write_packet(data).await?;

        // Also print to debug output
        if let Ok(s) = core::str::from_utf8(data) {
            // println!("Received: {}", s.trim());
        }
    }
}

#[embassy_executor::main(entry = "ch32_hal::entry")]
async fn main(spawner: Spawner) -> ! {
    // println!("HDMI Dummy Display Switch - Starting...");

    let p = hal::init(hal::Config {
        rcc: hal::rcc::Config::SYSCLK_FREQ_144MHZ_HSI,
        ..Default::default()
    });

    // println!("Initializing USB CDC...");

    // Create USB driver
    let driver = Driver::new(p.USBD, Irqs, p.PA12, p.PA11);

    // USB device configuration
    let mut config = embassy_usb::Config::new(0x16c0, 0x27dd);
    config.manufacturer = Some("ch32-rs");
    config.product = Some("ZEDBOP");
    config.serial_number = Some("12345678");
    config.max_power = 100;
    config.max_packet_size_0 = 64;

    // Required for CDC
    config.device_class = 0x02;
    config.device_sub_class = 0x02;
    config.device_protocol = 0x00;
    config.composite_with_iads = false;

    // Create buffers for USB descriptors
    static mut CONFIG_DESCRIPTOR: [u8; 256] = [0; 256];
    static mut BOS_DESCRIPTOR: [u8; 256] = [0; 256];
    static mut CONTROL_BUF: [u8; 64] = [0; 64];
    static mut STATE: State = State::new();

    let mut builder = unsafe {
        Builder::new(
            driver,
            config,
            &mut CONFIG_DESCRIPTOR,
            &mut BOS_DESCRIPTOR,
            &mut [],
            &mut CONTROL_BUF,
        )
    };

    // Create CDC-ACM class
    let class = CdcAcmClass::new(&mut builder, unsafe { &mut STATE }, 64);

    // Build USB device
    let mut usb = builder.build();

    // println!("USB initialized, starting tasks...");

    // Configure PA0 as LED output
    let mut led = Output::new(p.PA0, Level::Low, Speed::default());

    // Configure PA1 as EEPROM write protect (high = protected)
    let _wc = Output::new(p.PA1, Level::High, Speed::default());

    // Spawn USB task
    spawner.spawn(usb_task(class)).unwrap();

    // println!("USB CDC task spawned, running main loop");

    // Run USB device and blink LED concurrently
    let mut counter = 0u32;
    join(usb.run(), async {
        loop {
            led.set_high();
            Timer::after_millis(500).await;
            // println!("LED ON - Counter: {}", counter);

            led.set_low();
            Timer::after_millis(500).await;
            // println!("LED OFF - Counter: {}", counter);

            counter += 1;
        }
    })
    .await;

    loop {}
}
