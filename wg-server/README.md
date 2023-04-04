# wg-server

Service that adds users to wireguard server peers

## Design

1. At startup: find wg0 interface or exit with error
2. Ask `keys` microservice for private key
3. Add private key to the `wg0` interface
4. Query peers from `keys` microservice
5. Add peers to the `wg0` interfaces

## Features

- Endpoint to add or delete peers
- Healthcheck endpoint
- Metrics endpoint
