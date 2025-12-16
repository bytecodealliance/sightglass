#!/usr/bin/env python3
"""Generate input data for compression benchmark.

Creates a file with mixed content that compresses reasonably well:
- Repeated text patterns
- JSON-like structured data
- Some random data
"""

import random
import string


def generate_text_block(size):
    """Generate semi-random text that compresses well."""
    words = [
        "the",
        "quick",
        "brown",
        "fox",
        "jumps",
        "over",
        "lazy",
        "dog",
        "hello",
        "world",
        "test",
        "data",
        "benchmark",
        "compression",
        "algorithm",
        "performance",
        "measure",
        "speed",
        "quality",
    ]

    text = []
    while len(" ".join(text)) < size:
        text.append(random.choice(words))

    return " ".join(text)[:size]


def generate_json_like_data():
    """Generate JSON-like structured data."""
    data = []
    for i in range(1000):
        record = f'{{"id": {i}, "name": "user_{i}", "email": "user{i}@example.com", '
        record += f'"status": "active", "score": {random.randint(0, 100)}, '
        record += f'"tags": ["tag1", "tag2", "tag3"]}}\n'
        data.append(record)
    return "".join(data)


def generate_repeated_pattern():
    """Generate data with lots of repetition (compresses very well)."""
    pattern = "ABCDEFGHIJ" * 100
    return (pattern + "\n") * 1000


def main():
    with open("default.input", "wb") as f:
        # Mix of different data types for realistic compression testing

        # 1. Repeated patterns (compress very well)
        f.write(generate_repeated_pattern().encode("utf-8"))

        # 2. Structured data (compresses well)
        f.write(generate_json_like_data().encode("utf-8"))

        # 3. Natural language-like text (compresses moderately)
        f.write(generate_text_block(500000).encode("utf-8"))

        # 4. Some random data (doesn't compress well)
        random_bytes = bytes(random.randint(0, 255) for _ in range(100000))
        f.write(random_bytes)

    import os

    size = os.path.getsize("default.input")
    print(f"Generated input file: {size} bytes ({size / 1024 / 1024:.2f} MB)")


if __name__ == "__main__":
    main()
