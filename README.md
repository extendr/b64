
<!-- README.md is generated from README.Rmd. Please edit that file -->

# b64 <img src="man/figures/logo.svg" align="right" height="139" alt="" />

<!-- badges: start -->

[![R-CMD-check](https://github.com/extendr/b64/actions/workflows/R-CMD-check.yaml/badge.svg)](https://github.com/extendr/b64/actions/workflows/R-CMD-check.yaml)
<!-- badges: end -->

b64 is a dependency free, fast, lightweight, and vectorized base64
encoder and decoder.

## Installation

You can install b64 from CRAN with

``` r
install.packages("b64")
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
#> [1] "Consectetur in sapien interdum diam lobortis eros?"                  
#> [2] "Lorem sed ligula fames?"                                             
#> [3] "Adipiscing suscipit magna sapien varius."                            
#> [4] "Elit tellus taciti turpis hendrerit sagittis."                       
#> [5] "Sit suspendisse ultrices augue class parturient ultricies venenatis."

encoded <- encode(lorem)
encoded
#> [1] "Q29uc2VjdGV0dXIgaW4gc2FwaWVuIGludGVyZHVtIGRpYW0gbG9ib3J0aXMgZXJvcz8="                        
#> [2] "TG9yZW0gc2VkIGxpZ3VsYSBmYW1lcz8="                                                            
#> [3] "QWRpcGlzY2luZyBzdXNjaXBpdCBtYWduYSBzYXBpZW4gdmFyaXVzLg=="                                    
#> [4] "RWxpdCB0ZWxsdXMgdGFjaXRpIHR1cnBpcyBoZW5kcmVyaXQgc2FnaXR0aXMu"                                
#> [5] "U2l0IHN1c3BlbmRpc3NlIHVsdHJpY2VzIGF1Z3VlIGNsYXNzIHBhcnR1cmllbnQgdWx0cmljaWVzIHZlbmVuYXRpcy4="
```

We can decode all of these using `decode()` as well.

``` r
decode(encoded)
#> <blob[5]>
#> [1] blob[50 B] blob[23 B] blob[40 B] blob[45 B] blob[68 B]
```

## Encoding and decoding files

`b64` shines when encoding and decoding files. `encode_file()` and
`decode_file()` both work by reading a file as a stream making it far
faster than the alternative.

``` r
tmp <- tempfile() 
fp <- "https://github.com/extendr/b64/blob/main/src/rust/vendor.tar.xz"

download.file(fp, tmp)

bench::mark(
  b64 = encode_file(tmp),
  base64enc = base64enc::base64encode(tmp)
)
#> # A tibble: 2 × 6
#>   expression      min   median `itr/sec` mem_alloc `gc/sec`
#>   <bch:expr> <bch:tm> <bch:tm>     <dbl> <bch:byt>    <dbl>
#> 1 b64           334µs    340µs     2810.     218KB     0   
#> 2 base64enc     905µs    938µs     1043.     729KB     4.22
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
#> 1 b64        107.09µs  122.6µs     7414.     164KB     8.77
#> 2 base64enc    1.58ms    1.6ms      602.     172KB     3.70
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
