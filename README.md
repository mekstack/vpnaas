# vpnaas

A Blazingly fast VPN as a Service.

Allows users to manage their public keys on Wireguard VPN servers.

Authentication via [blau](https://github.com/mekstack/saas).

## Wg

Manages a Linux kernel wireguard server peers

### Configuration

- `VPNAAS_GRPC_PORT`: port that the gRPC server will listen on (default: 80)
- `VPNAAS_GRPC_KEYS_URL`: url for connection to the `Keys` microservice (default: 'http://keys:80')
- `VPNAAS_WGDEVICE_NAME`: name for a wireguard interface (default: 'wg0')
- `VPNAAS_WGDEVICE_PORT`: port for a wireguard interface to listen on (default: 51820)
- `VPNAAS_WGDEVICE_PRIVKEY`: a base64 encoded 32 bit private key without padding

Wireguard interface must exist before starting the server.

This program uses netlink to modify wireguard interface attributes, so either
use root or add necessary capabilities with `setcap CAP_NET_ADMIN=+eip`.

## Keys

Manages peers, allowed IPs and public keys in a Redis database

### Configuration

- `VPNAAS_GRPC_PORT`: port that the gRPC server will listen on (default: 80)
- `VPNAAS_GRPC_WG_URL`: url for connection to the `Wg` microservice (default: 'http://wg:80')
- `VPNAAS_JWT_SECRET_KEY`: key for verification of user provided JWT access tokens
- `VPNAAS_REDIS_USERNAME`: username for connecting to the Redis instance (default: 'keys')
- `VPNAAS_REDIS_PASSWORD`: password for connecting to the Redis instance
- `VPNAAS_REDIS_HOSTNAME`: hostname or IP address of the Redis instance (default: 'redis')
- `VPNAAS_REDIS_PORT`: port number for connecting to the Redis instance (default: 6379)
- `VPNAAS_REDIS_DATABASE`: database number to use within the Redis instance (default: 0)

The Redis server must be up and running before starting the `keys` microservice.
`ip_pool` must contain available IP addresses.

### Redis

- SET `ip_pool`: available and unallocated ips in decimal representation
- HASH `user:to:ip`: managed by `Keys`
- HASH `ip:to:pubkey`: managed by `Keys`
- HASH `pubkey:to:username`: managed by `Keys`


## Confus

> im confus

Provides wireguard configuration values that can be used to generate wg0.conf

### Configuration

- `VPNAAS_GRPC_PORT`: port that the gRPC server will listen on (default: 80)
- `VPNAAS_GRPC_KEYS_URL`: url for connection to the `Keys` microservice (default: 'http://keys:80')
- `VPNAAS_CONFIG_ENDPOINT`: Wireguard server endpoint address (IP or hostname)
- `VPNAAS_CONFIG_PUBKEY`: Wireguard server public key
- `VPNAAS_CONFIG_DNS`: DNS server configuration for Wireguard clients
- `VPNAAS_CONFIG_ALLOWED_IPS`: space-separated list of IP addresses that will be routed via wireguard server for Wireguard clients

Kinda dumb because all it does is read env and send it over gRPC

## Web

Web application in Svelte that allows user to modify his public key and view wg0.conf.

    protoc -I=../proto/ vpnaas.proto \
        --js_out=import_style=commonjs:./src/grpc \
        --grpc-web_out=import_style=typescript,mode=grpcweb:./src/grpc
    npm install
    npm run dev
