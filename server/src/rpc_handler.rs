use tonic::{Request, Response, Status};

use bookstore_grpc::book_store_server::BookStore;
pub use bookstore_grpc::book_store_server::BookStoreServer;

use bookstore_grpc::*;

use rusqlite::{params, OptionalExtension, TransactionBehavior};

use crate::log::Logs;

pub mod bookstore_grpc {
    tonic::include_proto!("bookstore");
}

#[derive(Debug)]
pub struct BookStoreHandler {
    pool: r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>,
    logs: Logs,
}

impl BookStoreHandler {
    pub fn new(pool: r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>, logs: Logs) -> Self {
        Self { pool, logs }
    }
}

#[tonic::async_trait]
#[allow(unused_variables)]
impl BookStore for BookStoreHandler {
    async fn hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloReply>, Status> {
        // println!("\nGot a request: {:?}", request);
        let reply = HelloReply {
            hello_message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }

    async fn search(
        &self,
        request: Request<BookTopicRequest>,
    ) -> Result<Response<BookListReply>, Status> {
        let topic = request.into_inner().topic;

        let conn = match self.pool.get() {
            Ok(conn) => conn,
            Err(e) => {
                let reply = BookListReply {
                    success: false,
                    message: format!("Error occurred while connecting to DB: {}", e),
                    books: vec![],
                };
                return Ok(Response::new(reply));
            }
        };

        // for simplicity, here we are using strict equal for topic
        let mut stmt = conn
            .prepare("SELECT id, title, topic, stock, price FROM BOOKS WHERE TOPIC = ?")
            .expect("Failed to prepare statement");

        let rows = match stmt.query_map(params![topic], |row| {
            Ok(Book {
                id: row.get(0)?,
                title: row.get(1)?,
                topic: row.get(2)?,
                stock: row.get(3)?,
                price: row.get(4)?,
            })
        }) {
            Ok(rows) => rows,
            Err(e) => {
                let reply = BookListReply {
                    success: false,
                    message: format!("Error occurred while querying DB: {}", e),
                    books: vec![],
                };
                return Ok(Response::new(reply));
            }
        };

        // ignore failed items
        let books: Vec<_> = rows.into_iter().filter_map(|b| b.ok()).collect();

        Ok(Response::new(BookListReply {
            success: true,
            message: "".into(),
            books,
        }))
    }

    async fn lookup(
        &self,
        request: Request<ItemNumberRequest>,
    ) -> Result<Response<BookInfoReply>, Status> {
        let id = request.into_inner().id;

        let conn = match self.pool.get() {
            Ok(conn) => conn,
            Err(e) => {
                let reply = BookInfoReply {
                    success: false,
                    message: format!("Error occurred while connecting to DB: {}", e),
                    book: None,
                };
                return Ok(Response::new(reply));
            }
        };

        let reply = match conn
            .query_row(
                "SELECT id, title, topic, stock, price FROM books WHERE id = ?",
                params![id],
                |row| {
                    Ok(Book {
                        id: row.get(0)?,
                        title: row.get(1)?,
                        topic: row.get(2)?,
                        stock: row.get(3)?,
                        price: row.get(4)?,
                    })
                },
            )
            .optional()
        {
            Ok(Some(book)) => BookInfoReply {
                success: true,
                message: "".into(),
                book: Some(book),
            },
            Ok(None) => BookInfoReply {
                success: false,
                message: "Book not Found".into(),
                book: None,
            },
            Err(e) => BookInfoReply {
                success: false,
                message: format!("Error: {}", e),
                book: None,
            },
        };

        Ok(Response::new(reply))
    }

    async fn buy(
        &self,
        request: Request<ItemNumberRequest>,
    ) -> Result<Response<BuyBookReply>, Status> {
        let id = request.into_inner().id;

        let mut conn = match self.pool.get() {
            Ok(conn) => conn,
            Err(e) => {
                let reply = BuyBookReply {
                    success: false,
                    message: format!("Error occurred while connecting to DB: {}", e),
                };
                return Ok(Response::new(reply));
            }
        };

        // here we use BEGIN IMMEDIATE to acquire a write lock on the database so other connections cannot read or write before this transaction is committed (similar to a mutex, but enforced by DB)
        let tx = match conn.transaction_with_behavior(TransactionBehavior::Immediate) {
            Ok(tx) => tx,
            Err(e) => {
                let reply = BuyBookReply {
                    success: false,
                    message: format!("Error occurred while acquiring DB write lock: {}", e),
                };
                return Ok(Response::new(reply));
            }
        };

        // first lookup if book exists, and if it does, check if there are enough copies
        let mut stmt = tx
            .prepare("SELECT stock FROM books WHERE id = ?")
            .expect("Failed to prepare statement");
        let number = stmt
            .query_row(params![id], |row| row.get::<usize, i32>(0))
            .optional();
        drop(stmt);

        let mut reply = match number {
            Ok(Some(number)) => {
                if number <= 0 {
                    BuyBookReply {
                        success: false,
                        message: "Not enough copies".into(),
                    }
                } else {
                    BuyBookReply {
                        success: true,
                        message: "Book bought".into(),
                    }
                }
            }
            Ok(None) => BuyBookReply {
                success: false,
                message: "Book not found".into(),
            },
            Err(ref e) => BuyBookReply {
                success: false,
                message: format!("Error occurred while looking up the book: {}", e),
            },
        };

        if reply.success {
            // decrement the number of copies
            match tx.execute(
                "UPDATE books SET stock = stock - 1 WHERE id = ?",
                params![id],
            ) {
                Ok(_) => {}
                Err(e) => {
                    reply = BuyBookReply {
                        success: false,
                        message: format!("Error occurred while updating SQL: {}", e),
                    };
                }
            }

            // write to logs
            let old_stock = number.unwrap().unwrap();
            self.logs
                .lock()
                .unwrap()
                .push(crate::Log::new_buy(id, old_stock, old_stock - 1));
        }

        match tx.commit() {
            Ok(_) => {}
            Err(e) => {
                reply = BuyBookReply {
                    success: false,
                    message: format!("Error occurred while committing transaction: {}", e),
                };
            }
        }
        return Ok(Response::new(reply));
    }
}
