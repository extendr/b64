
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

Decode using `decode()`

``` r
decoded <- decode(hello)
decoded
#> [[1]]
#>  [1] 48 65 6c 6c 6f 2c 20 66 72 6f 6d 20 65 78 74 65 6e 64 72
#> 
#> attr(,"class")
#> [1] "blob"          "vctrs_list_of" "vctrs_vctr"    "list"
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
#> [1] "Dolor malesuada cursus faucibus facilisi accumsan viverra?"   
#> [2] "Sit penatibus lobortis at aptent pellentesque!"               
#> [3] "Sit euismod accumsan semper ante cubilia nam velit himenaeos!"
#> [4] "Lorem pulvinar augue aliquam!"                                
#> [5] "Dolor nullam facilisi senectus penatibus."

encoded <- encode(lorem)
encoded
#> [1] "RG9sb3IgbWFsZXN1YWRhIGN1cnN1cyBmYXVjaWJ1cyBmYWNpbGlzaSBhY2N1bXNhbiB2aXZlcnJhPw=="    
#> [2] "U2l0IHBlbmF0aWJ1cyBsb2JvcnRpcyBhdCBhcHRlbnQgcGVsbGVudGVzcXVlIQ=="                    
#> [3] "U2l0IGV1aXNtb2QgYWNjdW1zYW4gc2VtcGVyIGFudGUgY3ViaWxpYSBuYW0gdmVsaXQgaGltZW5hZW9zIQ=="
#> [4] "TG9yZW0gcHVsdmluYXIgYXVndWUgYWxpcXVhbSE="                                            
#> [5] "RG9sb3IgbnVsbGFtIGZhY2lsaXNpIHNlbmVjdHVzIHBlbmF0aWJ1cy4="
```

We can decode all of these using `decode()` as well. This will always
return a `blob` object.

``` r
decode(encoded)
#> [[1]]
#>  [1] 44 6f 6c 6f 72 20 6d 61 6c 65 73 75 61 64 61 20 63 75 72 73 75 73 20 66 61
#> [26] 75 63 69 62 75 73 20 66 61 63 69 6c 69 73 69 20 61 63 63 75 6d 73 61 6e 20
#> [51] 76 69 76 65 72 72 61 3f
#> 
#> [[2]]
#>  [1] 53 69 74 20 70 65 6e 61 74 69 62 75 73 20 6c 6f 62 6f 72 74 69 73 20 61 74
#> [26] 20 61 70 74 65 6e 74 20 70 65 6c 6c 65 6e 74 65 73 71 75 65 21
#> 
#> [[3]]
#>  [1] 53 69 74 20 65 75 69 73 6d 6f 64 20 61 63 63 75 6d 73 61 6e 20 73 65 6d 70
#> [26] 65 72 20 61 6e 74 65 20 63 75 62 69 6c 69 61 20 6e 61 6d 20 76 65 6c 69 74
#> [51] 20 68 69 6d 65 6e 61 65 6f 73 21
#> 
#> [[4]]
#>  [1] 4c 6f 72 65 6d 20 70 75 6c 76 69 6e 61 72 20 61 75 67 75 65 20 61 6c 69 71
#> [26] 75 61 6d 21
#> 
#> [[5]]
#>  [1] 44 6f 6c 6f 72 20 6e 75 6c 6c 61 6d 20 66 61 63 69 6c 69 73 69 20 73 65 6e
#> [26] 65 63 74 75 73 20 70 65 6e 61 74 69 62 75 73 2e
#> 
#> attr(,"class")
#> [1] "blob"          "vctrs_list_of" "vctrs_vctr"    "list"
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
#> 1 b64          76.9ms   79.8ms     12.6     24.1MB      0  
#> 2 base64enc     189ms  199.7ms      5.07    66.9MB     10.1
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
#> 1 b64            46ms   47.1ms     21.2     18.1MB     5.30
#> 2 base64enc     308ms  314.1ms      3.18    18.1MB     0
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
#> [[1]]
#> [1] fa ec 20 55
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
