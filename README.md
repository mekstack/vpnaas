# vpnaas

## Redis

- SET `ip_pool` -- available and unallocated ips in decimal
- HASH `user:to:ip` -- maps user to allowed_ip
- HASH `ip:to:pubkey` -- wireguard peers

## Wg

Wireguard Server with gRPC peer management

### Configuration

- `WG_SERVER_PRIVKEY`: a base64 encoded 32 bit private key without padding

Wireguard interface `wg0` must exist

sudo or `setcap CAP_NET_ADMIN=+eip`

## Keys

Manages users, allowed_ips and public keys

### Configuration

- `JWT_SECRET_KEY`: for JWT signing

## Confus

> im confus

Generates wg0.conf for user

### Configuration

- `WG_SERVER_ENDPOINT`: publicly accessible address of wireguard server
- `WG_SERVER_PUBKEY`: pulic key of wireguard server peer
- `ALLOWED_IPS`: space separated list of networks that will be accessible via wireguard server
- `KEYS_URL`: url of Keys microservice
- `DNS_CONFIG`: string that will go into DNS field of wg0.conf

## web

    protoc -I=../proto/ vpnaas.proto \
        --js_out=import_style=commonjs:./src/grpc \
        --grpc-web_out=import_style=typescript,mode=grpcweb:./src/grpc
    npm install
    npm run dev
