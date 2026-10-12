# Create a custom encoding engine

Create a custom encoding engine

## Usage

``` r
new_config(
  encode_padding = TRUE,
  decode_padding_trailing_bits = FALSE,
  decode_padding_mode = c("canonical", "indifferent", "none")
)
```

## Arguments

- encode_padding:

  default `TRUE` add 1-2 trailing `=` to pad results

- decode_padding_trailing_bits:

  default `FALSE`. "If invalid trailing bits are present and this is
  true, those bits will be silently ignored." (See details for
  reference).

- decode_padding_mode:

  default `"canonical"`. Other values are `"indifferent"` and `"none"`.
  See details for more.

## Value

an object of class `engine_config`

## Details

See [base64
crate](https://docs.rs/base64/latest/base64/engine/general_purpose/struct.GeneralPurposeConfig.html#method.with_encode_padding)
for more details.

### Decode Padding Modes

There are three modes that can be used for `decode_padding_mode`
argument.

- `"canonical"`: padding must consist of 0, 1, or 2 `=` characters

- `"none"`: there must be no padding characters present

- `"indifferent"`: canonical padding is used, but omitted padding
  characters are also permitted

## Examples

``` r
# create a new nonsensicle config
new_config(FALSE, TRUE, "none")
#> <engine_config>
```
