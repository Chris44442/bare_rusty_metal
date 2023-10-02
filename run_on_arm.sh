#!/bin/bash

source ~/.fpga_config_de10
IP=$SOC_IP_DE10
# MEM="target/armv7a-none-eabi/debug/rusty_bare_metal"
MEM="ke"
MEM_HPS="~/sdcard/ke"

ssh root@$IP 'mkdir -p sdcard && mount /dev/mmcblk0p1 ~/sdcard'
scp $MEM root@$IP:$MEM_HPS > /dev/null
