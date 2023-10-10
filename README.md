# vpnaas

A Blazingly fast VPN as a Service.

Allows users to manage their public keys on Wireguard VPN servers.

Authentication via OpenID tokens from vault.mekstack.ru

## Wg

Manages a Linux kernel wireguard server peers

Wireguard interface must exist before starting the server.

This program uses netlink to modify wireguard interface attributes, so either
use root or add necessary capabilities with `setcap CAP_NET_ADMIN=+eip`.

## Keys

Manages peers, allowed IPs and public keys in a Redis database

The Redis server must be up and running before starting the `keys` microservice.
`ip_pool` must contain available IP addresses.

### Redis

- SET `ip_pool`: available and unallocated ips in decimal representation
- HASH `user:to:ip`: managed by `Keys`
- HASH `ip:to:pubkey`: managed by `Keys`
- HASH `pubkey:to:username`: managed by `Keys`

To init a database do

    echo {172311820..172311850}

    AUTH {{ password }}
    SADD ip_pool {{ ips }}

## Confus

> im confus

Provides wireguard configuration values that can be used to generate wg0.conf

Kinda dumb because all it does is read env and send it over gRPC

## Web

Web application in Svelte that allows user to modify his public key and view wg0.conf.

    protoc -I=../proto/ vpnaas.proto \
        --js_out=import_style=commonjs:./src/grpc \
        --grpc-web_out=import_style=typescript,mode=grpcweb:./src/grpc
    npm install
    npm run dev
