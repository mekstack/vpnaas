#!/bin/sh

ip link add dev wg0 type wireguard
ip addr add 10.69.69.1/24 dev wg0
ip link set wg0 up
iptables -A FORWARD -i wg0 -j ACCEPT
iptables -t nat -A POSTROUTING -s 10.69.69.0/24 -o eth0 -j MASQUERADE

/usr/local/bin/wgrpc
