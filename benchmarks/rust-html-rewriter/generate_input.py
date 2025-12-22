#!/usr/bin/env python3
"""Generate a realistic HTML file for benchmarking HTML rewriting."""


def generate_html():
    """Generate a realistic HTML document with various elements."""

    html = """<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Benchmark HTML Document</title>
    <link rel="stylesheet" href="/styles/main.css">
    <link rel="stylesheet" href="/styles/theme.css">
</head>
<body>
    <header>
        <nav class="navbar">
            <div class="container">
                <a href="/" class="logo">Benchmark Site</a>
                <ul class="nav-menu">
                    <li><a href="/home">Home</a></li>
                    <li><a href="/about">About</a></li>
                    <li><a href="/products">Products</a></li>
                    <li><a href="https://external.com">External Link</a></li>
                    <li><a href="/contact">Contact</a></li>
                </ul>
            </div>
        </nav>
    </header>

    <main class="content">
"""

    # Generate multiple article sections
    for i in range(50):
        html += f"""
        <article class="post" id="post-{i}">
            <div class="post-header">
                <h2><a href="/posts/{i}">Article Title {i}</a></h2>
                <div class="meta">
                    <span class="author">Author {i % 10}</span>
                    <span class="date">2024-12-{(i % 28) + 1:02d}</span>
                </div>
            </div>

            <div class="post-content">
                <img src="/images/article-{i}.jpg" alt="Article {i} image" class="featured-image">
                <p>This is the introduction paragraph for article {i}. It contains some introductory text that describes what the article is about.</p>

                <div class="post-body">
                    <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.</p>
                    <p>Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.</p>

                    <div class="callout">
                        <p>This is an important callout box with additional information.</p>
                    </div>

                    <ul class="features">
                        <li><a href="/feature/{i}-1">Feature 1</a></li>
                        <li><a href="/feature/{i}-2">Feature 2</a></li>
                        <li><a href="/feature/{i}-3">Feature 3</a></li>
                    </ul>
                </div>

                <div class="post-footer">
                    <div class="tags">
                        <a href="/tag/technology" class="tag">Technology</a>
                        <a href="/tag/programming" class="tag">Programming</a>
                        <a href="/tag/web" class="tag">Web</a>
                    </div>
                    <div class="social-share">
                        <a href="https://twitter.com/share?url=post-{i}" class="share-btn twitter">Share</a>
                        <a href="https://facebook.com/sharer?url=post-{i}" class="share-btn facebook">Share</a>
                        <a href="https://linkedin.com/share?url=post-{i}" class="share-btn linkedin">Share</a>
                    </div>
                </div>
            </div>
        </article>
"""

    # Add footer
    html += """
    </main>

    <aside class="sidebar">
        <div class="widget">
            <h3>Popular Posts</h3>
            <ul>
                <li><a href="/popular/1">Popular Post 1</a></li>
                <li><a href="/popular/2">Popular Post 2</a></li>
                <li><a href="/popular/3">Popular Post 3</a></li>
                <li><a href="/popular/4">Popular Post 4</a></li>
                <li><a href="/popular/5">Popular Post 5</a></li>
            </ul>
        </div>

        <div class="widget">
            <h3>Categories</h3>
            <ul>
                <li><a href="/category/tech">Technology</a></li>
                <li><a href="/category/science">Science</a></li>
                <li><a href="/category/business">Business</a></li>
                <li><a href="/category/lifestyle">Lifestyle</a></li>
            </ul>
        </div>

        <div class="widget ad-widget">
            <img src="/ads/sidebar-ad.jpg" alt="Advertisement">
            <a href="https://advertiser.com/product">Learn More</a>
        </div>
    </aside>

    <footer>
        <div class="container">
            <div class="footer-content">
                <div class="footer-section">
                    <h4>About Us</h4>
                    <p>We are a benchmark site for testing HTML rewriting performance.</p>
                    <a href="/about">Read more</a>
                </div>

                <div class="footer-section">
                    <h4>Quick Links</h4>
                    <ul>
                        <li><a href="/privacy">Privacy Policy</a></li>
                        <li><a href="/terms">Terms of Service</a></li>
                        <li><a href="/sitemap">Sitemap</a></li>
                        <li><a href="https://github.com/example">GitHub</a></li>
                    </ul>
                </div>

                <div class="footer-section">
                    <h4>Contact</h4>
                    <p>Email: <a href="mailto:info@example.com">info@example.com</a></p>
                    <p>Phone: <a href="tel:+1234567890">+1 (234) 567-890</a></p>
                </div>
            </div>

            <div class="footer-bottom">
                <p>&copy; 2024 Benchmark Site. All rights reserved.</p>
            </div>
        </div>
    </footer>

    <script src="/js/main.js"></script>
    <script src="/js/analytics.js"></script>
</body>
</html>
"""

    return html


def main():
    html = generate_html()

    with open("default.input", "w") as f:
        f.write(html)

    print(f"Generated HTML file: {len(html)} bytes ({len(html) / 1024:.1f} KB)")


if __name__ == "__main__":
    main()
