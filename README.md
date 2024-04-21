# lucos_file_sync
Syncs files between locations for resilience

## Dependencies
* docker
* docker-compose

## Build-time environment variables (needs passing into docker compose)
* __ARCH__ - the CPU architecture of the current environment.  (Can use "local" for dev purposes)

## Runtime environment varibles (set inside docker-compose.yml)
* __PORT__ - The TCP port for the web server to listen on

## Running
`ARCH={architecture} nice -19 docker-compose up -d --no-build`

## Deprecated
This project only ever got as far as a stub.  Useful logic wasn't implemented.  So no point running it at the moment.  May pick it up in future (or may take a different approach instead)