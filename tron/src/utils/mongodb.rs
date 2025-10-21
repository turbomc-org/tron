use mongodb::error::Error as MongoError;
use mongodb::{Client, Database};
use tracing::info;

pub struct MongoDB {
    pub client: Client,
    pub database: Database,
}

impl MongoDB {
    pub async fn new() -> Result<Self, MongoError> {
        let mongo_uri = std::env::var("MONGO_URI").expect("missing MONGO_URI");

        let client = Client::with_uri_str(&mongo_uri).await?;

        let database = client.database("tron");

        info!("✅ Database connected successfully.");

        Ok(MongoDB { client, database })
    }
}
