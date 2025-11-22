# Bare-Metal USB CDC Implementation

## Current Status

We've successfully created a **bare-metal firmware** that compiles and runs **without embassy or ch32-hal dependencies**. This avoids all the version conflict issues.

### What Works Now ✅
- ✅ Bare-metal compilation (no HAL, no embassy)
- ✅ Direct hardware register access
- ✅ GPIO control (LED on PA0, EEPROM WC on PA1)
- ✅ USB peripheral initialization
- ✅ USB D+ pullup enabled (device will enumerate)
- ✅ USB interrupts configured

### What's Missing ❌
- ❌ USB protocol state machine
- ❌ USB descriptor handling
- ❌ USB endpoint management
- ❌ CDC-ACM class implementation
- ❌ Actual data transmission/reception

## Implementing Full USB CDC

To complete USB CDC functionality, you need to implement:

### 1. USB Device Enumeration (~200 lines)
Handle USB RESET, SETUP requests, GET_DESCRIPTOR, SET_ADDRESS, SET_CONFIGURATION

### 2. Endpoint Buffer Management (~100 lines)
Manage the USB packet memory (PMA) at 0x40006000, setup endpoint buffers

### 3. CDC-ACM Class Requests (~150 lines)
Handle SET_LINE_CODING, SET_CONTROL_LINE_STATE, GET_LINE_CODING

### 4. Data Transfer (~100 lines)
Implement bulk IN/OUT endpoints for actual serial data

## Why This Is Complex

USB CDC requires:
1. **State machine**: ATTACHED → POWERED → DEFAULT → ADDRESSED → CONFIGURED
2. **Descriptor tables**: Device, Configuration, Interface, Endpoint, String descriptors
3. **Control transfers**: Setup stage → Data stage → Status stage
4. **Interrupt handling**: Process every USB packet in ISR
5. **Timing constraints**: Must respond to setup packets within 5ms

### Total Implementation Effort
- **~600 lines of code**
- **3-5 days of development** for someone familiar with USB
- **1-2 weeks** if learning USB protocol from scratch

## Recommended Paths Forward

### Option 1: Wait for HAL Update ⏳
**Effort**: None  
**Timeline**: Weeks/months  
The ch32-hal maintainers will eventually update embassy dependencies.

### Option 2: Use WCH's Official Examples 🔄
**Effort**: Medium  
**Timeline**: 1-2 days  
Port WCH's official C USB examples to Rust. They provide working USB CDC code.

### Option 3: Full Custom Implementation 💪
**Effort**: High  
**Timeline**: 1-2 weeks  
Complete the bare-metal USB CDC stack we started.

### Option 4: USB-UART Bridge Hardware 🔌
**Effort**: None (requires hardware)  
**Timeline**: Immediate  
Add a CP2102/FT232 module if you can access USART pins.

## References

- **USB 2.0 Specification**: https://www.usb.org/document-library/usb-20-specification
- **USB CDC Class Spec**: https://www.usb.org/document-library/class-definitions-communication-devices-12
- **CH32V Reference Manual**: Has USB peripheral register details
- **WCH Examples**: Check WCH's SDK for working CDC examples

## Current Code Status

The firmware currently:
- Initializes USB hardware ✅
- Blinks LED to show it's running ✅
- Has placeholder interrupt handlers ✅
- Needs full USB protocol implementation ❌
