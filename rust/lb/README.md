# Simple Load Balancer (Layer 7)

## Goals

- Sends traffic to 2+ servers
- Health checks the server
- Handle a server going offline (failing health check)
- Handle a server coming back online (pass a health check)

## Steps

### Step 1

- [ ] Create a basic server that listens for incoming connects
  - [ ] forward them to a single server
- [x] Create a load balancer program (server) that listens for connections on specific port, and logs out incoming connection
- [x] create a "backend" mock server that listens and logs request, also responds with a simple 200 "Ok"

### Step 2

- [ ] distribute requests between 2+ backend servers using round robin
  - [ ]  Keep list of servers sent to and iterate through them on repeat
