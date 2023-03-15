bookstore
======

## Writeup

Please refer to [/writeup/writeup.pdf](/writeup/writeup.pdf).

The sample output are found at [/output.md](/output.md)

## Dependencies

The versions listed below are the ones I run on (and tested).
The program *might* run on older versions with no guarantees.

- Rust Toolchain 1.66
  - Cargo (the Rust package manager) will handle all the dependencies
- Go 1.20
  - `make` (to automate compilation of protobuf stub)
  - The packages below will be installed by my Makefile:
    - `protoc-gen-go`
    - `protoc-gen-go-grpc`
  - Go will handle the rest dependencies
- Python 3.11.2
  - `pip` (for manually installing dependencies)
- protoc 3.21


## Compile & Run Instructions

### Run the server

```shell
$ cd server

$ cargo run --bin db-init
# this command creates and initializes the database with initial value
# you can also specify path to database file like this
$ cargo run --bin db-init -- -d database.db

$ cargo run
# Run the server. This is the same to the following command because server is the default bin
$ cargo run --bin server

# You can pass in the port and path to db file like this
$ cargo run -- -d database.db -p 10086

# Alternatively, use the following command to run the server in release mode
$ cargo run --release
```

After you startup, type `help` to view list of available commands and way to use them.

### Run the Go Client

```shell
$ cd client-go

$ make
# it will compile the protobuf stub and the go client

$ ./client-go localhost:10086
# run the client with `client-go [server address]`
```

After you startup, type `help` to view list of available commands and way to use them.

### Run the Python Client

```shell
$ cd client-py

$ python --version
# We've tested our client on latest python (3.11.2). This may not work for python <3.8
# We have specifically modified our file so it doesn't use `switch...case` syntax in Python >3.10
# But we haven't tested in an earlier versions

$ pip install -r requirements.txt
# Install the requirements

$ python -m grpc_tools.protoc -I../proto --python_out=. --pyi_out=. --grpc_python_out=. ../proto/bookstore.proto
# compile the protobuf stub
# might work without this (we have added the compiled stub for python)

$ python main.py localhost:10086
# Runs the script
```
