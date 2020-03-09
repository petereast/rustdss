# RUSTDSS

> An implementation of something that uses the RESP (Redis) protocol.

It contains the following modules
- `Connection`: deals with incoming connections, exists for the lifetime of a connection
- `Requests`: deals with each indidual request, expists for the lifetime of a request
  - Responsible for parsing requests, and running them against the core.
- `Core`: deals with the data, exists for the lifetime of the application.
