# tinytrix

A small and simplistic addition for running a fully federated matrix homeserver behind `traefik`

## Motivation

A fully federated matrix homeserver shall serve two endpoints:

| endpoint                              | expected response                                       |
| ------------------------------------- | ------------------------------------------------------- |
| `domain.io/.well-known/matrix/client` | `{"m.homeserver":{"base_url":"https://domain.io"}},200` |
| `domain.io/.well-known/matrix/server` | `{"m.server":"domain.io:443"},200`                      |

Configuring `nginx` to return these responses is trivial. Configuring `traefik` less so.
This microservice can serve as that missing endpoint behind a `traefik` reverse proxy.

## Usage

**MX_HOSTNAME** represents the domain your homeserver is exposed on.

```sh
# using cargo
MX_HOSTNAME=domain.io cargo run
# check if it works
curl localhost/.well-known/matrix/client
# should return {"m.homeserver":{"base_url":"https://domain.io"}}

# using docker
docker run --rm -ti -e MX_HOSTNAME=domain.io ghcr.io/fabius/tinytrix

# using docker-compose.yml
version: "3"
services:
  traefik:
    image: traefik:v2.4
    command:
      - --providers.docker=true
      - --entrypoints.web.address=:80
      - --entrypoints.websecure.address=:443
    ports:
      - 80:80
      - 443:443
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock:ro
  tinytrix:
    depends_on:
      - traefik
    image: ghcr.io/fabius/tinytrix
    restart: always
    environment:
      - MX_HOSTNAME=actix
    labels:
      - traefik.enable=true
      # route requests going to domain.io/.well-known/* over to tinytrix
      - traefik.http.routers.tinytrix.rule=Host(`domain.io`) && PathPrefix(`/.well-known`)
      - traefik.http.routers.tinytrix.service=tinytrix
      - traefik.http.routers.tinytrix.entrypoints=web
      # connect traefik to tinytrix port 8080
      - traefik.http.services.tinytrix.loadbalancer.server.port=8080
```
