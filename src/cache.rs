use redis::AsyncCommands;
use serde::{Serialize, de::Deserialize};
use serde_json::{from_str, to_string};
use std::fmt::Debug;

pub struct Client {
    redis_client: redis::Client,
}

impl Client {
    pub async fn set<T>(&self, key: &str, value: T, expiration: u64) -> Result<(), CacheError>
    where
        T: Serialize,
    {
        let value = to_string(&value)?;

        let mut conn = self.redis_client.get_multiplexed_tokio_connection().await?;
        conn.set_ex::<&str, String, ()>(key, value, expiration)
            .await?;

        Ok(())
    }

    pub async fn get<U>(&self, key: &str) -> Result<Option<U>, CacheError>
    where
        U: for<'de> Deserialize<'de> + Debug,
    {
        let mut conn = self.redis_client.get_multiplexed_tokio_connection().await?;
        let value: Option<String> = conn.get(key).await?;

        match value {
            Some(content) => {
                let result: U = from_str(&content)?;
                Ok(Some(result))
            }
            None => Ok(None),
        }
    }
}

impl Default for Client {
    fn default() -> Self {
        let redis_url = std::env::var("REDIS").expect("Need to specify REDIS with the REDIS_URL");
        Self {
            redis_client: redis::Client::open(redis_url).unwrap(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CacheError {
    #[error("Error from Serde: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Error accessing database: {0}")]
    Database(#[from] redis::RedisError),
}

#[cfg(test)]
mod tests {
    use playground_api::endpoints::VersionsResponse;

    use super::*;

    #[tokio::test]
    async fn success() {
        let redis_client = Client::default();
        let play_client = playground_api::Client::default();

        let res = play_client.versions().await.unwrap();
        redis_client
            .set("success_test", res.clone(), 60)
            .await
            .unwrap();

        let cached_res = redis_client
            .get::<VersionsResponse>("success_test")
            .await
            .unwrap();

        assert_eq!(Some(res), cached_res);
    }
}
