#!/bin/bash
FLASH_ADDRESS=$1
ELF=$2
arm-none-eabi-objcopy -O binary $ELF $ELF.bin
st-flash --reset write $ELF.bin $FLASH_ADDRESS