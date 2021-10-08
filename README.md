# G A L A X Y

A collection of rust microservices + gRPC + graphql leveraging dgraph database

To build the project/workspace, run the following from the root

```
# builds all members of the workspace
$ cargo build

# build a specific workspace member (e.g. jupiter service)
cargo run -p jupiter
```

Then start the service with the following

```sh
$ docker compose build
$ docker compose up

# tear down
$ docker compose down
```

Test the grpc server using [evans](https://github.com/ktr0731/evans)

```sh
$ evans --proto path/to/proto/file.proto

  ______
 |  ____|
 | |__    __   __   __ _   _ __    ___
 |  __|   \ \ / /  / _. | | '_ \  / __|
 | |____   \ V /  | (_| | | | | | \__ \
 |______|   \_/    \__,_| |_| |_| |___/

 more expressive universal gRPC client


ping.PingService@127.0.0.1:50051>
```

Call the ping service using [evans](https://github.com/ktr0731/evans).

```sh
ping.PingService@127.0.0.1:50051> call Ping

{
  "data": {
    "name": "dgraph",
    "version": "v21.03.2"
  },
  "id": "someid",
  "msg": "pong",
  "ts": {
    "created": "2021-10-08T16:36:57.256842Z",
    "updated": "2021-10-08T16:36:57.256842Z"
  },
  "version": "0.1.0"
}
```



### Todo

- [x] add telemetry / tracing
- [x] add docker files
- [ ] build protos at microservice; decentralize proto build
