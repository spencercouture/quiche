#!/bin/bash

# if tcpdump is running, kill it
if [ -f /var/run/tcpdump.pid ]; then
	kill $(cat /var/run/tcpdump.pid)
	rm -f /var/run/tcpdump.pid
fi

# # helper
# run_dir="/home/scouture/HTTP-3-Prioritization/quiche/rundir"
# date_str=$(date +%Y%m%d%H%M%S)
#
# # copy all server-data folders to staging area
# docker cp quiche-server:/server-data/ $run_dir/
# # copy data into mirrored folders
# cp -r $run_dir/server-data/* $run_dir/
# rm -rf $run_dir/server-data/
#
# # put pcap in each server folder
# server_folders=$(ls $run_dir/ | grep -v '\.pcap')
# for dir in $server_folders; do
# 	cp $run_dir/quiche-tcpdump.pcap $run_dir/$dir/
# 	# mv $run_dir/$dir $run_dir/${dir}-${date_str}
# done

# # helper
# log_dir="/home/scouture/HTTP-3-Prioritization/quiche/logs"
# date_str=$(date +%Y%m%d%H%M%S)
#
# # clear staging area
# rm -fr $log_dir/staging-area/*
#
# # copy all server-data folders to staging area
# docker cp quiche-server:/server-data/ $log_dir/staging-area/
# # move same folders out of server-data folder
# mv $log_dir/staging-area/server-data/* $log_dir/staging-area/
# rm -rf $log_dir/staging-area/server-data/
#
# # put pcap in each server folder
# server_folders=$(ls $log_dir/staging-area | grep -v '\.pcap')
# for dir in $server_folders ; do
#         cp $log_dir/quiche-tcpdump.pcap $log_dir/staging-area/$dir/
#         mv $log_dir/staging-area/$dir $log_dir/${dir}-${date_str}
# done
#

# normal docker stuff
docker stop quiche-client
rm /var/run/netns/quiche-client

docker stop quiche-server
rm /var/run/netns/quiche-server
