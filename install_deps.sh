#!/bin/bash
apt-get update
apt-get install -y libz3-4 cargo #python3-pip
ln -s /usr/lib/x86_64-linux-gnu/libz3.so.4 /usr/lib/x86_64-linux-gnu/libz3.so
#pip3 install --user z3-solver
