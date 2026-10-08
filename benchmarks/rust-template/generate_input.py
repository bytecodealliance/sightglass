"""Generates `rust-template.input.json`, the catalog data the template renders."""

import json
import random
import sys

WORDS = """lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor
incididunt ut labore et dolore magna aliqua enim ad minim veniam quis nostrud
exercitation ullamco laboris nisi aliquip ex ea commodo consequat duis aute irure
in reprehenderit voluptate velit esse cillum fugiat nulla pariatur excepteur sint
occaecat cupidatat non proident sunt culpa qui officia deserunt mollit anim id est
laborum""".split()
BRANDS = ["Acme", "Globex", "Initech", "Umbrella", "Hooli", "Stark", "Wayne",
          "Wonka", "Tyrell", "Cyberdyne", "Soylent", "Vandelay"]
TAGS = ["new", "eco friendly", "bestseller", "limited edition", "refurbished",
        "free shipping", "gift idea", "staff pick", "clearance", "premium"]
CATEGORIES = ["kitchen", "garden tools", "electronics", "outdoor gear", "books",
              "toys and games", "office supplies", "home decor"]


def sentence(rng, lo, hi):
    return " ".join(rng.choice(WORDS) for _ in range(rng.randint(lo, hi))).capitalize() + "."


def main(n_products):
    rng = random.Random(42)
    products = []
    for i in range(n_products):
        name = f"{sentence(rng, 2, 4)[:-1]} {i}"
        products.append({
            "id": 1000 + i,
            "slug": name.lower().replace(" ", "-"),
            "name": name,
            "brand": rng.choice(BRANDS),
            "price": round(rng.uniform(2, 500), 2),
            "discount": rng.choice([0, 0, 0, 5, 10, 15, 25, 40]),
            "rating": round(rng.uniform(1, 5), 1),
            "in_stock": rng.random() < 0.8,
            "description": " ".join(sentence(rng, 6, 16) for _ in range(rng.randint(1, 4))),
            "tags": rng.sample(TAGS, rng.randint(0, 4)),
            "reviews": [{
                "author": f"{rng.choice(WORDS).capitalize()} {rng.choice(WORDS).capitalize()}",
                "rating": rng.randint(1, 5),
                "helpful": rng.randint(0, 200),
                "text": " ".join(sentence(rng, 4, 14) for _ in range(rng.randint(1, 3))),
            } for _ in range(rng.randint(0, 6))],
        })
    data = {
        "site": {
            "name": "Example <Store>",
            "locale": "en",
            "year": 2026,
            "currency": "USD",
            "static_url": "https://static.example.com",
            "tagline": "Everything you need and plenty of things you do not, delivered to your door.",
            "stylesheets": ["reset.css", "layout.css", "catalog.css", "print.css"],
            "footer_links": [
                {"title": t, "url": f"/{t.lower().replace(' ', '-')}", "external": t == "Blog"}
                for t in ["About", "Careers", "Blog", "Privacy Policy", "Terms of Service", "Contact"]
            ],
        },
        "categories": [{
            "slug": c.replace(" ", "-"), "name": c, "product_count": rng.randint(10, 900),
        } for c in CATEGORIES],
        "current_category": "garden-tools",
        "page_title": "Garden Tools",
        "user": {
            "display_name": "Ada & Co",
            "cart": [{"id": rng.randint(1000, 1000 + n_products), "price": round(rng.uniform(2, 500), 2)}
                     for _ in range(3)],
        },
        "products": products,
    }
    json.dump(data, sys.stdout, indent=1, sort_keys=True)
    sys.stdout.write("\n")


if __name__ == "__main__":
    main(int(sys.argv[1]) if len(sys.argv) > 1 else 140)
