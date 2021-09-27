All these commands are on Mac so linux will be a bit different.

Build:
```
function deploy_dev() {(
  set -e 
  rustup run nightly cargo build -p stubot-mcu
  openocd -f interface/stlink.cfg -f target/stm32g4x.cfg -c "program ../target/thumbv7em-none-eabihf/debug/stubot-mcu verify reset exit"
)}

function deploy_release() {(
  set -e 
  rustup run nightly cargo build -p stubot-mcu --release
  openocd -f interface/stlink.cfg -f target/stm32g4x.cfg -c "program ../target/thumbv7em-none-eabihf/release/stubot-mcu verify reset exit"
)}
```

How to see the serial output:
```
picocom /dev/tty.usbmodem* --baud 9600 --imap lfcrlf
```


Misc stuff I debug with:
```
arm-none-eabi-objcopy -O ihex stubot-mcu stubot-mcu.hex
arm-none-eabi-readelf -a stubot-mcu
rust-size stubot-mcu
rustup run nightly cargo bloat -p stubot-mcu
```