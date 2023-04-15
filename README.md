# vpnaas

## Expected redis structure

- SET available_ips
- HASH user:to:allowed_ip
- HASH allowed_ip:to:pubkey
