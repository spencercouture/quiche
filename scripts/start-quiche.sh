#!/usr/bin/env bash
cert_file=$1
key_file=$2
addr=$3
idx=$4
directory=$5

# clean the directory if files are lingering...
docker exec quiche-server rm -rf /server-data/$idk
# move cert and key files over
docker exec quiche-server mkdir -p /server-data/$idx
docker cp $cert_file quiche-server:/server-data/$idx
docker cp $key_file quiche-server:/server-data/$idx

# move protobuf/static files over
echo "docker cp $directory/protobuf_files/. quiche-server:/protobuf_files" >/tmp/delme.log
docker cp $directory/protobuf_files/. quiche-server:/protobuf_files
docker cp $directory/static_files/. quiche-server:/static_files

cert_basename=$(basename $cert_file)
key_basename=$(basename $key_file)

cmd_string="export QLOGDIR=/server-data/$idx/ && export RUST_BACKTRACE=1 && export SSLKEYLOGFILE=/server-data/$idx/sslkeyfile && quiche-server --listen $addr --cert /server-data/$idx/cert.crt --key /server-data/$idx/cert.key --priorities-input /static_files/priorities.json --priorities-output /server-data/$idx/priorities.json 2>&1 | tee /server-data/$idx/server.log"

docker exec -d quiche-server bash -c "echo \"$cmd_string\" > /server-data/$idx/start-server.sh"

# docker exec -d quiche-server quiche-server --listen $addr --cert /$cert_basename --key /$key_basename
#docker exec -d quiche-server bash -c "export SSLKEYLOGFILE=/server-data/$idx/sslkeyfile && quiche-server --listen $addr --cert /server-data/$idx/cert.crt --key /server-data/$idx/cert.key 2>&1 | tee /server-data/$idx/server.log"

docker exec -d quiche-server bash -c "bash /server-data/$idx/start-server.sh"
