use tokio::fs::File;
use tokio::io::{BufWriter, AsyncWriteExt};
use std::vec;

use futures_util::future;
use tonic::{Request, Status};

use bookstore_grpc::book_store_client::BookStoreClient;
use bookstore_grpc::*;

pub mod bookstore_grpc {
    tonic::include_proto!("bookstore");
}

struct TimedResult {
    client_n: usize,
    request_n: usize,
    time: chrono::DateTime<chrono::Local>,
    result: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
enum Option {
    Search,
    Buy,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "http://localhost:10086";

    let _book_id = 53477;
    let _book_topic = "distributed systems";

    let option = Option::Buy;

    let num_clients = 100; // number of concurrent clients
    let num_requests = 50; // number of sequential requests per client

    let mut handles = vec![];

    let time_start = chrono::Local::now();
    for client_n in 0..num_clients {
        let handle = tokio::spawn(async move {
            let mut client = BookStoreClient::connect(addr)
                .await
                .expect("connect failed");

            let mut results = Vec::<TimedResult>::with_capacity(num_requests);
            for request_n in 0..num_requests {
                let success = match option {
                    Option::Search => {
                        let resp = client
                            .search(Request::new(BookTopicRequest {
                                topic: _book_topic.into(),
                            }))
                            .await?;
                        resp.into_inner().success
                    }
                    Option::Buy => {
                        let resp = client
                            .buy(Request::new(ItemNumberRequest { id: _book_id }))
                            .await?;
                        resp.into_inner().success
                    }
                };

                results.push(TimedResult {
                    client_n,
                    request_n,
                    time: chrono::Local::now(),
                    result: success,
                });
            }
            Ok::<_, Status>(results)
        });
        handles.push(handle);
    }

    let fut = future::join_all(handles).await;
    let time_end = chrono::Local::now();

    // print individual times
    let results = fut
        .into_iter()
        .map(|r| {
            let result = r.unwrap().unwrap();
            result
        })
        .flatten()
        .collect::<Vec<_>>();

    // for r in &results {
    //     println!(
    //         "c{} r{}: [{}] {:?}",
    //         r.client_n, r.request_n, r.time, r.result
    //     );
    // }

    // print time (aggregate)
    println!("time start: {}", time_start);
    println!("time end: {}", time_end);

    let time_diff = time_end - time_start;
    println!(
        "time diff: {}.{}s",
        time_diff.num_seconds(),
        time_diff.num_microseconds().unwrap_or(time_diff.num_milliseconds() * 1000) % 1_000_000
    );

    let num_success = results.iter().filter(|r| r.result).count();
    println!("num success: {}", num_success);

    // write to file
    let mut file = BufWriter::new(File::create("output.csv").await?);
    file.write_all("client_nth, request_nth, time, success\n".as_bytes()).await?;
    for r in results {
        let line = format!(
            "{}, {}, {}, {}\n",
            r.client_n, r.request_n, r.time, r.result
        );
        file.write_all(line.as_bytes()).await?;
    }
    file.flush().await?; // important so the file is actually written to disk

    Ok(())
}
