use mongodb::error::Error as MongoError;
use mongodb::{Client, Database};

pub struct MongoDB {
    pub client: Client,
    pub database: Database,
}

impl MongoDB {
    pub async fn new() -> Result<Self, MongoError> {
        let mongo_uri = std::env::var("MONGO_URI").expect("missing MONGO_URI");

        // The '?' operator will propagate any connection errors.
        let client = Client::with_uri_str(&mongo_uri).await?;

        let database = client.database("h01");

        println!("✅ Database connected successfully.");

        Ok(MongoDB { client, database })
    }
}
