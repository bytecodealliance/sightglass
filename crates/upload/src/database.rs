use anyhow::{anyhow, bail, Result};
use reqwest::blocking::Client;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// A simple HTTP wrapper for communicating with an ElasticSearch database.
pub struct Database {
    url: String,
    dry_run: bool,
}

impl Database {
    pub fn new(url: String, dry_run: bool) -> Self {
        Self {
            url: url.trim_end_matches("/").to_string(),
            dry_run,
        }
    }

    /// Use the ElasticSearch [Get
    /// API](https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-get.html)
    /// to verify if a document exists in `index` with the given `id`.
    #[allow(dead_code)]
    pub fn exists(&self, index: &str, id: &str) -> Result<bool> {
        let url = format!("{}/{}/_doc/{}", self.url, index, id);
        if self.dry_run {
            Ok(true)
        } else {
            let client = Client::new();
            let response = client.head(url).send()?;
            Ok(response.status().is_success())
        }
    }

    /// Retrieve an object from the database, if it exists.
    pub fn get<T>(&self, index: &str, id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}/{}/_doc/{}", self.url, index, id);
        if self.dry_run {
            bail!("could not retrieve object with ID: {}", id);
        } else {
            let client = Client::new();
            let response = client.get(url).send()?;
            let bytes = response.bytes()?;
            Ok(serde_json::from_slice(bytes.as_ref())?)
        }
    }

    /// Create an object in the database, reusing an existing object if
    /// possible. This function handles several cases:
    ///  1. if the ID is not used in the database, create the object
    ///  2. if the ID is used and the existing object matches `object`, simply
    ///     return the ID without creating a new database entry
    ///  3. if the ID is used and the existing object does not match `object`,
    ///     append a `!` to the ID and retry (up to 5 times).
    pub fn create_if_not_exists<T>(&self, index: &str, object: &T, id: &str) -> Result<String>
    where
        T: DeserializeOwned + Serialize + PartialEq,
    {
        let mut id = id.to_string();
        for _ in 0..NUM_RETRIES {
            match self.get(index, &id) {
                Ok(stored_object) => {
                    if object == &stored_object {
                        // Case #2: the same object already exists in the
                        // database; simply return the ID.
                        return Ok(id);
                    } else {
                        // Case #3: a different object exists with the same ID;
                        // change the ID and retry.
                        id.push('!');
                    }
                }
                Err(_) => {
                    // Case #1: no object exists with the ID; create it.
                    return self.create(index, object, Some(&id));
                }
            }
        }
        Err(anyhow!("failed to find a usable ID"))
    }

    /// Use the ElasticSearch [Index
    /// API](https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-index_.html)
    /// to add a document to `index`, optionally with the given `id`.
    pub fn create<T>(&self, index: &str, object: &T, id: Option<&str>) -> Result<String>
    where
        T: Serialize,
    {
        log::debug!("Creating record in '{index}' with ID {id:?}");
        let url = if let Some(id) = id {
            format!("{}/{}/_doc/{}", self.url, index, id)
        } else {
            format!("{}/{}/_doc", self.url, index)
        };

        let body = serde_json::to_string(object)?;
        log::trace!("Record body: {}", &body);

        if self.dry_run {
            Ok(if let Some(id) = id {
                id.to_string()
            } else {
                "dry-run-id".to_string()
            })
        } else {
            let client = Client::new();
            let response = client
                .post(url)
                .header("Content-Type", "application/json")
                .body(body)
                .send()?;

            let success = response.status().is_success();
            let content: HashMap<String, Value> = serde_json::from_slice(&response.bytes()?)?;
            log::debug!("ElasticSearch response: {content:?}");

            if success {
                let id = content.get("_id").unwrap().as_str().unwrap().to_string();
                Ok(id)
            } else {
                bail!("Failed to create record: {:?}", content)
            }
        }
    }

    /// Use the ElasticSearch [Bulk
    /// API](https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-bulk.html)
    /// to add many objects to `index`. This should be significantly faster than
    /// `create`.
    pub fn create_batched<T>(&self, index: &str, objects: &[T]) -> Result<()>
    where
        T: Serialize,
    {
        log::debug!("Batching up {} records to index '{}'", objects.len(), index);
        let url = format!("{}/{}/_bulk", self.url, index);

        // Capture all of the records as index requests.
        let mut body = vec![];
        for o in objects {
            body.push("{\"index\": {}}\n".to_string());
            body.push(format!("{}\n", serde_json::to_string(o)?));
        }
        let body = body.concat();

        // Upload the entire batch request to ElasticSearch.
        if self.dry_run {
            log::trace!("Record body: {}", &body);
            Ok(())
        } else {
            let client = Client::new();
            let response = client
                .post(url)
                .header("Content-Type", "application/x-ndjson")
                .body(body)
                .send()?;

            let success = response.status().is_success();
            let content: HashMap<String, Value> = serde_json::from_slice(&response.bytes()?)?;
            log::debug!("ElasticSearch response: {content:?}");

            if success {
                Ok(())
            } else {
                bail!("Failed to batch-create records: {:?}", content)
            }
        }
    }
}

const NUM_RETRIES: i32 = 5;
