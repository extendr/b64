# Create an encoding engine

Create an encoding engine

## Usage

``` r
engine(which = "standard")

new_engine(.alphabet = alphabet(), .config = new_config())
```

## Arguments

- which:

  default `"standard"`. The base64 encoding engine to be used. See
  details for more.

- .alphabet:

  an object of class `alphabet` as created with
  [`alphabet()`](alphabet.md) or [`new_alphabet()`](alphabet.md)

- .config:

  an object of class `engine_config` as created with
  [`new_config()`](new_config.md)

## Value

an object of class `engine`.

## Details

### Engines

By default, the "standard" base64 engine is used which is specified in
[RFC 4648](https://datatracker.ietf.org/doc/html/rfc4648#section-4).

Additional pre-configured base64 engines are provided these are:

- `"standard_no_pad"`: uses the standard engine without padding

- `"url_safe"`: uses a url-safe alphabet with padding

- `"url_safe_no_pad"`: uses a url-safe alphabet without padding

See [base64
crate](https://docs.rs/base64/latest/base64/engine/general_purpose/index.html#constants)
for more.

## Examples

``` r
engine()
#> <engine>
new_engine(alphabet("bcrypt"), new_config())
#> <engine>
```
