#!/usr/bin/env python3
"""Generate protobuf binary input file for benchmarking."""

# We'll generate the protobuf data by creating a JSON structure first,
# then manually encode it to protobuf format (or use the Rust code to generate it)
# For simplicity, we'll create a JSON file and convert it using the Rust benchmark
import json
import random
from datetime import datetime, timedelta


def generate_user(user_id):
    """Generate a single user with realistic data."""
    return {
        "id": user_id,
        "username": f"user_{user_id}",
        "email": f"user{user_id}@example.com",
        "full_name": f"User {user_id} Name",
        "is_active": random.choice([True, False]),
        "created_at": (
            datetime.now() - timedelta(days=random.randint(1, 1000))
        ).isoformat(),
        "updated_at": (
            datetime.now() - timedelta(days=random.randint(0, 100))
        ).isoformat(),
        "profile": {
            "bio": f"This is a bio for user {user_id}. " * random.randint(1, 5),
            "avatar_url": f"https://example.com/avatars/{user_id}.jpg",
            "location": random.choice(
                ["New York", "London", "Tokyo", "Berlin", "Paris", "Sydney"]
            ),
            "website": f"https://user{user_id}.example.com"
            if random.random() > 0.3
            else None,
            "social_links": [
                f"https://twitter.com/user{user_id}",
                f"https://github.com/user{user_id}",
                f"https://linkedin.com/in/user{user_id}",
            ],
        },
        "settings": {
            "theme": random.choice(["light", "dark", "auto"]),
            "language": random.choice(["en", "es", "fr", "de", "ja", "zh"]),
            "notifications_enabled": random.choice([True, False]),
            "privacy_level": random.choice(["public", "friends", "private"]),
            "preferences": {
                f"pref_{i}": f"value_{i}" for i in range(random.randint(3, 8))
            },
        },
        "posts": [
            {
                "id": user_id * 1000 + post_id,
                "title": f"Post {post_id} by user {user_id}",
                "content": f"This is the content of post {post_id}. "
                * random.randint(10, 50),
                "tags": [
                    f"tag{random.randint(1, 20)}" for _ in range(random.randint(1, 5))
                ],
                "likes": random.randint(0, 1000),
                "comments_count": random.randint(0, 100),
                "published_at": (
                    datetime.now() - timedelta(days=random.randint(0, 365))
                ).isoformat(),
            }
            for post_id in range(random.randint(5, 15))
        ],
    }


def main():
    # Generate 100 users for a reasonably sized benchmark
    users = [generate_user(i) for i in range(1, 101)]

    # Save as JSON temporarily - we'll convert to protobuf using a helper script
    with open("temp_data.json", "w") as f:
        json.dump({"users": users}, f, indent=2)

    print(f"Generated JSON with {len(users)} users")
    print("Run the Rust converter to create default.input")


if __name__ == "__main__":
    main()
