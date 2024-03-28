use serde::Deserialize;
use surrealdb::sql::Thing;



#[derive(Debug, Deserialize)]
pub struct Record {
    pub id: Thing,
    pub rating: Option<u8>,
    pub review_text: String,
}

