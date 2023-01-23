#!/bin/bash

set -e

export AVR_CPU_FREQUENCY_HZ=16000000

echo $1

if [[ -z $1 || $1 == "all" ]]
then
    cargo build -Z build-std=core --target avr-atmega328p.json --release
    sudo avrdude -v -patmega328p -carduino -P/dev/ttyUSB0 -b57600 -D -Uflash:w:/home/user/project/keyboard-atm32p8/target/avr-atmega328p/release/keyboard-atm32p8.elf:e
elif [[ $1 == "build" ]]
then
    cargo build -Z build-std=core --target avr-atmega328p.json --release
elif [[ $1 == "flash" ]]
then
    sudo avrdude -v -patmega328p -carduino -P/dev/ttyUSB0 -b57600 -D -Uflash:w:/home/user/project/keyboard-atm32p8/target/avr-atmega328p/release/keyboard-atm32p8.elf:e
fi
