# rustdss

> An implementation of something like Redis that uses exactly the same protocol, but with threads and memory safety.

**This is very much a pet project and should absolutely not be used in production, unless you want production to break and/or have terrible performance. I would love for this to be used eventually, but there's a mountain of work to do before that would be possible, so please feel free to raise issues and PRs to help eventually make this into something useful**

It contains the following modules
- `Connection`: deals with incoming connections, exists for the lifetime of a connection
  - Responsible for accepting incoming socket connections, and running parsers on incoming bytestreams to form requests.
- `Transport`: Contains serialisers and deserialisers for transmitting and recieving RESP data
- `Requests`: deals with each indidual request, expists for the lifetime of a request
  - Responsible for interpreting parsed requests, and running them against the core.
- `Core`: deals with the data, exists for the lifetime of the application.

# Running it and testing it
This uses the redis protocol, so you can test it by doing this:
```bash
cargo run --release &

redis-benchmark -t set,get,ping -r 10000 -n 1000000

```

# Project Roadmap
- Increase command coverage
  - Start serialising lists
    - as a stream (this should result in a big performance increase)
  - handle different data types in the backing store
    - starting with list operations
    - Then sets
    - Then maps
- Refactor the `core` module into a separate crate, so it can be embedded.
- Increase underlying datastructure performance
  - Use a radix tree to support lower O operations.
- Basic pubsub stuff
  - need to support blocking commands/responses

