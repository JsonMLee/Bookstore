use std::io::Write;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    sync::oneshot::Sender,
};

use rusqlite::{params, TransactionBehavior};

use crate::{log::Logs, rpc_handler::bookstore_grpc::Book, Log};

#[allow(unused_variables)]
pub async fn cmd_interface(
    shutdown_tx: Sender<()>,
    pool: r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>,
    logs: Logs,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = std::io::stdout();
    let mut stdin = BufReader::new(tokio::io::stdin()).lines();

    loop {
        // Print the prompt
        stdout.write_all(b"> ")?;
        stdout.flush()?;

        // Read in next line (blocking) and detect EOF
        let input = stdin.next_line().await?;
        let line = match input {
            Some(line) => line,
            None => {
                println!("EOF Received. Quitting...");
                shutdown_tx.send(()).unwrap();
                break;
            }
        };

        // Handle the commands
        // slice by space and then match the first word
        match line.split_whitespace().next() {
            Some("help") => {
                println!("Usage:");
                println!("  help    \t\t Print this help message.");
                println!("  list    \t\t List all books.");
                println!("  log     \t\t Print out recent transactions.");
                println!("  restock [amount]    \t Restock all books with stock < [amount] to [amount]. Default is 20.");
                println!(
                    "  update <id> <price> \t Update the price of the book with the given ID."
                );
                println!("  quit/exit \t\t Exit the program.");
            }

            Some("list") => {
                // list all books, id, title, topic, stock, price
                match pool.get() {
                    Ok(conn) => {
                        let mut stmt = conn
                            .prepare("SELECT id, title, topic, stock, price FROM BOOKS")
                            .expect("Failed to prepare statement");
                        let book_iter = match stmt.query_map(params![], |row| {
                            Ok(Book {
                                id: row.get(0)?,
                                title: row.get(1)?,
                                topic: row.get(2)?,
                                stock: row.get(3)?,
                                price: row.get(4)?,
                            })
                        }) {
                            Ok(book_iter) => {
                                // filter out errors
                                book_iter.filter_map(|b| b.ok())
                            }
                            Err(e) => {
                                println!("Error occurred while listing books: {}", e);
                                continue;
                            }
                        };

                        for book in book_iter {
                            println!("{:?}", book);
                        }
                    }
                    Err(e) => {
                        println!("Error occurred while connecting to DB: {}", e);
                    }
                }
            }

            Some("log") => {
                let logs_unlocked = logs.lock().unwrap();
                for log in logs_unlocked.iter() {
                    println!("{}", log);
                }
            }

            Some("restock") => {
                let restock_amount = match line.split_whitespace().nth(1) {
                    Some(amount) => amount.parse::<i32>().unwrap_or(20),
                    None => 20,
                };
                println!(
                    "Restocking books with stock < {} to {}...",
                    restock_amount, restock_amount
                );

                match pool.get() {
                    Ok(mut conn) => {
                        // immediate transaction with write lock
                        let tx =
                            match conn.transaction_with_behavior(TransactionBehavior::Immediate) {
                                Ok(tx) => tx,
                                Err(e) => {
                                    println!("Error occurred while starting transaction: {}", e);
                                    continue;
                                }
                            };

                        // get list of books with stock < 20
                        let mut stmt = tx
                            .prepare("SELECT id, stock FROM BOOKS WHERE STOCK < ?")
                            .expect("Failed to prepare statement");
                        let books_to_restock: Vec<_> = match stmt
                            .query_map(params![restock_amount], |row| {
                                Ok((row.get(0)?, row.get(1)?))
                            }) {
                            Ok(book_iter) => {
                                // filter out errors and collect into vector
                                book_iter.filter_map(|b| b.ok()).collect()
                            }
                            Err(e) => {
                                println!("Error occurred while listing books: {}", e);
                                continue;
                            }
                        };

                        if books_to_restock.is_empty() {
                            println!("No book needs restock.");
                        }

                        // set stock to 20 for book_iter
                        let mut stmt2 = tx
                            .prepare("UPDATE BOOKS SET stock = ? WHERE id = ?")
                            .expect("Failed to prepare statement");
                        for (id, old_stock) in books_to_restock.iter() {
                            match stmt2.execute(params![restock_amount, id]) {
                                Ok(_) => {
                                    println!(
                                        "Restocking book {}: stock {} => {}",
                                        id, old_stock, restock_amount
                                    );
                                }
                                Err(e) => {
                                    println!("Error occurred while restocking: {}", e);
                                }
                            }
                        }

                        // drop statements to end borrow of tx
                        drop(stmt);
                        drop(stmt2);

                        // commit transaction
                        match tx.commit() {
                            Ok(_) => {
                                // write to log
                                let mut logs_unlocked = logs.lock().unwrap();
                                for (id, old_stock) in books_to_restock {
                                    logs_unlocked.push(Log::new_restock(
                                        id,
                                        old_stock,
                                        restock_amount,
                                    ))
                                }
                                drop(logs_unlocked);
                            }
                            Err(e) => {
                                println!("Error occurred while committing transaction: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error occurred while connecting to DB: {}", e);
                    }
                }
            }

            Some("update") => {
                let inputs: Vec<_> = line.split_whitespace().collect();
                if inputs.len() != 3 {
                    println!("Invalid update command. Expected format: update <id> <price>");
                    continue;
                }

                let id = match inputs[1].parse::<i32>() {
                    Ok(id) => id,
                    Err(e) => {
                        println!("Invalid id: {}", e);
                        continue;
                    }
                };
                let price = match inputs[2].parse::<f64>() {
                    Ok(price) => {
                        if price < 0.0 {
                            println!("Price must be positive.");
                            continue;
                        }
                        price
                    }
                    Err(e) => {
                        println!("Invalid price: {}", e);
                        continue;
                    }
                };

                match pool.get() {
                    Ok(conn) => {
                        match conn.execute(
                            "UPDATE BOOKS SET PRICE = ? WHERE ID = ?",
                            params![price, id],
                        ) {
                            Ok(0) => {
                                println!("No book with id {} found.", id);
                            }
                            Ok(num) => {
                                println!("Updated book with id {} to price {}. Number of changed lined: {}", id, price, num);
                                logs.lock().unwrap().push(Log::new_price(id, price));
                            }
                            Err(e) => {
                                println!("Error occurred while updating: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error occurred while connecting to DB: {}", e);
                    }
                }
            }

            Some("quit") | Some("exit") => {
                println!("Exiting...");
                shutdown_tx.send(()).unwrap();
                break;
            }
            Some("") | None => {} // ignore empty lines
            Some(x) => {
                println!("Unknown command: {}", x);
                println!("Type in `help` to view the list of supported commands");
            }
        }
    }

    Ok(())
}
