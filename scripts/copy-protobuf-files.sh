#!/usr/bin/env bash
staging_path="/home/scouture/HTTP-3-Prioritization/quiche/protobuf_files"
rm $staging_path/*

cp $1/*save $staging_path/ 
