#!/bin/bash
apt-get update
apt-get install -y libz3-4 cargo
cp dependencies/libz3.so /usr/lib/x86_64-linux-gnu/libz3.so
