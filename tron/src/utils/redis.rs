#[derive(Clone)]
pub struct Redis {
    pub client: redis::Client,
}

impl Redis {
    pub fn new() -> Self {
        let redis_uri = std::env::var("REDIS_URI").expect("missing REDIS_URI");
        let client = redis::Client::open(redis_uri).expect("Failed to connect to Redis");
        Self { client }
    }

    pub async fn get_connection(&self) -> redis::aio::MultiplexedConnection {
        self.client
            .get_multiplexed_async_connection()
            .await
            .expect("Failed to get Redis connection")
    }
}
