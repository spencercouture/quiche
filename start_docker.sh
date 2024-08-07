#!/bin/bash
set -e

# start client container
docker run --rm -it -d --dns 10.0.1.4 --network=none -v "/tmp/quiche-server-data-$1":/server-data --name quiche-client cloudflare/quiche /bin/bash
#docker run --shm-size=4g --rm -it -d  --network=none --entrypoint "/bin/bash" -v "/tmp/browsertime-$1":/browsertime --dns 10.0.1.4 --cap-add=SYS_NICE --name $1-browsertime constantin/browsertime
DPID=$(docker inspect -f '{{.State.Pid}}' quiche-client)
mkdir -p /var/run/netns
ln -s /proc/$DPID/ns/net /var/run/netns/quiche-client
# set env variable
# docker run -e SSLKEYLOGFILE=/sslkeyfile quiche-client

# start server container
docker run --rm -it -d --dns 10.0.1.4 --network=none -v "/tmp/quiche-server-data-$1":/server-data --name quiche-server cloudflare/quiche /bin/bash
#docker run --shm-size=4g --rm -it -d  --network=none --entrypoint "/bin/bash" -v "/tmp/browsertime-$1":/browsertime --dns 10.0.1.4 --cap-add=SYS_NICE --name $1-browsertime constantin/browsertime
DPID=$(docker inspect -f '{{.State.Pid}}' quiche-server)
mkdir -p /var/run/netns
ln -s /proc/$DPID/ns/net /var/run/netns/quiche-server
# set env variable
# docker run -e SSLKEYLOGFILE=/sslkeyfile quiche-server
