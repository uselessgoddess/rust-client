#![feature(bench_black_box)]

use reqwest::Client;
use serde_json::{json, Value};
use std::error::Error;
use std::future::Future;
use std::hint::black_box;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::mpsc;

const TASKS: usize = 1000;

async fn delay<T: Future>(name: &'static str, f: impl Fn() -> T) {
    let instant = Instant::now();
    let _ = f().await;
    println!("{}: {:?}", name, instant.elapsed());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut args: Vec<_> = std::env::args().collect();
    let url = format!("http://localhost:{}/query", args.remove(1));
    println!("{}", url);

    let client = Arc::new(reqwest::Client::new());

    delay("big write", || big_write(client.clone(), url.clone())).await;
    //delay("big read", || big_read(client.clone(), url.clone())).await;
    //
    //delay("small write", || small_write(client.clone(), url.clone())).await;
    //delay("small read", || small_read(client.clone(), url.clone())).await;

    Ok(())
}

async fn big_write(client: Arc<Client>, url: String) {
    let (tx, mut rx) = mpsc::channel(2048);

    for i in 0..TASKS {
        let url = url.clone();
        let client = client.clone();
        let tx = tx.clone();
        tokio::spawn(async move {
            let response = client
                .post(&url)
                .json(&json!({
                    "query":
                        format!(
                            r#"
                            mutation {{
                                big(id: "{}", data: {{
                                    data: [
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                        "biiiiiiiiiiiiiiiiiiiiiiiiiiiiig",
                                    ]
                            }})
                        }}
                    "#,
                            i
                        )
                }))
                .send()
                .await
                .unwrap();
            let json: Value = response.json().await.unwrap();
            tx.send(json).await.unwrap();
        });
    }

    let mut count = 0;
    while let Some(message) = rx.recv().await {
        count += 1;
        if count == TASKS {
            return;
        }
    }
}
/*
async fn small_write(client: Arc<Client>, url: String) {
    let mut pool = vec![];
    for i in 0..TASKS {
        let client = client.clone();
        let url = url.clone();
        pool.push(tokio::spawn(async move {
            let response = client
                .post(&url)
                .json(&json!({
                    "query":
                        format!(
                            r#"
                        mutation {{
                            small(id: "{}", data: {{
                                data: {}
                            }})
                        }}
                    "#,
                            i, i
                        )
                }))
                .send()
                .await
                .unwrap();
            let json: Value = response.json().await.unwrap();
            black_box(json);
        }));
    }

    for task in pool {
        task.await.unwrap();
    }
}

async fn big_read(client: Arc<Client>, url: String) {
    let mut pool = vec![];
    for i in 0..TASKS {
        let client = client.clone();
        let url = url.clone();
        pool.push(tokio::spawn(async move {
            let response = client
                .post(&url)
                .json(&json!({
                    "query":
                        format!(
                            r#"
                        query {{
                            big(id: "{}") {{
                                data
                            }}
                        }}
                    "#,
                            i
                        )
                }))
                .send()
                .await
                .unwrap();
            let json: Value = response.json().await.unwrap();
            black_box(json);
        }));
    }

    for task in pool {
        task.await.unwrap();
    }
}

async fn small_read(client: Arc<Client>, url: String) {
    let mut pool = vec![];
    for i in 0..TASKS {
        let client = client.clone();
        let url = url.clone();
        pool.push(tokio::spawn(async move {
            let response = client
                .post(&url)
                .json(&json!({
                    "query":
                        format!(
                            r#"
                        query {{
                            small(id: "{}") {{
                                data
                            }}
                        }}
                    "#,
                            i
                        )
                }))
                .send()
                .await
                .unwrap();
            let json: Value = response.json().await.unwrap();
            black_box(json);
        }));
    }

    for task in pool {
        task.await.unwrap();
    }
}
*/
