
<!-- README.md is generated from README.Rmd. Please edit that file -->

# b64

<!-- badges: start -->
<!-- badges: end -->

The goal of b64 is to provide a very fast and lightweight base64 encoder
and decoder and truly open sourced.

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

Decode using `decode()`

``` r
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
#> [1] "Sit eu eleifend id fringilla."  "Dolor ad neque metus metus."   
#> [3] "Dolor at curae proin."          "Elit vivamus torquent taciti." 
#> [5] "Dolor eget velit cum vehicula."

encoded <- encode(lorem)
encoded
#> [1] "U2l0IGV1IGVsZWlmZW5kIGlkIGZyaW5naWxsYS4="
#> [2] "RG9sb3IgYWQgbmVxdWUgbWV0dXMgbWV0dXMu"    
#> [3] "RG9sb3IgYXQgY3VyYWUgcHJvaW4u"            
#> [4] "RWxpdCB2aXZhbXVzIHRvcnF1ZW50IHRhY2l0aS4="
#> [5] "RG9sb3IgZWdldCB2ZWxpdCBjdW0gdmVoaWN1bGEu"
```

We can decode all of these using `decode()` as well. This will always
return a `blob` object.

``` r
decode(encoded)
#> <blob[5]>
#> [1] blob[29 B] blob[27 B] blob[21 B] blob[29 B] blob[30 B]
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
#> 1 b64          40.3ms   41.5ms     23.1       24MB      0  
#> 2 base64enc   110.4ms  113.2ms      8.72    66.5MB     17.4
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
#> 1 b64          17.8ms   20.2ms     49.3       18MB     9.39
#> 2 base64enc   206.4ms  206.7ms      4.83      18MB     0
```

## Alternative engines

Out of the box, `b64` provides a number of pre-configured engines that
can be used. The function `engine()` allows you to choose one of these
different engines For example, `engine("url_safe")` provides a standard
engine that uses a url-safe alphabet with padding.

``` r
unsafe_chars <- charToRaw("-uwgVQA=")

decode(unsafe_chars, engine("url_safe"))
#> <blob[1]>
#> [1] blob[5 B]
```

## TODO

- [ ] provide interface to create custom encoder and decoders
  - custom alphabets
  - padding
  - url safe alphabets
  - streaming encoding and decoding
