#' Standard base64 alphabets
#'
#' Create an alphabet from a set of standard base64 alphabets, or use your own.
#'
#' @param which default `"standard"`. Which base64 alphabet to use.
#'  See details for other values.
#' @param chars a character scalar contains 64 unique characters.
#'
#' @details
#'
#' - `"bcrypt"`: bcrypt alphabet
#' - `"bin_hex"`: alphabet used in BinHex 4.0 files
#' - `"crypt"`: crypt(3) alphabet (with . and / as the first two characters)
#' - `"imap_mutf7"`: alphabet used in IMAP-modified UTF-7 (with + and ,)
#' - `"standard"`: standard alphabet (with + and /) specified in RFC 4648
#' - `"url_safe"`: URL-safe alphabet (with - and _) specified in RFC 4648
#'
#' See [base64 crate](https://docs.rs/base64/latest/base64/alphabet/index.html#constants)
#' from where these definitions come.
#'
#' @export
#' @examples
#' alphabet("standard")
#' alphabet("bcrypt")
#' alphabet("bin_hex")
#' alphabet("crypt")
#' alphabet("imap_mutf7")
#' alphabet("url_safe")
#'
#' new_alphabet("qwertyuiop[]asdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890")
#' @returns an object of class `alphabet`
alphabet <- function(which = "standard") {
  match.arg(
    which,
    choices = c("standard", "bcrypt", "bin_hex", "crypt", "imap_mutf7", "url_safe")
  )
  structure(alphabet_(which), class = "alphabet")
}

#' @export
#' @rdname alphabet
new_alphabet <- function(chars) {
  n <- nchar(chars)
  if (nchar(chars) != 64) {
    stop(
      paste(
        "`chars` must contain 64 unique characters. Only", n, "characters were provided."
      )
    )
  }

  structure(new_alphabet_(chars), class = "alphabet")
}


#' @export
print.alphabet <- function(x, ...) {
  cat("<alphabet>\n")
  cat(get_alphabet_(x))
  invisible(x)
}


#' @export
as.character.alphabet <- function(x, ...) {
  get_alphabet_(x)
}
