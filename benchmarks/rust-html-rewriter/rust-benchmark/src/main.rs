//! HTML rewriting benchmark using lol_html.
//!
//! lol_html is a streaming HTML rewriter/parser used by Cloudflare Workers.
//! It can parse, modify, and rewrite HTML on the fly with low memory overhead.
//! This benchmark tests common HTML transformation operations.

use lol_html::{element, html_content::ContentType, rewrite_str, RewriteStrSettings};
use sightglass_api as bench;

fn main() {
    // Read the HTML input
    let html = std::fs::read_to_string("default.input")
        .expect("unable to read default.input");

    bench::start();

    // Perform HTML rewriting with multiple transformations:
    // 1. Add a class to all <div> elements
    // 2. Rewrite all <a> href attributes to add tracking parameters
    // 3. Inject analytics script before </head>
    // 4. Add "noopener noreferrer" to external links
    let output = rewrite_str(
        &html,
        RewriteStrSettings {
            element_content_handlers: vec![
                // Add class to all divs
                element!("div", |el| {
                    let existing_class = el.get_attribute("class").unwrap_or_default();
                    let new_class = if existing_class.is_empty() {
                        "rewritten".to_string()
                    } else {
                        format!("{} rewritten", existing_class)
                    };
                    el.set_attribute("class", &new_class)?;
                    Ok(())
                }),

                // Modify all links
                element!("a[href]", |el| {
                    if let Some(href) = el.get_attribute("href") {
                        // Add tracking parameter
                        let separator = if href.contains('?') { "&" } else { "?" };
                        let new_href = format!("{}{}utm_source=benchmark", href, separator);
                        el.set_attribute("href", &new_href)?;

                        // Add rel attribute for external links
                        if href.starts_with("http") {
                            el.set_attribute("rel", "noopener noreferrer")?;
                        }
                    }
                    Ok(())
                }),

                // Inject script before </head>
                element!("head", |el| {
                    el.append(
                        r#"<script>console.log('Analytics loaded');</script>"#,
                        ContentType::Html,
                    );
                    Ok(())
                }),

                // Add data attribute to all images
                element!("img", |el| {
                    el.set_attribute("data-processed", "true")?;
                    Ok(())
                }),
            ],
            ..RewriteStrSettings::default()
        },
    ).expect("failed to rewrite HTML");

    bench::end();

    eprintln!("[rust-html-rewriter] input size: {} bytes", html.len());
    eprintln!("[rust-html-rewriter] output size: {} bytes", output.len());
    eprintln!("[rust-html-rewriter] size change: {:+} bytes",
              output.len() as i64 - html.len() as i64);
}
