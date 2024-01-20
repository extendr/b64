
<!-- README.md is generated from README.Rmd. Please edit that file -->

# b64 <img src="man/figures/logo.svg" align="right" height="139" alt="" />

<!-- badges: start -->

[![R-CMD-check](https://github.com/extendr/b64/actions/workflows/R-CMD-check.yaml/badge.svg)](https://github.com/extendr/b64/actions/workflows/R-CMD-check.yaml)
<!-- badges: end -->

The goal of b64 is to provide a very fast, lightweight, and vectorized
base64 encoder and decoder.

## Installation

You can install the development version of b64 like so:

``` r
pak::pak("extendr/b64")
```

## Example

Encode to base64 using `encode()`.

``` r
library(b64)

hello <- encode("Hello, from extendr")
hello
#> [1] "SGVsbG8sIGZyb20gZXh0ZW5kcg=="
```

Decode using `decode()`. Note that the returned object will always have
the `"blob"` class. To achieve 0 dependencies, the `blob` package is
only listed as a suggested dependency but if you attach it, its print
method will be used.

``` r
library(blob)
decoded <- decode(hello)
decoded
#> <blob[1]>
#> [1] blob[19 B]
```

We can convert the decoded base64 to characters and see how it worked.

``` r
rawToChar(decoded[[1]])
#> [1] "Hello, from extendr"
```

### Vectorized

Both `encode()` and `decode()` are vectorized.

``` r
lorem <- unlist(lorem::ipsum(5, 1,  5))
lorem
#> [1] "Elit porttitor litora phasellus primis."                        
#> [2] "Sit vel natoque eu quisque."                                    
#> [3] "Sit accumsan elementum pharetra aliquet parturient ullamcorper."
#> [4] "Consectetur iaculis nunc elementum."                            
#> [5] "Dolor donec iaculis sem."

encoded <- encode(lorem)
encoded
#> [1] "RWxpdCBwb3J0dGl0b3IgbGl0b3JhIHBoYXNlbGx1cyBwcmltaXMu"                                
#> [2] "U2l0IHZlbCBuYXRvcXVlIGV1IHF1aXNxdWUu"                                                
#> [3] "U2l0IGFjY3Vtc2FuIGVsZW1lbnR1bSBwaGFyZXRyYSBhbGlxdWV0IHBhcnR1cmllbnQgdWxsYW1jb3JwZXIu"
#> [4] "Q29uc2VjdGV0dXIgaWFjdWxpcyBudW5jIGVsZW1lbnR1bS4="                                    
#> [5] "RG9sb3IgZG9uZWMgaWFjdWxpcyBzZW0u"
```

We can decode all of these using `decode()` as well.

``` r
decode(encoded)
#> <blob[5]>
#> [1] blob[39 B] blob[27 B] blob[63 B] blob[35 B] blob[24 B]
```

## Encoding and decoding files

`b64` shines when encoding and decoding files. `encode_file()` and
`decode_file()` both work by reading a file as a stream making it far
faster than the alternative.

``` r
tmp <- tempfile() 
fp <- "https://github.com/datablist/sample-csv-files/raw/main/files/leads/leads-100000.csv"

download.file(fp, tmp)

bench::mark(
  b64 = encode_file(tmp),
  base64enc = base64enc::base64encode(tmp)
)
#> Warning: Some expressions had a GC in every iteration; so filtering is
#> disabled.
#> # A tibble: 2 × 6
#>   expression      min   median `itr/sec` mem_alloc `gc/sec`
#>   <bch:expr> <bch:tm> <bch:tm>     <dbl> <bch:byt>    <dbl>
#> 1 b64          67.5ms   74.2ms     13.7     24.1MB     1.96
#> 2 base64enc   177.9ms  183.3ms      5.50    66.9MB    11.0
```

While the encoding is very impressive, better yet is the decoding
performance.

``` r
# create a temp file
tmp2 <- tempfile()

# encode it and write to tmep file
encode_file(tmp) |>
  charToRaw() |>
  writeBin(tmp2)

bench::mark(
  b64 = decode_file(tmp2),
  base64enc = base64enc::base64decode(file(tmp2))
)
#> # A tibble: 2 × 6
#>   expression      min   median `itr/sec` mem_alloc `gc/sec`
#>   <bch:expr> <bch:tm> <bch:tm>     <dbl> <bch:byt>    <dbl>
#> 1 b64          43.4ms   51.7ms     19.7     18.1MB     5.63
#> 2 base64enc   356.7ms  373.3ms      2.68    18.1MB     0
```

## Alternative engines

Out of the box, `b64` provides a number of pre-configured engines that
can be used. The function `engine()` allows you to choose one of these
different engines For example, `engine("url_safe")` provides a standard
engine that uses a url-safe alphabet with padding.

``` r
url_engine <- engine("url_safe")
url_safe_encoded <- encode("\xfa\xec U", url_engine)
url_safe_encoded
#> [1] "-uwgVQ=="
```

If we try to decode this using the standard engine, we will encounter an
error.

``` r
decode(url_safe_encoded)
#> Error in decode_(what, eng): Invalid byte 45, offset 0.
```

We can use our new engine to decode it.

``` r
decode(url_safe_encoded, url_engine)
#> <blob[1]>
#> [1] blob[4 B]
```

### Custom Engines

We can create custom engines with `new_engine()`. This allows us to
provide our on alphabet and configuration.

We can use one of the many predefined alphabets or create one our selves
with `new_alphabet()`. We can also specify our engine config using
`new_config()` which lets us choose whether or not to pad and how to
handle decoding.

``` r
my_eng <- new_engine(
  alphabet("crypt"),
  new_config(TRUE, TRUE, "none")
)
```

This engine can be used to encode or decode text.

``` r
txt <- "lorem ipsum sit dolor amet"

encode(txt, my_eng)
#> [1] "P4xmNKoUOL/nRKoUQqZo64FjP4xm643hNLE="
```

Compare this to the standard encoder:

``` r
encode(txt)
#> [1] "bG9yZW0gaXBzdW0gc2l0IGRvbG9yIGFtZXQ="
```
