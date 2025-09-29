use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::{Display, Formatter};
use url::form_urlencoded;

use crate::configuration::{DatabaseSettings, get_config};

#[derive(Debug, Deserialize)]
pub struct SQLError {
    query: String,
    error: String,
    position: i32,
}

#[derive(Debug)]
pub enum Error {
    DatabaseError(questdb::Error),
    DeserializeError(serde_json::error::Error),
    ExecError(reqwest::Error),
    SQLError(SQLError),
    NoDataFound,
    UnsupportedResolution,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::DatabaseError(err) => format!("Database error: {}", err),
                Error::DeserializeError(err) => format!("Error deserializing json: {}", err),
                Error::ExecError(err) => format!("Error executing query: {}", err),
                Error::SQLError(err) => format!(
                    "Error '{}' with '{}' at position '{}'",
                    err.error, err.query, err.position
                ),
                Error::NoDataFound => format!("No data found"),
                Error::UnsupportedResolution => format!("Unsupported resolution type"),
            }
        )
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::ExecError(err)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Error {
        Error::DeserializeError(err)
    }
}

#[derive(Clone)]
pub struct Database {
    client: Client,
    pub url: String,
    pub config: DatabaseSettings,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct QueryResult<T> {
    // pub count: u64,
    pub data: Vec<T>,
    // page_number
    // page_size
    // total_count
    // records
}

impl Database {
    /// Creates a new connection to questdb
    ///
    /// # Example
    /// use database::Database;
    /// let connection = Database::new("http://127.0.0.1:8000");
    ///
    pub fn new(url: &str) -> Self {
        let config = get_config().expect("Failed to read config");
        Database {
            client: Client::new(),
            url: String::from(url),
            config: config.database,
        }
    }

    pub async fn exec<T: DeserializeOwned>(
        &self,
        query: &str,
        limit_lower: &Option<usize>,
        limit_upper: &Option<usize>,
    ) -> Result<QueryResult<T>, Error> {
        let max_page_size: usize = self.config.max_page_size;
        let encoded_query: String = form_urlencoded::byte_serialize(query.as_bytes()).collect();
        let mut url = format!("{}/exec?count=true&query={}", self.url, encoded_query);

        // check all optional arguments and append to the url
        url += format!(
            "&limit={},{}",
            limit_lower.unwrap_or_default(),
            limit_upper.unwrap_or(max_page_size)
        )
        .as_str();

        let res = self
            .client
            .get(url.as_str())
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        let deserialized = match res.get("dataset") {
            Some(d) => d,
            None => {
                // query failed
                let err: SQLError = serde_json::from_value(res)?;
                return Err(Error::SQLError(err));
            }
        }
        .to_owned();

        Ok(serde_json::from_value(json!({
            "count": res.get("count"),
            "data": deserialized
        }))?)
    }
}
