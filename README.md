# rust_rpi_lora
Long range LoRa communication with Rpi Zero and Rust

Taking inputs from a joystick on /dev/input/js0 and outputting over serial

Made for long range wireless control of progects using LoRa Uart moduldes, however will also work with a wired serial connection.

This code sits on the 'ground station' and is used for the transmittion of commands, a similar setup can be placed on the wireless device for control.


Requires nightly rust build - will not run on stable
