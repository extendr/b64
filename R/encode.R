#' Encode and decode using base64
#'
#' @param what a character, raw, or blob vector
#' @param eng a base64 engine. See [engine()] for details.
#' @param path a path to a base64 encoded file.
#'
#' @return
#' Both `encode()` and `decode()` are vectorized. They will return a character
#' and blob vector the same length as `what`, respectively.
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
