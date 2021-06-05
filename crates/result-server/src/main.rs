#[macro_use]
extern crate rouille;
#[macro_use]
extern crate serde_derive;
mod history;

use history::*;
use mvdb::Mvdb;
use rouille::{start_server, Request, Response};
use serde_json;
use std::clone::Clone;
use std::env;
use std::io;
use std::path::{Path, PathBuf};

/// A file-based database implementation for storing the benchmark results.
#[derive(Clone)]
struct PersistentHistory {
    db: Mvdb<History>,
}

impl PersistentHistory {
    fn new(path: &Path) -> Result<PersistentHistory, io::Error> {
        let db = Mvdb::from_file_or_default_pretty(path).map_err(|_| io::Error::last_os_error())?;
        Ok(PersistentHistory { db })
    }

    fn insert(&mut self, timestamp: Timestamp, run: Run) {
        self.db.access_mut(|db| db.insert(timestamp, run)).unwrap();
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
        println!("Starting server at {}", listen_address);
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
        let body = request
            .data()
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidInput,
                "No JSON body provided",
            ))
            .unwrap();
        let run: Run = serde_json::from_reader(body)
            .map_err(|e| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Unable to parse JSON body: {}", e),
                )
            })
            .unwrap();
        println!("{:#?}", run);

        self.history.clone().insert(run.meta.timestamp, run);
        rouille::Response::html("Success!")
    }
}

fn main() {
    let listen_addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:8001".to_string());
    let history_location = env::var_os("HISTORY_PATH")
        .map(PathBuf::from)
        .unwrap_or(PathBuf::from("history.json"));
    let history_server = HistoryServer::new(&history_location).unwrap();
    history_server.start(&listen_addr);
}

#[cfg(test)]
mod tests {
    use crate::history::BenchResults;
    use crate::HistoryServer;
    use rouille::{Request, Response};
    use std::env::temp_dir;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    static FIXTURE_PATH: &'static str = "../../test/fixtures";

    #[test]
    fn deserialize_from_json() {
        let json =
            std::fs::read_to_string(format!("{}/sightglass-output.json", FIXTURE_PATH)).unwrap();
        let results: BenchResults = serde_json::from_str(&json).unwrap();

        assert_eq!(results.len(), 1);
        let bench_result = results.get("a").unwrap();
        assert_eq!(bench_result.len(), 2);
        assert!(bench_result.contains_key("b"));
        assert!(bench_result.contains_key("c"));
    }

    #[test]
    fn example_post() {
        let server = HistoryServer::new(&temporary_history_file()).unwrap();
        let headers = vec![("content-type".into(), "application/json".into())];
        let json = std::fs::read_to_string(format!("{}/stored-run.json", FIXTURE_PATH)).unwrap();
        let request = Request::fake_http(
            "POST",
            "http://localhost/submit",
            headers,
            json.into_bytes(),
        );

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
