package main

import (
	"bufio"
	"context"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"

	pb "cs.williams.edu/cs339-s23/23jml8-24ys5/bookstore/client-go/proto" // generated protobuf stub

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

func main() {
	// set default server address if args not passed in
	if len(os.Args) < 2 {
		fmt.Println("[WARN] Server address not found")
		fmt.Println("[WARN]     Usage: client-go [server address]")
		fmt.Println("[WARN] Setting default server address to localhost:10086...")
		os.Args = append(os.Args, "localhost:10086")
	}

	// Connect to the server
	conn, err := grpc.Dial(os.Args[1], grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		log.Fatalf("fail while connecting to server: %v", err)
	}
	defer conn.Close()

	// Create a new client and test connection
	client := pb.NewBookStoreClient(conn)
	_, err = client.Hello(context.Background(), &pb.HelloRequest{
		Name: "Go Client",
	})
	if err != nil {
		fmt.Printf("Fail during test connection: %v\n", err)
		os.Exit(1)
	}

	scanner := bufio.NewScanner(os.Stdin)

CmdLoop:
	for {
		// Print a prompt for the user
		fmt.Print("> ")

		// Read the user's input
		scanner.Scan()
		input := scanner.Text()

		// Split the user's input into a command and its arguments
		parts := strings.SplitN(input, " ", 2)

		// Call the appropriate function based on the command
		switch parts[0] {
		case "exit", "quit":
			fmt.Println("Exiting...")
			break CmdLoop

		case "ping":
			response, err := client.Hello(context.Background(), &pb.HelloRequest{
				Name: "Go Client",
			})
			if err != nil {
				fmt.Printf("Fail while ping: %v\n", err)
				continue
			}
			fmt.Println(response)

		case "search":
			if len(parts) != 2 {
				fmt.Println("Usage: search <topic>")
				continue
			}

			response, err := client.Search(context.Background(), &pb.BookTopicRequest{
				Topic: parts[1],
			})
			if err != nil {
				fmt.Printf("Fail while search: %v\n", err)
				continue
			}

			if response.GetSuccess() {
				books := response.GetBooks()
				for _, book := range books {
					fmt.Printf("Book %d: %s. Topic: %s. Price: %.2f. Stock: %d.\n", book.GetId(), book.GetTitle(), book.GetTopic(), book.GetPrice(), book.GetStock())
				}
			} else {
				fmt.Printf("failed to search for topic %s: %v\n", parts[1], response.GetMessage())
			}

		case "lookup":
			if len(parts) != 2 {
				fmt.Println("Usage: lookup <id>")
				continue
			}

			num, err := strconv.Atoi(parts[1])
			if err != nil {
				fmt.Println("Invalid id")
				continue
			}

			response, err := client.Lookup(context.Background(), &pb.ItemNumberRequest{
				Id: int32(num),
			})
			if err != nil {
				fmt.Printf("Fail while lookup: %v\n", err)
				continue
			}

			if response.GetSuccess() {
				book := response.GetBook()
				fmt.Printf("Book %d: %s. Topic: %s. Price: %.2f. Stock: %d.\n", book.GetId(), book.GetTitle(), book.GetTopic(), book.GetPrice(), book.GetStock())
			} else {
				fmt.Printf("failed to find book %d: %v\n", num, response.GetMessage())
			}

		case "buy":
			if len(parts) != 2 {
				fmt.Println("Usage: buy <id>")
				continue
			}

			num, err := strconv.Atoi(parts[1])
			if err != nil {
				fmt.Println("Invalid id")
				continue
			}

			response, err := client.Buy(context.Background(), &pb.ItemNumberRequest{
				Id: int32(num),
			})
			if err != nil {
				fmt.Printf("Fail while buy: %v\n", err)
				continue
			}

			if response.GetSuccess() {
				fmt.Printf("bought book %d\n", num)
			} else {
				fmt.Printf("failed to buy book %d with error %v\n", num, response.GetMessage())
			}

		case "help":
			fmt.Println("Commands:")
			fmt.Println("  ping  \t\t Test connection to server")
			fmt.Println("  search <topic> \t Search for books by topic")
			fmt.Println("  lookup <id> \t\t Lookup book by id")
			fmt.Println("  buy <id> \t\t Buy book by id")
			fmt.Println("  help \t\t\t Show this help message")
			fmt.Println("  exit \t\t\t Exit the program")

		case "":
			// ignore empty lines

		default:
			// usage
			fmt.Println("Unknown command. Type 'help' for a list of commands.")
		}
	}
}
