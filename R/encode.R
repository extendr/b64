#' Encode and decode using base64
#'
#' @details
#'
#' ## Encoding
#'
#' - `encode()` takes a character vector, list of raw vectors (or blob class), or a raw vector and encodes them into base64 strings.
#' - `encode_file()` takes a path to a file and encodes it as a base64 string.
#'
#' ## Decoding
#'
#' - `decode()` will decode either a base64 encoded character scalar, a raw vector, or a list of raw vectors (see blob package).
#' - `decode_file()` will decode a base64 encoded file into a raw vector.
#' - `decode_as_string()` is designed to decode a base64 encoded string to a utf-8 string. By default, it will decode a chunked base64 encoded strings using `\n` as the separator. Use the `newline` argument to determine how to split the input string prior to decoding.
#'
#'
#' @param what a character, raw, or blob vector
#' @param eng a base64 engine. See [engine()] for details.
#' @param path a path to a base64 encoded file.
#' @param newline a character sequence to split in the input base64 encoded string on before decoding.
#'
#' @return
#' Both `encode()` and `decode()` are vectorized. They will return a character
#' and blob vector the same length as `what`, respectively.
#'
#' `decode_as_string()` returns a character scalar.
#' @export
#' @name encode
#' @examples
#' # encode hello world
#' encoded <- encode("Hello world")
#' encoded
#'
#' # decode to a blob
#' decoded <- decode(encoded)
#' decoded
#'
#' # convert back to a character
#' rawToChar(decoded[[1]])
encode <- function(what, eng = engine()) {
  n <- length(what)
  if (inherits(what, "raw") || (n == 1 & inherits(what, "character"))) {
    encode_(what, eng)
  } else {
    encode_vectorized_(what, eng)
  }
}

#' @export
#' @rdname encode
decode <- function(what, eng = engine()) {
  n <- length(what)
  if (inherits(what, "raw") || (n == 1 & inherits(what, "character"))) {
    decode_(what, eng)
  } else {
    decode_vectorized_(what, eng)
  }
}

#' @export
#' @rdname encode
decode_as_string <- function(what, newline = "\n", eng = engine()) {
  if (!inherits(what, "character") || length(what) != 1) {
    stop("`what` must be a scalar character vector")
  }

  if (!inherits(newline, "character") || length(newline) != 1) {
    stop("`newline` must be a scalar character vector")
  }

  decode_as_string_(what, newline, eng)
}


#' @export
#' @name encode
encode_file <- function(path, eng = engine()) {
  if (!file.exists(path)) {
    stop(paste0("`", path, "` does not exist"))
  }

  encode_file_(path, eng)
}


#' @export
#' @rdname encode
decode_file <- function(path, eng = engine()) {
  if (!file.exists(path)) {
    stop(paste0("`", path, "` does not exist"))
  }

  decode_file_(path, eng)
}
