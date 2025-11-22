# Logging Over USB
- ❌ Don't wanna use Debugger/SWD
- ❌ Don't wanna use UART
- ✅ Use USB CDC ACM

**Implement USB CDC our own firmware**
- Because the CH32V203 supports USB device mode, you can write firmware that implements a USB CDC (virtual COM port) via a USB stack (e.g. TinyUSB or whatever our environment supports)
- That way, when our firmware runs, it can present a virtual serial port over USB, separate from the bootloader
- For example, Adafruit’s QT Py CH32V203 board supports USB CDC via TinyUSB
- Then you could open /dev/tty.usbmodem… (or similar) on our computer to read runtime logs