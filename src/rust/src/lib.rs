use extendr_api::prelude::*;
use base64::{prelude::*, alphabet, engine};
use base64::engine::{general_purpose, DecodePaddingMode, GeneralPurposeConfig};
use std::fs;
use itertools::Itertools;

/// Encode to base64
/// 
/// @param input A string, raw vector, or file path.
/// @export
/// @name encode
#[extendr]
fn encode_string(input: String) -> String {
    general_purpose::STANDARD.encode(input)
}


/// @export
/// @name encode
#[extendr]
fn encode_raw(input: Raw) -> String {
    general_purpose::STANDARD.encode(input.as_slice())
}

/// @export
/// @name encode
#[extendr]
fn encode_file(path: String) -> String {
    let fp = fs::read(&path);
    if let Ok(contents) = fp {
        general_purpose::STANDARD.encode(&contents)
    } else {
        extendr_api::throw_r_error("Unable to read file.")
    }
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
fn line_wrap(chunks: Strings, newline: &str) -> String {
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

// use a built-in alphabet
#[extendr]
fn alphabet_(which: &str) -> ExternalPtr<alphabet::Alphabet> {
    match which {
        "bcrypt" => ExternalPtr::new(alphabet::BCRYPT),
        "bin_hex" => ExternalPtr::new(alphabet::BIN_HEX),
        "crypt" => ExternalPtr::new(alphabet::CRYPT),
        "imap_mutf7" => ExternalPtr::new(alphabet::IMAP_MUTF7),
        "standard" => ExternalPtr::new(alphabet::STANDARD),
        "url_safe" => ExternalPtr::new(alphabet::URL_SAFE),
        _ => extendr_api::throw_r_error(&format!("Unknown alphabet: {}", which))
    }
}

// Create new alphabet
#[extendr]
fn new_alphabet(chars: &str) ->  ExternalPtr<alphabet::Alphabet> {
    let res = alphabet::Alphabet::new(chars).unwrap();
    ExternalPtr::new(res)
}

#[extendr]
fn print_alphabet(alphabet: Robj) {
    let alph: ExternalPtr<alphabet::Alphabet> = alphabet.try_into().unwrap();
    println!("{:?}", &alph);
}

// default configs 
// padding = true, 
// decode_allow_trailing_bits = false, 
// and decode_padding_mode = DecodePaddingMode::RequireCanonicalPadding
#[extendr]
fn new_config_(
    encode_padding: bool,
    decode_padding_trailing_bits: bool,
    decode_padding_mode: &str,
) -> ExternalPtr<GeneralPurposeConfig> {
    let pad_mode = match decode_padding_mode {
        "indifferent" => DecodePaddingMode::Indifferent,
        "canonical" => DecodePaddingMode::RequireCanonical,
        "none" => DecodePaddingMode::RequireNone,
        _ => extendr_api::throw_r_error(&format!("Unknown padding mode: {}", decode_padding_mode))
    };

    let config = GeneralPurposeConfig::new()
        .with_encode_padding(encode_padding)
        .with_decode_allow_trailing_bits(decode_padding_trailing_bits)
        .with_decode_padding_mode(pad_mode);

    ExternalPtr::new(config)
}


/// Create base64 Engines
/// 
/// `engine()` creates a new `GeneralPurpose` engine. 
/// 
/// @param
/// 
/// @export
#[extendr]
fn engine_(which: &str) -> ExternalPtr<engine::GeneralPurpose> {
    match which {
        "standard" => ExternalPtr::new(general_purpose::STANDARD),
        "standard_no_pad" => ExternalPtr::new(general_purpose::STANDARD_NO_PAD),
        "url_safe" => ExternalPtr::new(general_purpose::URL_SAFE),
        "url_safe_no_pad" => ExternalPtr::new(general_purpose::URL_SAFE_NO_PAD),
        _ => extendr_api::throw_r_error(&format!("Unknown engine: {}", which))
    }
}

#[extendr]
fn new_engine_(alphabet: Robj, config: Robj) -> ExternalPtr<engine::GeneralPurpose> {
    let alph: ExternalPtr<alphabet::Alphabet> = alphabet.try_into().unwrap();
    let conf: ExternalPtr<GeneralPurposeConfig> = config.try_into().unwrap();
    let inner = conf.addr();
    let engine = general_purpose::GeneralPurpose::new(&alph, *conf);
    ExternalPtr::new(engine)
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
    fn line_wrap;

    // alphabets
    fn alphabet_;
    fn new_alphabet;
    fn print_alphabet;
}

