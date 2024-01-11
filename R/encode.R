#' Encode as base64
#'
#' Perform base 64 encoding.
#'
#' @param what a scalar character or a raw vector.
#' @param engine a base64 engine. See [engine()] for details.
#' @return
#' `encode()` is vectorized and will return a character vector of the same
#' lenght as `what`.
#' @export
#' @name encode
encode <- function(what, eng = engine()) {
  n <- length(what)
  if (inherits(what, "raw") || (n == 1 & inherits(what, "character"))) {
    encode_(what, eng)
  } else {
    encode_vectorized_(what, eng)
  }
}

#' @export
#' @name encode
encode_file <- function(path, eng = engine()) {
  encode_file_(path, eng)
}
