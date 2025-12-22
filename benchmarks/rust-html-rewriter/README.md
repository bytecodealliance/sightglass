# Rust HTML Rewriter Benchmark

This benchmark tests HTML parsing and rewriting performance using `lol_html`, a streaming HTML rewriter/parser developed by Cloudflare and used in Cloudflare Workers.

## What it tests

The benchmark performs multiple HTML transformation operations on a realistic HTML document:

1. **Add CSS classes** - Adds a "rewritten" class to all `<div>` elements
2. **Modify links** - Adds tracking parameters to all `<a href>` attributes
3. **Security attributes** - Adds `rel="noopener noreferrer"` to external links
4. **Script injection** - Injects an analytics script before `</head>`
5. **Image processing** - Adds `data-processed="true"` to all `<img>` elements

These operations represent common use cases for HTML rewriting:
- Adding analytics/tracking
- Injecting scripts and content
- Modifying attributes for security or functionality
- Processing images for lazy loading or CDN rewriting

## Input Data

The `default.input` file (~114 KB) contains a realistic HTML document with:
- Navigation menus with internal and external links
- 50 article sections with images, text, and metadata
- Sidebar widgets
- Footer with contact information and links
- Multiple link types (internal, external, mailto, tel)
- Social sharing buttons

## Implementation

Uses:
- `lol_html` 2.1 - Streaming HTML rewriter
- Selector-based element matching (CSS selectors)
- Low-memory streaming parser (doesn't load entire DOM)

## Performance Notes

lol_html is designed for:
- Streaming HTML transformation (processes as it parses)
- Low memory overhead (no DOM tree)
- Production use in edge computing (Cloudflare Workers)

The benchmark shows realistic workload similar to edge function HTML transformations.
