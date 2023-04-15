# vpnaas

## Expected redis structure

- SET available_ips
- HASH user:to:allowed_ip
- HASH allowed_ip:to:pubkey

### Flows

#### Add a new `{{ user }}` with `{{ pubkey }}`

1. `SPOP` an `ip` from `available_ips`
2. `HSET` `user:to:allowed_ip` to `{{ user }}`:`ip`
2. `HSET` `allowed_ip:to:pubkey` to `ip`:`{{ pubkey }}`
