#![no_std]
#![no_main]

use core::ptr;
use panic_halt as _;
use qingke_rt::entry;
use ch32v2::ch32v20x;

#[entry]
fn main() -> ! {
    let dp = unsafe { ch32v20x::Peripherals::steal() };
    {
        // Enable clocks
        unsafe {
            dp.RCC.ahbpcenr.write(|w| w.bits(0x00000001)); // DMA
            dp.RCC.apb2pcenr.write(|w| w.bits(0x0000001D)); // GPIOA, AFIO
            dp.RCC.apb1pcenr.write(|w| w.bits(0x00800000)); // USB
        }

        // Configure PA0 as output (LED)
        unsafe {
            let val = dp.GPIOA.cfglr.read().bits();
            dp.GPIOA.cfglr.write(|w| w.bits((val & !0xF) | 0x3));
        }

        // Configure PA1 as output high (EEPROM WC)
        unsafe {
            let val = dp.GPIOA.cfglr.read().bits();
            dp.GPIOA.cfglr.write(|w| w.bits((val & !0xF0) | 0x30));
            dp.GPIOA.outdr.write(|w| w.bits(0x02));
        }

        // Initialize USB
        usb_init();

        let mut counter = 0u32;
        let mut led_state = false;

        loop {
            if counter % 1000000 == 0 {
                led_state = !led_state;
                unsafe {
                    if led_state {
                        dp.GPIOA.outdr.write(|w| w.bits(0x03));
                    } else {
                        dp.GPIOA.outdr.write(|w| w.bits(0x02));
                    }
                }
            }
            counter = counter.wrapping_add(1);
        }
    }

    loop {}
}

fn usb_init() {
    unsafe {
        // Enable USB D+ pullup
        let extend = 0x40023800 as *mut u32;
        ptr::write_volatile(extend, ptr::read_volatile(extend) | (1 << 7));

        // Reset USB
        let rcc_apb1prstr = 0x40021010 as *mut u32;
        ptr::write_volatile(rcc_apb1prstr, 0x00800000);
        delay(1000);
        ptr::write_volatile(rcc_apb1prstr, 0x00000000);
        delay(1000);

        // Initialize USB peripheral
        let usb_cntr = 0x40005C40 as *mut u16;
        let usb_btable = 0x40005C50 as *mut u16;

        // Clear power down and reset bits
        ptr::write_volatile(usb_cntr, 0);
        delay(100);

        // Set buffer table address to 0
        ptr::write_volatile(usb_btable, 0);

        // Enable USB interrupts: CTR, RESET, SUSP, WKUP
        ptr::write_volatile(usb_cntr, 0xBC00);
    }
}

fn delay(count: u32) {
    for _ in 0..count {
        unsafe {
            core::arch::asm!("nop");
        }
    }
}

// Dummy USB interrupt handlers (will implement full protocol later)
#[no_mangle]
unsafe extern "C" fn USB_HP_CAN1_TX() {
    // High priority USB interrupt
}

#[no_mangle]
unsafe extern "C" fn USB_LP_CAN1_RX0() {
    // Low priority USB interrupt
    // This is where we'll handle USB CDC protocol
}