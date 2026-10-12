# Encode and decode using base64

Encode and decode using base64

## Usage

``` r
encode(what, eng = engine())

decode(what, eng = engine())

decode_as_string(what, newline = "\n", eng = engine())

encode_file(path, eng = engine())

decode_file(path, eng = engine())
```

## Arguments

- what:

  a character, raw, or blob vector

- eng:

  a base64 engine. See [`engine()`](engine.md) for details.

- newline:

  a character sequence to split in the input base64 encoded string on
  before decoding.

- path:

  a path to a base64 encoded file.

## Value

Both `encode()` and `decode()` are vectorized. They will return a
character and blob vector the same length as `what`, respectively.

`decode_as_string()` returns a character scalar.

## Details

### Encoding

- `encode()` takes a character vector, list of raw vectors (or blob
  class), or a raw vector and encodes them into base64 strings.

- `encode_file()` takes a path to a file and encodes it as a base64
  string.

### Decoding

- `decode()` will decode either a base64 encoded character scalar, a raw
  vector, or a list of raw vectors (see blob package).

- `decode_file()` will decode a base64 encoded file into a raw vector.

- `decode_as_string()` is designed to decode a base64 encoded string to
  a utf-8 string. By default, it will decode a chunked base64 encoded
  strings using `\n` as the separator. Use the `newline` argument to
  determine how to split the input string prior to decoding.

## Examples

``` r
# encode hello world
encoded <- encode("Hello world")
encoded
#> [1] "SGVsbG8gd29ybGQ="

# decode to a blob
decoded <- decode(encoded)
decoded
#> <blob[1]>
#> [1] blob[11 B]

# convert back to a character
rawToChar(decoded[[1]])
#> [1] "Hello world"
```
