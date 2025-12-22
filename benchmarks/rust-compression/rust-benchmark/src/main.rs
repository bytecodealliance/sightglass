//! Compression and decompression benchmark testing multiple algorithms.
//!
//! Tests two popular compression algorithms:
//! - Deflate (via flate2) - gzip/zlib compression
//! - Brotli - compression by Google, optimized for web content

use flate2::read::{GzDecoder, GzEncoder};
use flate2::Compression;
use sightglass_api as bench;
use std::io::Read;

fn main() {
    // Read the input data
    let data = std::fs::read("default.input").expect("unable to read default.input");

    bench::start();

    // Test 1: Gzip (deflate) compression/decompression
    let gzip_compressed = compress_gzip(&data);
    let gzip_decompressed = decompress_gzip(&gzip_compressed);
    assert_eq!(data.len(), gzip_decompressed.len());

    // Test 2: Brotli compression/decompression
    let brotli_compressed = compress_brotli(&data);
    let brotli_decompressed = decompress_brotli(&brotli_compressed);
    assert_eq!(data.len(), brotli_decompressed.len());

    bench::end();

    eprintln!("[rust-compression] original size: {} bytes", data.len());
    eprintln!(
        "[rust-compression] gzip compressed: {} bytes ({:.1}%)",
        gzip_compressed.len(),
        100.0 * gzip_compressed.len() as f64 / data.len() as f64
    );
    eprintln!(
        "[rust-compression] brotli compressed: {} bytes ({:.1}%)",
        brotli_compressed.len(),
        100.0 * brotli_compressed.len() as f64 / data.len() as f64
    );
}

fn compress_gzip(data: &[u8]) -> Vec<u8> {
    let mut encoder = GzEncoder::new(data, Compression::default());
    let mut compressed = Vec::new();
    encoder.read_to_end(&mut compressed).unwrap();
    compressed
}

fn decompress_gzip(data: &[u8]) -> Vec<u8> {
    let mut decoder = GzDecoder::new(data);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).unwrap();
    decompressed
}

fn compress_brotli(data: &[u8]) -> Vec<u8> {
    let mut compressed = Vec::new();
    let mut reader = std::io::Cursor::new(data);
    brotli::BrotliCompress(&mut reader, &mut compressed, &Default::default()).unwrap();
    compressed
}

fn decompress_brotli(data: &[u8]) -> Vec<u8> {
    let mut decompressed = Vec::new();
    let mut reader = std::io::Cursor::new(data);
    brotli::BrotliDecompress(&mut reader, &mut decompressed).unwrap();
    decompressed
}
