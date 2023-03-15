mod log;
pub use log::Log;

mod rpc_handler;
use rpc_handler::{BookStoreHandler, BookStoreServer};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tonic::transport::Server;

mod cmd;
use cmd::cmd_interface;

use futures_util::FutureExt; // for using map on a future
use tokio::sync::oneshot;

use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::OpenFlags;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// port for gRPC server to listen on
    #[arg(short, long, default_value_t = 10086)]
    port: i32,

    /// path to the SQLite database file
    #[arg(short, long, default_value_t = {"database.db".to_string()})]
    database: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("[DEBUG] using {:?}", args);

    // a channel to shutdown RPC server from the user cmd
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    // if file does not exist, exit
    if !std::path::Path::new(&args.database).exists() {
        println!("[ERROR] database file {} does not exist. Please run `cargo run --bin db-init -- -d {}` to fill database with value.", &args.database, &args.database);
        return Ok(());
    }

    // initialize the sqlite connection pool. Flags are set to use the "multi-thread" SQLite isolation mode
    let manager = SqliteConnectionManager::file(&args.database).with_flags(
        OpenFlags::SQLITE_OPEN_READ_WRITE
            | OpenFlags::SQLITE_OPEN_CREATE
            | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    );
    let pool = r2d2::Pool::new(manager).unwrap();

    {
        // if DB doesn't contain the BOOKS table, exit
        let conn = pool.get().unwrap();
        let mut stmt =
            conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name=\"BOOKS\"")?;
        if !stmt.exists(rusqlite::params![])? {
            println!("[ERROR] database file {} does not contain the required tables. Please run `cargo run --bin db-init -- -d {}` to fill database with value.", &args.database, &args.database);
            return Ok(());
        }
    }

    // a shared and locked vector to store transaction logs
    let logs = Arc::new(Mutex::new(Vec::<Log>::new()));

    let addr: SocketAddr = format!("[::]:{}", &args.port).parse()?;
    let handler = BookStoreHandler::new(pool.clone(), logs.clone());
    println!("Bookstore Server listening on {}", addr);

    // run the user cmd in a separate thread. It will send a signal to the channel on exit
    tokio::spawn(async {
        cmd_interface(shutdown_tx, pool, logs)
            .await
            .expect("Command-line failed due to an unrecoverable error");
    });

    // Run the gRPC server, which graceful shutdowns if receives signal from channel
    Server::builder()
        .add_service(BookStoreServer::new(handler))
        // the .map(drop) is a dirty hack to convert oneshot::Receiver<()> from a future that returns Result<(), RecvError> to a future that returns ()
        // learned this trick from tonic's source code
        .serve_with_shutdown(addr, shutdown_rx.map(drop))
        .await?;

    Ok(())
}
