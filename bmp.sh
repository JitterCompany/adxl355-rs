#!/bin/bash
ELF=$1

BMP=`ls /dev/cu.usbmodem*1`

echo flashing on $BMP
arm-none-eabi-gdb -ex 'target extended-remote'$BMP'' -x bmp.gdb -q $ELF
