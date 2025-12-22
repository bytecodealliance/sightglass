//! Helper to convert JSON data to protobuf binary format
//! This is used during benchmark setup, not during the actual benchmark run.

use prost::Message;
use serde::Deserialize;
use std::collections::HashMap;

#[allow(non_snake_case)]
mod proto {
    include!(concat!(env!("OUT_DIR"), "/benchmark.rs"));
}

use proto::*;

#[derive(Deserialize)]
struct JsonUserList {
    users: Vec<JsonUser>,
}

#[derive(Deserialize)]
struct JsonUser {
    id: u64,
    username: String,
    email: String,
    full_name: String,
    is_active: bool,
    created_at: String,
    updated_at: String,
    profile: JsonProfile,
    settings: JsonSettings,
    posts: Vec<JsonPost>,
}

#[derive(Deserialize)]
struct JsonProfile {
    bio: String,
    avatar_url: String,
    location: String,
    website: Option<String>,
    social_links: Vec<String>,
}

#[derive(Deserialize)]
struct JsonSettings {
    theme: String,
    language: String,
    notifications_enabled: bool,
    privacy_level: String,
    preferences: HashMap<String, String>,
}

#[derive(Deserialize)]
struct JsonPost {
    id: u64,
    title: String,
    content: String,
    tags: Vec<String>,
    likes: u32,
    comments_count: u32,
    published_at: String,
}

fn convert_json_to_proto(json_user_list: JsonUserList) -> UserList {
    let users = json_user_list
        .users
        .into_iter()
        .map(|ju| User {
            id: ju.id,
            username: ju.username,
            email: ju.email,
            full_name: ju.full_name,
            is_active: ju.is_active,
            created_at: ju.created_at,
            updated_at: ju.updated_at,
            profile: Some(Profile {
                bio: ju.profile.bio,
                avatar_url: ju.profile.avatar_url,
                location: ju.profile.location,
                website: ju.profile.website,
                social_links: ju.profile.social_links,
            }),
            settings: Some(Settings {
                theme: ju.settings.theme,
                language: ju.settings.language,
                notifications_enabled: ju.settings.notifications_enabled,
                privacy_level: ju.settings.privacy_level,
                preferences: ju.settings.preferences,
            }),
            posts: ju
                .posts
                .into_iter()
                .map(|jp| Post {
                    id: jp.id,
                    title: jp.title,
                    content: jp.content,
                    tags: jp.tags,
                    likes: jp.likes,
                    comments_count: jp.comments_count,
                    published_at: jp.published_at,
                })
                .collect(),
        })
        .collect();

    UserList { users }
}

fn main() {
    // Read JSON file
    let json_data =
        std::fs::read_to_string("../temp_data.json").expect("Failed to read temp_data.json");

    let json_user_list: JsonUserList =
        serde_json::from_str(&json_data).expect("Failed to parse JSON");

    // Convert to protobuf
    let proto_user_list = convert_json_to_proto(json_user_list);

    // Serialize to binary
    let mut buf = Vec::new();
    proto_user_list
        .encode(&mut buf)
        .expect("Failed to encode protobuf");

    // Write to file
    std::fs::write("../default.input", &buf).expect("Failed to write default.input");

    println!(
        "Converted {} users to protobuf binary format",
        proto_user_list.users.len()
    );
    println!("Output size: {} bytes", buf.len());
}
