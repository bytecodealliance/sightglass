//! JSON serialization and deserialization benchmark using serde_json.
//!
//! This benchmark tests JSON parsing and serialization performance on a
//! realistic data structure representing API response data.

use serde::{Deserialize, Serialize};
use sightglass_api as bench;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    id: u64,
    username: String,
    email: String,
    full_name: String,
    is_active: bool,
    created_at: String,
    updated_at: String,
    profile: Profile,
    settings: Settings,
    posts: Vec<Post>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Profile {
    bio: String,
    avatar_url: String,
    location: String,
    website: Option<String>,
    social_links: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Settings {
    theme: String,
    language: String,
    notifications_enabled: bool,
    privacy_level: String,
    preferences: std::collections::HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Post {
    id: u64,
    title: String,
    content: String,
    tags: Vec<String>,
    likes: u32,
    comments_count: u32,
    published_at: String,
}

fn main() {
    // Read the JSON input file
    let json_data = std::fs::read_to_string("default.input")
        .expect("unable to read default.input");

    bench::start();

    // Deserialize: Parse JSON into structured data
    let users: Vec<User> = serde_json::from_str(&json_data)
        .expect("failed to parse JSON");

    // Serialize: Convert back to JSON
    let serialized = serde_json::to_string(&users)
        .expect("failed to serialize to JSON");

    bench::end();

    eprintln!("[rust-json] processed {} users", users.len());
    eprintln!("[rust-json] serialized size: {} bytes", serialized.len());
}
