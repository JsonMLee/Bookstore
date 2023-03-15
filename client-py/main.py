

import logging
import sys

import grpc

# python -m grpc_tools.protoc -I../proto --python_out=. --pyi_out=. --grpc_python_out=. ../proto/bookstore.proto
import bookstore_pb2
import bookstore_pb2_grpc

def run():
    server_addr = ""

    if len(sys.argv) != 2:
        print("[WARN] Using default address localhost:10086")
        print("[WARN]   Usage: python3 main.py <server address>")
        server_addr = "localhost:10086"
    else:
        server_addr = sys.argv[1]
        print(f"[INFO] connecting to server {sys.argv[1]}")

    channel = grpc.insecure_channel(server_addr)
    stub = bookstore_pb2_grpc.BookStoreStub(channel)
    print(">", end =" ")
    sys.stdout.flush()
    
    

    for line in sys.stdin:
        line = line.rstrip()
        parts = line.split(" ", 1)

        if parts[0] == "exit" or parts[0] == "quit":
            sys.exit(0)
        elif parts[0] == "ping":
            response = stub.Hello(bookstore_pb2.HelloRequest(name="Python Client"))
            print(response.hello_message)
        elif parts[0] == "search":
            if len(parts) != 2:
                print("Usage: search <topic>")
                print(">", end = " ")
                sys.stdout.flush()
                break
            response = stub.Search(bookstore_pb2.BookTopicRequest(topic=parts[1]))
            books = response.books
            if len(books) == 0:
                print("No books found under \"{0}\"".format(parts[1]))
            if response.success:
                
                for book in books:
                    print("Book {0}: {1}. Topic: {2}. Price: {3:.2f}. Stock: {4}.".format(book.id, book.title, book.topic, book.price, book.stock)) #finish
            else:
                print("failed to search for topic {0}: {1}".format(parts[1], response.message()))
        elif parts[0] == "lookup":
            if len(parts) != 2:
                print("Usage: lookup <id>")
                print(">", end = " ")
                sys.stdout.flush()
                break
            num = int(parts[1])
            response = stub.Lookup(bookstore_pb2.ItemNumberRequest(id = num))
            if response.success:
                book = response.book
                print("Book {0}: {1}. Topic: {2}. Price: {3:.2f}. Stock: {4}.".format(book.id, book.title, book.topic, book.price, book.stock)) #finish
            else:
                print("failed to find book {0}: {1}".format(num, response.message))
        elif parts[0] == "buy":
            if len(parts) != 2:
                print("Usage: buy <id>")
                print(">", end = " ")
                sys.stdout.flush()
                break
            num = int(parts[1])
            response = stub.Buy(bookstore_pb2.ItemNumberRequest(id = num))
            if response.success:
                print("bought book {0}".format(num))
            else:
                print("failed to buy book {0} with error {1}".format(num, response.message))
        elif parts[0] == "help":
            print("Commands:")
            print("  ping  \t\t Test connection to server")
            print("  search <topic> \t Search for books by topic")
            print("  lookup <id> \t\t Lookup book by id")
            print("  buy <id> \t\t Buy book by id")
            print("  help \t\t\t Show this help message")
            print("  exit \t\t\t Exit the program")
        elif parts[0] == "":
            continue
        else:
            print(f"command {parts[0]} not supported")

        # prepare for next line            
        print(">", end =" ")
        sys.stdout.flush()

            


if __name__ == '__main__':

    run()
    # arg parse
    