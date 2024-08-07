#!/usr/bin/env bash

nohup ip netns exec quiche-server /usr/sbin/tcpdump -ni vethd${1}664 -s 65535 -w /tmp/quiche-server-data-$1/quiche-tcpdump.pcap &
echo $! >/var/run/tcpdump.pid
