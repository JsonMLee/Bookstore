# Sample outputs

## Sample output for server

```shell
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.37s
     Running `target/debug/server`
[DEBUG] using Args { port: 10086, database: "database.db" }
Bookstore Server listening on [::]:10086
> help
Usage:
  help                   Print this help message.
  list                   List all books.
  log                    Print out recent transactions.
  restock [amount]       Restock all books with stock < [amount] to [amount]. Default is 20.
  update <id> <price>    Update the price of the book with the given ID.
  quit/exit              Exit the program.
> list
Book { id: 53477, title: "Achieve Less Bugs and More Hugs in CSCI 339", topic: "distributed systems", stock: 0, price: 29.99 }
Book { id: 53573, title: "Distributed Systems for Dummies", topic: "distributed systems", stock: 1000, price: 24.99 }
Book { id: 12365, title: "Surviving College", topic: "college life", stock: 1000, price: 19.99 }
Book { id: 12498, title: "Cooking for the Impatient Undergraduate", topic: "college life", stock: 1000, price: 15.99 }
> restock 20
Restocking books with stock < 20 to 20...
Restocking book 53477: stock 0 => 20
> list
Book { id: 53477, title: "Achieve Less Bugs and More Hugs in CSCI 339", topic: "distributed systems", stock: 20, price: 29.99 }
Book { id: 53573, title: "Distributed Systems for Dummies", topic: "distributed systems", stock: 1000, price: 24.99 }
Book { id: 12365, title: "Surviving College", topic: "college life", stock: 1000, price: 19.99 }
Book { id: 12498, title: "Cooking for the Impatient Undergraduate", topic: "college life", stock: 1000, price: 15.99 }
> update 53477 35.99
Updated book with id 53477 to price 35.99. Number of changed lined: 1
> list
Book { id: 53477, title: "Achieve Less Bugs and More Hugs in CSCI 339", topic: "distributed systems", stock: 20, price: 35.99 }
Book { id: 53573, title: "Distributed Systems for Dummies", topic: "distributed systems", stock: 1000, price: 24.99 }
Book { id: 12365, title: "Surviving College", topic: "college life", stock: 1000, price: 19.99 }
Book { id: 12498, title: "Cooking for the Impatient Undergraduate", topic: "college life", stock: 1000, price: 15.99 }
> exit
Exiting...
```

## Sample output for Go Client

```shell
$ ./client-go localhost:10086
> help
Commands:
  ping                   Test connection to server
  search <topic>         Search for books by topic
  lookup <id>            Lookup book by id
  buy <id>               Buy book by id
  help                   Show this help message
  exit                   Exit the program
> ping
hello_message:"Hello Go Client!"
> search distributed systems
Book 53477: Achieve Less Bugs and More Hugs in CSCI 339. Topic: distributed systems. Price: 35.99. Stock: 20.
Book 53573: Distributed Systems for Dummies. Topic: distributed systems. Price: 24.99. Stock: 1000.
> lookup 53477
Book 53477: Achieve Less Bugs and More Hugs in CSCI 339. Topic: distributed systems. Price: 35.99. Stock: 20.
> buy 53477
bought book 53477
> exit
Exiting...
```

## Sample Output for Python Client

```shell
> search distributed systems
Book 53477: Achieve Less Bugs and More Hugs in CSCI 339. Topic: distributed systems. Price: 29.99. Stock: 18.
Book 53573: Distributed Systems for Dummies. Topic: distributed systems. Price: 24.99. Stock: 20.
> buy 53477
bought book 53477
> lookup 53477
Book 53477: Achieve Less Bugs and More Hugs in CSCI 339. Topic: distributed systems. Price: 29.99. Stock: 17.
```
