use base64::{
    alphabet,
    engine::{general_purpose, DecodePaddingMode, GeneralPurpose, GeneralPurposeConfig},
    prelude::*,
    read::DecoderReader,
    write::EncoderStringWriter,
};
use extendr_api::prelude::*;
use itertools::{Either, Itertools};
use std::io::Read;

#[extendr]
fn encode_(what: Either<String, Raw>, engine: Robj) -> String {
    let eng: ExternalPtr<GeneralPurpose> = engine.try_into().unwrap();
    match what {
        Either::Left(s) => eng.encode(s),
        Either::Right(r) => eng.encode(r.as_slice()),
    }
}

#[extendr]
fn encode_vectorized_(what: Either<Strings, List>, engine: Robj) -> Strings {
    let eng: ExternalPtr<GeneralPurpose> = engine.try_into().unwrap();
    match what {
        Either::Left(s) => s
            .into_iter()
            .map(|s| {
                if s.is_na() {
                    Rstr::na()
                } else {
                    let to_encode = s.as_bytes();
                    Rstr::from(eng.encode(to_encode))
                }
            })
            .collect::<Strings>(),
        Either::Right(r) => r
            .into_iter()
            .map(|(_, b)| {
                if b.is_null() {
                    Rstr::na()
                } else {
                    let raw: Raw = b.try_into().unwrap();
                    Rstr::from(eng.encode(raw.as_slice()))
                }
            })
            .collect::<Strings>(),
    }
}

#[extendr]
fn encode_file_(path: &str, engine: Robj) -> String {
    let eng: ExternalPtr<GeneralPurpose> = engine.try_into().unwrap();
    let eng = eng.addr();
    let file = std::fs::File::open(path).unwrap();
    let mut reader = std::io::BufReader::new(file);
    let mut encoder = EncoderStringWriter::new(eng);
    std::io::copy(&mut reader, &mut encoder).unwrap();
    encoder.into_inner()
}

/// Utility Functions
///
/// Functions to perform common tasks when working with base64 encoded strings.
///
/// @details
///
/// `b64_chunk()` splits a character vector of base64 encoded strings into chunks of a
/// specified width.
///
/// `b64_wrap()` wraps a character vector of base64 encoded strings with a newline character.
///
/// @returns
///
/// - `b64_chunk()` returns a list of character vectors.
/// - `b64_wrap()` returns a scalar character vector.
///
/// @examples
/// encoded <- encode("Hello, world!")
/// chunked <- b64_chunk(encoded, 4)
/// chunked
///
/// b64_wrap(chunked, "\n")
/// @param width a numeric scalar defining the width of the chunks. Must be divisible by 4.
/// @param encoded a character vector of base64 encoded strings.
/// @export
/// @rdname utils
#[extendr]
fn b64_chunk(encoded: Strings, width: Either<i32, f64>) -> List {
    let width = match width {
        Left(l) => l,
        Right(r) => r as i32,
    };

    if width % 4 != 0 {
        extendr_api::throw_r_error("Chunk size must be a multiple of 4.");
    }
    encoded
        .into_iter()
        .map(|s| {
            if s.is_na() {
                Strings::new(0)
            } else {
                s.chars()
                    .chunks(width as usize)
                    .into_iter()
                    .map(|chunk| chunk.collect::<String>())
                    .collect::<Strings>()
            }
        })
        .collect::<List>()
}

/// @param chunks a character vector of base64 encoded strings.
/// @param newline a character scalar defining the newline character.
/// @export
/// @rdname utils
#[extendr]
fn b64_wrap(chunks: Either<List, Strings>, newline: &str) -> Strings {
    match chunks {
        Left(l) => l
            .into_iter()
            .map(|(_, s)| {
                if s.is_na() {
                    Rstr::na()
                } else {
                    let s = Strings::try_from(s).unwrap();
                    Rstr::from(b64_wrap_(s, newline))
                }
            })
            .collect::<Strings>(),
        Right(r) => b64_wrap_(r, newline).into(),
    }
}

fn b64_wrap_(chunks: Strings, newline: &str) -> String {
    chunks.into_iter().join(newline)
}

#[extendr]
fn decode_(input: Either<String, Raw>, engine: Robj) -> List {
    let eng: ExternalPtr<GeneralPurpose> = engine.try_into().unwrap();
    let res = match input {
        Either::Left(s) => {
            let res = eng.decode(s);
            match res {
                Ok(d) => d,
                Err(e) => throw_r_error(e.to_string().as_str()),
            }
        }
        Either::Right(r) => match eng.decode(r.as_slice()) {
            Ok(d) => d,
            Err(e) => throw_r_error(e.to_string().as_str()),
        },
    };

    list!(Raw::from_bytes(&res))
        .set_class(&["blob", "vctrs_list_of", "vctrs_vctr", "list"])
        .unwrap()
        .clone()
}

#[extendr]
fn decode_vectorized_(what: Either<Strings, List>, engine: Robj) -> List {
    let eng: ExternalPtr<GeneralPurpose> = engine.try_into().unwrap();
    match what {
        Either::Left(s) => s
            .into_iter()
            .map(|s| {
                if s.is_na() {
                    ().into_robj()
                } else {
                    let to_encode = s.as_str();
                    let decoded = eng.decode(to_encode);
                    match decoded {
                        Ok(d) => {
                            let r = Raw::from_bytes(&d);
                            r.into_robj()
                        }
                        Err(_) => ().into_robj(),
                    }
                }
            })
            .collect::<List>()
            .set_class(&["blob", "vctrs_list_of", "vctrs_vctr", "list"])
            .unwrap()
            .clone(),
        Either::Right(r) => r
            .into_iter()
            .map(|(_, b)| {
                let raw = Raw::try_from(b);
                match raw {
                    Ok(r) => {
                        let decoded = eng.decode(r.as_slice());
                        match decoded {
                            Ok(d) => Raw::from_bytes(&d).into_robj(),
                            Err(_) => ().into_robj(),
                        }
                    }
                    Err(_) => ().into_robj(),
                }
            })
            .collect::<List>()
            .set_class(&["blob", "vctrs_list_of", "vctrs_vctr", "list"])
            .unwrap()
            .clone(),
    }
}

#[extendr]
fn decode_file_(path: &str, engine: Robj) -> Vec<u8> {
    let eng: ExternalPtr<GeneralPurpose> = engine.try_into().unwrap();
    let eng = eng.addr();
    let file = std::fs::File::open(path).unwrap();
    let mut reader = std::io::BufReader::new(file);
    let mut decoder = DecoderReader::new(&mut reader, eng);
    let mut result = Vec::new();
    decoder.read_to_end(&mut result).unwrap();
    result
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
        _ => extendr_api::throw_r_error(format!("Unknown alphabet: {}", which)),
    }
}

// Create new alphabet
#[extendr]
fn new_alphabet_(chars: &str) -> ExternalPtr<alphabet::Alphabet> {
    let res = alphabet::Alphabet::new(chars);

    match res {
        Ok(r) => ExternalPtr::new(r),
        Err(e) => extendr_api::throw_r_error(format!("Error creating alphabet: {}", e)),
    }
}

// get alphabet as a string for printing
#[extendr]
fn get_alphabet_(alphabet: Robj) -> String {
    let alph: ExternalPtr<alphabet::Alphabet> = alphabet.try_into().unwrap();
    alph.as_str().to_string()
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
        _ => extendr_api::throw_r_error(format!("Unknown padding mode: {}", decode_padding_mode)),
    };

    let config = GeneralPurposeConfig::new()
        .with_encode_padding(encode_padding)
        .with_decode_allow_trailing_bits(decode_padding_trailing_bits)
        .with_decode_padding_mode(pad_mode);

    ExternalPtr::new(config)
}

#[extendr]
fn print_config_(config: Robj) -> String {
    let conf: ExternalPtr<GeneralPurposeConfig> = config.try_into().unwrap();
    format!("{:#?}", conf)
}

#[extendr]
fn engine_(which: &str) -> ExternalPtr<GeneralPurpose> {
    match which {
        "standard" => ExternalPtr::new(general_purpose::STANDARD),
        "standard_no_pad" => ExternalPtr::new(general_purpose::STANDARD_NO_PAD),
        "url_safe" => ExternalPtr::new(general_purpose::URL_SAFE),
        "url_safe_no_pad" => ExternalPtr::new(general_purpose::URL_SAFE_NO_PAD),
        _ => extendr_api::throw_r_error(format!("Unknown engine: {}", which)),
    }
}

// need to figure out a nice print pattern here
#[extendr]
fn print_engine_(engine: Robj) -> String {
    let eng: ExternalPtr<GeneralPurpose> = engine.try_into().unwrap();
    format!("{:#?}", eng)
}

#[extendr]
fn new_engine_(alphabet: Robj, config: Robj) -> ExternalPtr<GeneralPurpose> {
    let alph: ExternalPtr<alphabet::Alphabet> = alphabet.try_into().unwrap();
    let conf: ExternalPtr<GeneralPurposeConfig> = config.try_into().unwrap();
    let inner = conf.addr();
    let engine = general_purpose::GeneralPurpose::new(&alph, *inner);
    ExternalPtr::new(engine)
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod b64;
    // encoding
    fn encode_;
    fn encode_file_;
    fn encode_vectorized_;

    // decoding
    fn decode_;
    fn decode_file_;
    fn decode_vectorized_;

    // alphabets
    fn alphabet_;
    fn new_alphabet_;
    fn get_alphabet_;

    // engines
    fn new_engine_;
    fn engine_;
    fn print_engine_;

    // config
    fn new_config_;
    fn print_config_;

    // helpers
    fn b64_chunk;
    fn b64_wrap;
}
