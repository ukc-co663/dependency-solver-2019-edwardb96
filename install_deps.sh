#!/bin/bash
#apt-get update
apt-get install -y python3 python3-pip z3 #libz3-4 cargo
#cp dependencies/libz3.so /usr/lib/x86_64-linux-gnu/libz3.so
pip3 install --user z3-solver
