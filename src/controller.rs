use crate::models::Record;
use colored::Colorize;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Thing;
use surrealdb::Surreal;


pub async fn add_review(db: &Surreal<Client>, rtg: i32) -> surrealdb::Result<()> {
    db.query("CREATE review SET rating=$rating, review_text='mr moo boo!';")
        .bind(("rating", rtg))
        .await?;
    let mut display = db.query("SELECT * from review;").await?;
    let entries: Vec<Record> = display.take(0)?;
    for entry in entries {
        let rating = entry.rating.unwrap_or_default();
        println!("Review:{} Rating:{:?}", entry.review_text.red(), rating);
    }

    Ok(())
}

pub async fn get_info(db: &Surreal<Client>) -> surrealdb::Result<()> {
    let mut info = db.query("INFO FOR DB;").await?;
    println!("{:?}", info);
    Ok(())
}
