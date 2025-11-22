# What is USB CDC?

## CDC = Communications Device Class

**CDC (Communications Device Class)** is a USB specification that defines how communication devices like modems, network adapters, and serial ports should work over USB.

Think of it as a standardized "language" that allows USB devices to communicate as if they were traditional serial/communication devices.

## Why CDC Exists

In the past, computers had dedicated physical ports:
- **RS-232 Serial Ports** (COM ports) - for connecting to modems, terminals, serial devices
- **Parallel Ports** - for printers
- **Phone Line Jacks** - for dial-up modems

USB replaced all of these, but software still expected to talk to "COM ports" and "modems". CDC is the bridge that makes USB devices **appear as traditional communication devices** to the operating system.

## USB CDC-ACM (What We're Using)

**ACM = Abstract Control Model**

CDC-ACM is a specific profile within CDC for devices that act like **modems or serial ports**. It's the most common CDC profile.

### What CDC-ACM Provides:

1. **Virtual Serial Port**: Your USB device appears as `/dev/ttyUSB0` (Linux/Mac) or `COM3` (Windows)
2. **Standard Serial API**: Applications can use normal serial port APIs (open, read, write, close)
3. **Control Signals**: Supports DTR, RTS, CTS, DSR (though we often ignore them)
4. **Configuration**: Line coding (baud rate, parity, stop bits) - usually ignored for virtual ports

## How USB CDC Works

### The Physical Reality:
```
[Your CH32V203] <--USB Data--> [Computer]
     USB Peripheral               USB Host
```

### What the Computer Sees:
```
[Your CH32V203] <--"Serial Port"--> [Your Software]
  Appears as COM3                   Opens COM3 like any serial device
```

### The Magic:
The operating system's **USB CDC driver** translates between:
- USB packets (how data actually travels over USB)
- Serial port API calls (how applications want to communicate)

## CDC-ACM Components

A USB CDC-ACM device has **two interfaces**:

### 1. Communication Interface (Control)
- **Purpose**: Send commands and receive notifications
- **Endpoint**: Interrupt IN endpoint (EP2 in our case)
- **Examples**: 
  - "Please set baud rate to 115200"
  - "The device is ready"
  - "Line state changed"

### 2. Data Interface (Actual Data)
- **Purpose**: Send and receive the actual serial data
- **Endpoints**: 
  - Bulk OUT endpoint (EP1) - Computer → Device
  - Bulk IN endpoint (EP1) - Device → Computer
- **Examples**:
  - Your log messages
  - Commands from terminal
  - Binary data transfer

## How Data Flows

### Sending Data (Device → Computer):

1. Your firmware writes: `"Hello World\n"`
2. USB hardware packetizes it into 64-byte USB packets
3. Computer's USB host controller receives packets
4. CDC driver reassembles packets
5. Data appears in `/dev/ttyUSB0` buffer
6. Your terminal app reads: `"Hello World\n"`

### Receiving Data (Computer → Device):

1. User types in terminal: `"AT\r\n"`
2. Terminal app writes to `/dev/ttyUSB0`
3. CDC driver packetizes into USB packets
4. USB host controller sends packets
5. Your CH32V203 USB peripheral receives packets
6. Interrupt fires with data: `"AT\r\n"`

## USB CDC vs Regular UART

| Feature | Hardware UART | USB CDC |
|---------|--------------|---------|
| Physical | TX/RX pins, voltage levels | D+/D- differential pair |
| Speed | Fixed (9600, 115200, etc.) | Up to 12 Mbps (Full Speed USB) |
| Cables | Direct wire connection | USB cable with enumeration |
| Drivers | Built into OS | CDC driver (usually built-in) |
| Complexity | Simple UART peripheral | Complex USB protocol |
| Benefits | Low overhead, simple | No extra hardware needed, faster |

## Why USB CDC for Logging?

For embedded devices, USB CDC is perfect because:

### ✅ Advantages:
1. **No extra hardware needed** - Your MCU already has USB
2. **Fast** - Much faster than slow UART speeds
3. **One cable** - Same cable for programming and logging
4. **Works everywhere** - Built-in drivers on Windows/Mac/Linux
5. **Standard tools** - Use any serial terminal (PuTTY, screen, minicom)

### ❌ Disadvantages:
1. **Complex implementation** - USB protocol is complicated
2. **More code** - 500-1000 lines vs 50 lines for UART
3. **Harder to debug** - Need USB analyzer if things go wrong
4. **Can't debug USB** - If USB crashes, you can't debug over USB

## The USB CDC Protocol Stack

```
┌─────────────────────────────────────┐
│   Your Application                  │
│   (writes log messages)             │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│   CDC-ACM Class Layer               │
│   (formats data as serial)          │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│   USB Device Layer                  │
│   (handles descriptors, endpoints)  │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│   USB Hardware (USBD peripheral)    │
│   (sends electrical signals)        │
└──────────────┬──────────────────────┘
               │
               USB Cable
```

## CDC Class Requests

The Communication Interface handles these special requests:

### SET_LINE_CODING (0x20)
Computer sends: "Please configure the serial port"
- Baud rate: 115200
- Data bits: 8
- Parity: None
- Stop bits: 1

**For virtual ports**: We usually just acknowledge and ignore these

### GET_LINE_CODING (0x21)
Computer asks: "What's the current serial configuration?"
We respond with the last configuration (or default values)

### SET_CONTROL_LINE_STATE (0x22)
Computer sends: "Set DTR=1, RTS=0"
- DTR (Data Terminal Ready)
- RTS (Request To Send)

**For virtual ports**: Usually ignored, but some terminals use DTR to detect if device is present

## Example: Opening a CDC Port

When you run `screen /dev/ttyUSB0 115200`:

1. **USB Enumeration**:
   - Computer: "What are you?"
   - Device: "I'm a CDC-ACM device" (sends descriptors)
   - Computer: Loads CDC driver

2. **Configuration**:
   - Driver: SET_LINE_CODING(115200, 8N1)
   - Device: "OK" (we ignore it)

3. **Activation**:
   - Driver: SET_CONTROL_LINE_STATE(DTR=1)
   - Device: "OK"

4. **Ready**:
   - `/dev/ttyUSB0` appears
   - `screen` can now read/write

## USB CDC Descriptors

These are the "ID cards" we show the computer:

### Device Descriptor
"I'm a USB 2.0 device, I'm in the CDC class family"

### Configuration Descriptor  
"I have 2 interfaces: Communication and Data"

### Interface Descriptors
- Communication Interface: "I'm CDC-ACM, I have 1 endpoint"
- Data Interface: "I'm for data transfer, I have 2 endpoints"

### Endpoint Descriptors
- EP0: Control (mandatory for all USB devices)
- EP1 OUT: Bulk, 64 bytes (receive data from computer)
- EP1 IN: Bulk, 64 bytes (send data to computer)
- EP2 IN: Interrupt, 8 bytes (send notifications)

### String Descriptors
- Manufacturer: "ch32-rs"
- Product: "HDMI Dummy Display Switch"
- Serial Number: "12345678"

## Why Our Implementation Is Blocked

```
┌──────────────┐     uses      ┌──────────────┐
│  ch32-hal    ├──────────────►│ embassy-usb- │
│              │                │  driver 0.2  │
└──────────────┘                └──────────────┘

┌──────────────┐     uses      ┌──────────────┐
│ embassy-usb  ├──────────────►│ embassy-usb- │
│    0.3.x     │                │  driver 0.1  │
└──────────────┘                └──────────────┘
                    ❌ Different versions!

┌──────────────┐     uses      ┌──────────────┐
│ embassy-usb  ├──────────────►│ embassy-usb- │
│    0.4.x+    │                │  driver 0.3  │
└──────────────┘                └──────────────┘
                    ❌ Different versions!
```

The ch32-hal USB driver implements the 0.2 interface, but no published embassy-usb version uses 0.2.

## Summary

**CDC-ACM** = A USB standard that makes your device appear as a serial port

**Why it's useful**: 
- No extra hardware
- Fast communication
- Works with standard tools

**Why it's hard**:
- USB protocol is complex
- Requires careful state management
- Version conflicts in the Rust ecosystem

**Current status**:
- Hardware initialized ✅
- Descriptors defined ✅  
- USB protocol state machine needed ❌
- Endpoint management needed ❌
- CDC class request handling needed ❌

For now, the bare-metal approach gives you a foundation, but needs ~600 lines more code to actually work as a CDC device.
