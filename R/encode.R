#' Encode as base64
#'
#' Perform base 64 encoding.
#'
#' @details
#'
#' `encode()` performs base 64 encoding for a single
#'
#' @param what a scalar character or a raw vector.
#' @param engine a base64 engine. See [engine()] for details.
#' @return a base64 encoded string.
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
