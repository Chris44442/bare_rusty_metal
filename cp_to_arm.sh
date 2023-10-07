#!/bin/bash

source ~/.fpga_config_de10
IP=$SOC_IP_DE10
MEM="bin"
MEM_HPS="~/sdcard/bin"

ssh root@$IP 'mkdir -p sdcard && mount /dev/mmcblk0p1 ~/sdcard'
scp $MEM root@$IP:$MEM_HPS > /dev/null
