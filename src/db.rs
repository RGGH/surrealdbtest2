use surrealdb::dbs::Response;
use surrealdb::dbs::Status;
use surrealdb::engine::remote::ws::Client;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::Surreal;

// Start surreal server first!
#[tokio::main]
pub async fn connect() -> surrealdb::Result<()> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    db.use_ns("s1").use_db("s1").await?;
    println!("DB connected");
    Ok(())
}
