# rustdss

> An implementation of something that uses the RESP (Redis) protocol.

It contains the following modules
- `Connection`: deals with incoming connections, exists for the lifetime of a connection
  - Responsible for accepting incoming socket connections, and running parsers on incoming bytestreams to form requests.
- `Transport`: Contains serialisers and deserialisers for transmitting and recieving RESP data
- `Requests`: deals with each indidual request, expists for the lifetime of a request
  - Responsible for interpreting parsed requests, and running them against the core.
- `Core`: deals with the data, exists for the lifetime of the application.
