# vpnaas

## wg

Wireguard Server that adds peers via gRPC

### Configuration

- env `$WG_SERVER_PRIVKEY`: a base64 encoded 32 bit private key
- wireguard interface `wg0`
- `setcap CAP_NET_ADMIN=+eip`

## Expected redis structure

> Validation not required for data received from `Keys` as it ensures it's correctness.
> Pubkey.bytes is always 32 bytes long, username is never ""

- SET available_ips -- available and unallocated ips
- HASH user:to:allowed_ip -- maps user to allowed_ip
- HASH allowed_ip:to:pubkey -- wireguard peers

### Flows

#### Add a new `{{ user }}` with `{{ pubkey }}`

1. `SPOP` an `ip` from `ip_pool`
2. `HSET` `user:to:allowed_ip` to `{{ user }}`:`ip`
2. `HSET` `allowed_ip:to:pubkey` to `ip`:`{{ pubkey }}`

## Test

    cargo test -- --test-threads=1
