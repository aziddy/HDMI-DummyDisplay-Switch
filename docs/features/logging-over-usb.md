# USB CDC Logging - Implementation Notes

## Status: Blocked by dependency version conflicts

### Issue
The CH32V203 has USB hardware (USBD peripheral) and the ch32-hal provides a driver for it. However, there's a version mismatch between the HAL's USB driver and available embassy-usb versions:

- `ch32-hal` uses `embassy-usb-driver 0.2.0`
- `embassy-usb 0.3.x` uses `embassy-usb-driver 0.1.x`
- `embassy-usb 0.4.x+` uses incompatible driver versions

### Findings
- ✅ USB peripheral exists: `peripherals::USBD`
- ✅ Correct interrupt name found: `USB_LP_CAN1_RX0`
- ✅ USB pins: PA12 (DP), PA11 (DM)
- ❌ No compatible embassy-usb version available on crates.io

### Alternative: UART Logging
For reliable logging without a debug probe, use UART instead:
- USART1 or USART2 available on CH32V203G6U6
- Use a USB-to-UART adapter (CP2102, FT232, CH340, etc.)
- Much simpler, proven, and no dependency conflicts

### Future Options
1. Wait for ch32-hal to update to newer embassy-usb-driver versions
2. Use git dependency of embassy-usb that matches driver 0.2
3. Implement custom USB CDC without embassy-usb framework

## Why USB CDC Doesn't Work: Technical Explanation

### The Root Cause: Trait Version Incompatibility

In Rust's embedded ecosystem, USB functionality is split across multiple crates:
- **`embassy-usb-driver`**: Defines the `Driver` trait that USB drivers must implement
- **`embassy-usb`**: Provides the USB stack (classes like CDC-ACM, HID, etc.)
- **`ch32-hal`**: Implements the hardware-specific USB driver for CH32 chips

The problem occurs when these crates use **different versions** of `embassy-usb-driver`, because Rust treats trait definitions from different crate versions as completely different traits.

### What Happens During Compilation

1. **ch32-hal imports** `embassy-usb-driver v0.2.0` and implements:
   ```rust
   impl embassy_usb_driver_v0_2::Driver for ch32_hal::usbd::Driver { ... }
   ```

2. **embassy-usb v0.3.0** imports `embassy-usb-driver v0.1.0` and expects:
   ```rust
   fn new<D: embassy_usb_driver_v0_1::Driver>(...) { ... }
   ```

3. **The compiler sees**:
   - `ch32_hal::usbd::Driver` implements `embassy_usb_driver_v0_2::Driver` ✅
   - `embassy-usb` requires `embassy_usb_driver_v0_1::Driver` ✅
   - But these are **different traits** ❌

### The Actual Error
```
error[E0277]: the trait bound `ch32_hal::usbd::Driver<'static, USBD>:
              embassy_usb::embassy_usb_driver::Driver<'static>` is not satisfied
```

Translation: "You're trying to use ch32-hal's USB driver with embassy-usb, but they're speaking different versions of the USB driver 'language' and can't understand each other."

### Why Patching Doesn't Work

We tried using Cargo's `[patch]` feature to force all crates to use the same driver version:
```toml
[patch.crates-io]
embassy-usb-driver = { git = "...", rev = "..." }
```

However, this fails because:
1. **ch32-hal** directly specifies `embassy-usb-driver = "0.2.0"` in its `Cargo.toml`
2. **embassy-usb v0.3.x** was built against driver `0.1.x` and is **published/frozen** on crates.io
3. **embassy-usb v0.4.x+** uses driver `0.2.x`, but has breaking API changes incompatible with ch32-hal's implementation

The patch can't retroactively change already-published crate dependencies.

### Why Git Dependencies Don't Work

Attempting to use embassy-usb directly from git:
```toml
embassy-usb = { git = "https://github.com/embassy-rs/embassy.git", rev = "..." }
```

Still fails because:
- Different git commits/tags use different driver versions
- The embassy repo is a workspace with many interdependent crates
- ch32-hal's driver implementation was written for a specific driver API that no longer exists in newer embassy versions

### The Version Timeline

| Date | embassy-usb-driver | embassy-usb | ch32-hal Status |
|------|-------------------|-------------|-----------------|
| Early 2024 | v0.1.x | v0.3.x | ❌ Incompatible |
| Mid 2024 | v0.2.x | v0.4.0-alpha | ✅ ch32-hal uses this |
| Late 2024 | v0.2.x | v0.4.x | ❌ API changes broke compatibility |
| Current | v0.3.x | v0.5.x+ | ❌ Major version bump |

ch32-hal is stuck targeting the `v0.4.0-alpha` era of embassy-usb, which is no longer available.

### What Would Fix This

1. **ch32-hal updates**: The HAL maintainers update to the latest embassy versions
2. **Version rollback**: Use a very specific git commit of embassy from the narrow window when driver v0.2 existed and APIs matched
3. **Manual implementation**: Bypass embassy-usb entirely and write raw USB CDC code directly against the ch32 hardware

### Why This Is Common in Embedded Rust

The embedded Rust ecosystem is rapidly evolving. HALs (like ch32-hal) track low-level hardware and are updated less frequently, while frameworks (like embassy) iterate quickly to add features. This creates "version gap" problems that will resolve as the ecosystem matures.

For now, **USB CDC logging is technically possible but practically blocked** until version alignment occurs.
