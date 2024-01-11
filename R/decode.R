#' Decode base64 encodings
#'
#' @inheritParams encode
#' @export
#' @returns
#' `decode()` always returns a `blob` object. `decode_file()` returns a `raw` vector.
decode <- function(what, eng = engine()) {
  n <- length(what)
  if (inherits(what, "raw") || (n == 1 & inherits(what, "character"))) {
    decode_(what, eng)
  } else {
    decode_vectorized_(what, eng)
  }
}

#' @export
#' @rdname decode
decode_file <- function(path, engine = engine()) {
  decode_file_(path, engine)
}
