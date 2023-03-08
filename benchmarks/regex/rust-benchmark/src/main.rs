//! Find matches in a large input text.
//!
//! Note that this was adapted from [regex-benchmark]; the key part retained
//! is the patterns [regex-benchmark] used for finding the matches.
//!
//! [regex-benchmark]: https://github.com/mariomka/regex-benchmark

use regex::RegexBuilder;
use sightglass_api as bench;

const EMAIL_PATTERN: &str = r"[\w\.+-]+@[\w\.-]+\.[\w\.-]+";
const URI_PATTERN: &str = r"[\w]+://[^/\s?#]+[^\s?#]+(?:\?[^\s#]*)?(?:#[^\s]*)?";
const IP_PATTERN: &str =
    r"(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9])\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9])";

fn main() {
    let path = "default.input";
    eprintln!("[regex] matching {}", path);
    let data = std::fs::read_to_string(path).expect("unable to find `*.input` text file");

    bench::start();
    let emails = count_matches(&data, EMAIL_PATTERN);
    let uris = count_matches(&data, URI_PATTERN);
    let ips = count_matches(&data, IP_PATTERN);
    bench::end();

    eprintln!("[regex] found {} emails", emails);
    eprintln!("[regex] found {} URIs", uris);
    eprintln!("[regex] found {} IPs", ips);
}

fn count_matches(data: &str, pattern: &str) -> usize {
    let regex = RegexBuilder::new(pattern).build().unwrap();
    regex.find_iter(data).count()
}
