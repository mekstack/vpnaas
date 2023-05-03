#!/bin/sh

ip link add dev wg0 type wireguard

/usr/local/bin/wgrpc
