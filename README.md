
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

Encode to base64 using `encode_string()`, `encode_raw()` and
`encode_file()`.

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
#> [1] "Dolor aliquet maecenas est nascetur."                  
#> [2] "Ipsum cum pharetra taciti tempus!"                     
#> [3] "Amet duis viverra fames?"                              
#> [4] "Ipsum curae primis gravida risus auctor mauris aptent."
#> [5] "Amet laoreet ad dictumst non dapibus semper."

encoded <- encode(lorem)
encoded
#> [1] "RG9sb3IgYWxpcXVldCBtYWVjZW5hcyBlc3QgbmFzY2V0dXIu"                        
#> [2] "SXBzdW0gY3VtIHBoYXJldHJhIHRhY2l0aSB0ZW1wdXMh"                            
#> [3] "QW1ldCBkdWlzIHZpdmVycmEgZmFtZXM/"                                        
#> [4] "SXBzdW0gY3VyYWUgcHJpbWlzIGdyYXZpZGEgcmlzdXMgYXVjdG9yIG1hdXJpcyBhcHRlbnQu"
#> [5] "QW1ldCBsYW9yZWV0IGFkIGRpY3R1bXN0IG5vbiBkYXBpYnVzIHNlbXBlci4="
```

We can decode all of these using `decode()` as well. This will always
return a `blob` object.

``` r
decode(encoded)
#> <blob[5]>
#> [1] blob[36 B] blob[33 B] blob[24 B] blob[54 B] blob[44 B]
```

## Encoding and decoding files

`b64` is shines when encoding and decoding files. `encode_file()` and
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
#> 1 b64          39.3ms   40.9ms     24.3       24MB      0  
#> 2 base64enc   111.7ms  112.9ms      8.78    66.5MB     17.6
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
#> 1 b64          16.1ms   16.9ms     56.3       18MB     9.39
#> 2 base64enc     208ms  209.1ms      4.73      18MB     0
```

## TODO

- [ ] provide interface to create custom encoder and decoders
  - custom alphabets
  - padding
  - url safe alphabets
  - streaming encoding and decoding
