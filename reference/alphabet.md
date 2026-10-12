# Standard base64 alphabets

Create an alphabet from a set of standard base64 alphabets, or use your
own.

## Usage

``` r
alphabet(which = "standard")

new_alphabet(chars)
```

## Arguments

- which:

  default `"standard"`. Which base64 alphabet to use. See details for
  other values.

- chars:

  a character scalar contains 64 unique characters.

## Value

an object of class `alphabet`

## Details

- `"bcrypt"`: bcrypt alphabet

- `"bin_hex"`: alphabet used in BinHex 4.0 files

- `"crypt"`: crypt(3) alphabet (with . and / as the first two
  characters)

- `"imap_mutf7"`: alphabet used in IMAP-modified UTF-7 (with + and ,)

- `"standard"`: standard alphabet (with + and /) specified in RFC 4648

- `"url_safe"`: URL-safe alphabet (with - and \_) specified in RFC 4648

See [base64
crate](https://docs.rs/base64/latest/base64/alphabet/index.html#constants)
from where these definitions come.

## Examples

``` r
alphabet("standard")
#> <alphabet>
#> ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/
alphabet("bcrypt")
#> <alphabet>
#> ./ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789
alphabet("bin_hex")
#> <alphabet>
#> !"#$%&'()*+,-0123456789@ABCDEFGHIJKLMNPQRSTUVXYZ[`abcdehijklmpqr
alphabet("crypt")
#> <alphabet>
#> ./0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz
alphabet("imap_mutf7")
#> <alphabet>
#> ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+,
alphabet("url_safe")
#> <alphabet>
#> ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_

new_alphabet("qwertyuiop[]asdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890")
#> <alphabet>
#> qwertyuiop[]asdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890
```
