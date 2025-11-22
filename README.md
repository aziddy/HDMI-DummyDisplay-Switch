





### Getting Started

#### Install RISC-V Toolchain
```bash
rustup target add riscv32imc-unknown-none-elf
```

#### Get Bin/Hex after building
```bash
cargo objcopy --release -- -O binary target/riscv32imc-unknown-none-elf/release/hdmi-dummy-display-switch.bin
# OR
cargo objcopy --release -- -O ihex target/riscv32imc-unknown-none-elf/release/hdmi-dummy-display-switch.hex
```

File will be at `target/riscv32imc-unknown-none-elf/release/hdmi-dummy-display-switch.{bin,hex}`

#### Interact with device over USB CDC
```bash
screen /dev/ttyACM0 115200
```


#### MacOS see USB Logs Stream Live
```bash
log stream --predicate 'eventMessage contains "USB"' --style compact
```