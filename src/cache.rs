use std::fmt::Debug;

use crate::Error;
use redis::AsyncCommands;
use serde::{Serialize, de::Deserialize};
use serde_json::{from_str, to_string};

pub struct Client {
    redis_client: redis::Client,
}

impl Client {
    pub async fn set<T>(&self, key: &str, value: T, expiration: u64) -> Result<(), Error>
    where
        T: Serialize,
    {
        let value = to_string(&value)?;

        let mut conn = self.redis_client.get_multiplexed_tokio_connection().await?;
        conn.set_ex::<&str, String, ()>(key, value, expiration)
            .await?;

        Ok(())
    }

    pub async fn get<U>(&self, key: &str) -> Result<Option<U>, Error>
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
        Self {
            redis_client: redis::Client::open("redis://127.0.0.1/").unwrap(),
        }
    }
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
