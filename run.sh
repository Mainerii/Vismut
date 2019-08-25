#!/bin/sh
set -m
if cargo b --release ; then
sudo setcap cap_net_admin=eip $PWD/target/release/vismut
$PWD/target/release/vismut &
pid=$!
sudo ip addr add 192.168.0.50/24 dev tun0
sudo ip link set up dev tun0
fg
fi