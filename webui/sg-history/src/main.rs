#[macro_use]
extern crate rouille;

use serde_json;
#[macro_use]
extern crate serde_derive;
use mvdb::Mvdb;
use std::clone::Clone;
use std::collections::BTreeMap;
use std::env;
use std::io;
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Summary {
    mean: f64,
    median: f64,
    min: f64,
    max: f64,
    std_dev: f64,
}

type TestImplementation = Vec<(String, Summary)>;

type Test = (String, TestImplementation);

type Results = Vec<Test>;

type RevisionId = String;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
struct RevisionMeta {
    ts: SystemTime,
    author: String,
    message: String,
    gitref: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Revision {
    meta: RevisionMeta,
    results: Results,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct History {
    history: BTreeMap<RevisionId, Revision>,
}

impl History {
    fn insert(&mut self, revision_id: RevisionId, revision: Revision) {
        self.history.insert(revision_id, revision);
    }
}

#[derive(Clone)]
struct PersistentHistory {
    db: Mvdb<History>,
}

impl PersistentHistory {
    fn new(path: &Path) -> Result<PersistentHistory, io::Error> {
        let db = Mvdb::from_file_or_default_pretty(path).map_err(|_| io::Error::last_os_error())?;
        Ok(PersistentHistory { db })
    }

    fn insert(&mut self, revision_id: RevisionId, revision: Revision) {
        self.db
            .access_mut(|db| db.insert(revision_id, revision))
            .unwrap();
    }

    fn get(&self) -> Result<History, io::Error> {
        self.db
            .access(Clone::clone)
            .map_err(|_| io::Error::last_os_error())
    }
}

fn server_start(listen_addr: &str) -> Result<(), io::Error> {
    let history = PersistentHistory::new(Path::new("history.json"))?;

    rouille::start_server(listen_addr, move |request| {
        router!(request,
                (GET) (/history) => {
                    println!("GET /history");
                    let history = history.get().unwrap();
                    let mut response = rouille::Response::json(&history);
                    response
                        .headers
                        .push(("Access-Control-Allow-Origin".into(), "*".into()));
                    response
                },

                (POST) (/submit) => {
                    let data = try_or_400!(post_input!(request, {
                                    revision_id: String,
                                    results: String,
                                    author: String,
                                    message: String,
                                    gitref: String,
                                    ts: u64,
                                }));

                    let results: Results = serde_json::from_str(&data.results)
                        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, ""))
                        .unwrap();
                    println!("{:#?}", results);
                    let ts = UNIX_EPOCH + Duration::from_secs(data.ts);
                    let revision = Revision {
                        meta: RevisionMeta {
                                ts,
                                author: data.author,
                                message: data.message,
                                gitref: data.gitref,
                            },
                        results,
                    };
                    history.clone().insert(data.revision_id, revision);
                    rouille::Response::html("Success!")
                },

                _ => rouille::Response::empty_404()
        )
    });
}

fn main() {
    let listen_addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:8001".to_string());
    server_start(&listen_addr).unwrap();
}
