use extendr_api::prelude::*;
use base64::prelude::*;
use base64::engine::general_purpose;
use std::fs;
use itertools::Itertools;

/// Encode to base64
/// 
/// @param input A string, raw vector, or file path.
/// @export
/// @name encode
#[extendr]
fn encode_string(input: String) -> String {
    let mut buf = String::new();
    general_purpose::STANDARD.encode_string(input,  &mut buf);
    buf
}


/// @export
/// @name encode
#[extendr]
fn encode_raw(input: Raw) -> String {
    let mut buf = String::new();
    general_purpose::STANDARD.encode_string(input.as_slice(),  &mut buf);
    buf
}

/// @export
/// @name encode
#[extendr]
fn encode_file(path: String) -> String {
    let mut buf = String::new();
    let fp = fs::read(&path);

    if let Ok(contents) = fp {
        general_purpose::STANDARD.encode_string(&contents, &mut buf);
    } else {
        extendr_api::throw_r_error("Unable to read file.")
    }
    
    buf
}

#[extendr]
fn chunk_encoding(encoded: String, size: i32) -> Strings {
    if size % 4 != 0  {
        extendr_api::throw_r_error("Chunk size must be a multiple of 4.");
    }

    encoded.chars().chunks(size as usize)
        .into_iter()
        .map(|chunk| {
            chunk.collect::<String>()
        })
        .collect::<Strings>()
}

#[extendr]
fn collapse_chunks(chunks: Strings, newline: &str) -> String {
    chunks.into_iter().join(newline)
}

 
/// Decode from base64
/// 
/// @param input A string or raw vector.
/// @export
/// @name decode
#[extendr]
fn decode_string(input: String) -> Vec<u8> {
    let res = general_purpose::STANDARD.decode(input);
    match res {
        Ok(decoded) => decoded,
        Err(_) => extendr_api::throw_r_error(&format!("Input could not be decoded"))
    }
}

/// @export
/// @name decode
#[extendr]
fn decode_raw(input: Raw) -> Vec<u8> {
    let res = general_purpose::STANDARD.decode(input.as_slice());
    match res {
        Ok(decoded) => decoded,
        Err(_) => extendr_api::throw_r_error(&format!("Input could not be decoded"))
    }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod b64;
    // encoding
    fn encode_string;
    fn encode_raw;
    fn encode_file;
    // decoding
    fn decode_string;
    fn decode_raw;
    // helpers
    fn chunk_encoding;
    fn collapse_chunks;
}

