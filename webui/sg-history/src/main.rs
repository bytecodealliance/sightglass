#[macro_use]
extern crate rouille;

use serde_json;
#[macro_use]
extern crate serde_derive;
use mvdb::Mvdb;
use rouille::{start_server, Request, Response};
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

/// A wrapper around a rouille::Server for easier test access
struct HistoryServer {
    history: PersistentHistory,
}

impl HistoryServer {
    /// Create a new history server from a backing file
    fn new(history_location: &Path) -> Result<Self, io::Error> {
        let history = PersistentHistory::new(history_location)?;
        Ok(Self { history })
    }

    /// Start running the server by binding to the listen_address; only exits on failure
    fn start(self, listen_address: &str) -> ! {
        start_server(listen_address, move |request| {
            router!(request,
                (GET) (/history) => { self.retrieve_history(request) },
                (POST) (/submit) => { self.submit_results(request) },
                _ => rouille::Response::empty_404()
            )
        })
    }

    /// Handle a history request
    fn retrieve_history(&self, _: &Request) -> Response {
        println!("GET /history");
        let history = self.history.get().unwrap();
        let mut response = rouille::Response::json(&history);
        response
            .headers
            .push(("Access-Control-Allow-Origin".into(), "*".into()));
        response
    }

    /// Handle submitting a new set of results
    fn submit_results(&self, request: &Request) -> Response {
        println!("POST /submit");
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
        self.history.clone().insert(data.revision_id, revision);
        rouille::Response::html("Success!")
    }
}

fn main() {
    let listen_addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:8001".to_string());
    let history_location = Path::new("history.json");
    let history_server = HistoryServer::new(history_location).unwrap();
    history_server.start(&listen_addr);
}

#[cfg(test)]
mod tests {
    use crate::HistoryServer;
    use rouille::{Request, Response};
    use std::env::temp_dir;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn example_post() {
        let server = HistoryServer::new(&temporary_history_file()).unwrap();
        let headers = vec![(
            "content-type".into(),
            "application/x-www-form-urlencoded".into(),
        )];
        let data = "revision_id=6e97e343&author=test&message=Some+commit+message&gitref=master&ts=1234567890&results=[]".to_owned().into_bytes();
        let request = Request::fake_http("POST", "http://localhost/submit", headers, data);

        let response = server.submit_results(&request);

        assert_eq!(response.status_code, 200);
        assert_eq!(read_contents(response), "Success!");
    }

    #[test]
    fn example_get() {
        let server = HistoryServer::new(&temporary_history_file()).unwrap();
        let request = Request::fake_http("GET", "http://localhost/history", vec![], vec![]);

        let response = server.retrieve_history(&request);

        assert_eq!(response.status_code, 200);
        assert_eq!(read_contents(response), "{\"history\":{}}");
    }

    fn read_contents(response: Response) -> String {
        let mut contents = String::new();
        response
            .data
            .into_reader_and_size()
            .0
            .read_to_string(&mut contents)
            .unwrap();
        contents
    }

    fn temporary_history_file() -> PathBuf {
        let mut temporary_history = temp_dir();
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        temporary_history.push(format!("history-{}.json", ts));
        temporary_history
    }
}
