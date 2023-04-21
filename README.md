# vpnaas

## wg

Wireguard Server that adds peers via gRPC

### Configuration

- env `$WG_SERVER_PRIVKEY`: a base64 encoded 32 bit private key without padding
- wireguard interface `wg0`
- `setcap CAP_NET_ADMIN=+eip`

## Expected redis structure

- SET ip_pool -- available and unallocated ips in decimal
- HASH user:to:ip -- maps user to allowed_ip
- HASH ip:to:pubkey -- wireguard peers

## Test

    cargo test -- --test-threads=1

## web

    protoc -I=../proto/ vpnaas.proto \
        --js_out=import_style=commonjs:./src/grpc \
        --grpc-web_out=import_style=typescript,mode=grpcweb:./src/grpc
    npm install
    npm run dev
