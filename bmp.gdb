

#target extended-remote /dev/cu.usbmodem*1
#target extended-remote /dev/cu.usbmodem*1

monitor swdp_scan
attach 1
monitor traceswo
set mem inaccessible-by-default off

# common
break Reset
# break main

load

# start the process but immediately halt the processor
stepi
