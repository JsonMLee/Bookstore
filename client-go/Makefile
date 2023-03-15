.PHONY: client-go
client-go: proto/bookstore.pb.go proto/bookstore_grpc.pb.go
	go build -o client-go .

.PHONY: run
run: client-go
	./client-go

proto/bookstore.pb.go: ../proto/bookstore.proto grpc-go
	if [ ! -d "proto" ]; then mkdir -p proto; fi
	protoc --go_out=./proto --go_opt=paths=source_relative --proto_path=../proto bookstore.proto

proto/bookstore_grpc.pb.go: ../proto/bookstore.proto grpc-go
	if [ ! -d "proto" ]; then mkdir -p proto; fi
	protoc --go-grpc_out=./proto --go-grpc_opt=paths=source_relative --proto_path=../proto bookstore.proto

.PHONY: grpc-go
grpc-go:
ifeq (, $(shell which protoc))
	$(error "No protoc in $(PATH), please install it manually")
endif

ifeq (, $(shell which protoc-gen-go))
	$(info "No protoc-gen-go in $PATH, installing with go")
	go install google.golang.org/protobuf/cmd/protoc-gen-go@v1.28
endif

ifeq (, $(shell which protoc-gen-go-grpc))
	$(info "No protoc-gen-go-grpc in $PATH, installing with go")
	go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@v1.2
endif

.PHONY: clean
clean:
	rm -rf proto
	rm -rf client-go
