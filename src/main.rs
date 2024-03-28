#![allow(unused)]
use controller::{add_review, get_info};
use futures_lite::stream::StreamExt;
use models::Record;
use serde::Deserialize;
use surrealdb::dbs::Response;
use surrealdb::dbs::Status;
use surrealdb::engine::remote::ws::Client;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::Resource;
use surrealdb::sql::Thing;
use surrealdb::Notification;
use surrealdb::Result;
use surrealdb::Surreal;

mod controller;
mod db;
mod models;

#[tokio::main]
pub async fn main() -> surrealdb::Result<()> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    db.use_ns("s1").use_db("s1").await?;
    println!("DB connected");

    let mut stream = db.select("review").live().await?;

    while let Some(notification) = stream.next().await {
        print(notification);
    }

    Ok(())
}

fn print(result: Result<Notification<Record>>) {
    match result {
        Ok(notification) => {
            let action = notification.action;
            let rating = notification.data.rating.unwrap_or_default();
            let review = notification.data.review_text;
            if rating < 2 {
                println!("{action:?} > Rating = {rating:?} Review = {review:?}");
            }
        }
        Err(error) => eprintln!("{error}"),
    }
}
