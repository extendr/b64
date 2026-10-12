# Utility Functions

Functions to perform common tasks when working with base64 encoded
strings.

## Usage

``` r
b64_chunk(encoded, width)

b64_wrap(chunks, newline)
```

## Arguments

- encoded:

  a character vector of base64 encoded strings.

- width:

  a numeric scalar defining the width of the chunks. Must be divisible
  by 4.

- chunks:

  a character vector of base64 encoded strings.

- newline:

  a character scalar defining the newline character.

## Value

- `b64_chunk()` returns a list of character vectors.

- `b64_wrap()` returns a scalar character vector.

## Details

`b64_chunk()` splits a character vector of base64 encoded strings into
chunks of a specified width.

`b64_wrap()` wraps a character vector of base64 encoded strings with a
newline character.

## Examples

``` r
encoded <- encode("Hello, world!")
chunked <- b64_chunk(encoded, 4)
chunked
#> [[1]]
#> [1] "SGVs" "bG8s" "IHdv" "cmxk" "IQ=="
#> 

b64_wrap(chunked, "\n")
#> [1] "SGVs\nbG8s\nIHdv\ncmxk\nIQ=="
```
