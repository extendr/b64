#' @export
format.alphabet <- function(x, ...) {
  cat("<alphabet>\n")
  cat(get_alphabet_(x))
}

#' @export
print.alphabet <- function(x, ...) {
  format(x, ...)
}


#' Standard base64 alphabets
#'
#' @param which default `"standard"`. Which base64 alphabet to use.
#'
#' @details
#'
#' - `"bcrypt"`: the bcrypt alphabet
#'
#' See [base64 crate](https://docs.rs/base64/latest/base64/alphabet/index.html#constants)
#' for more details.
#' @export
alphabet <- function(which = "standard") {
  rlang::arg_match(
    which,
    values = c("standard", "bcrypt", "bin_hex", "crypt", "imap_mutf7", "url_safe")
  )
  structure(alphabet_(which), class = "alphabet")
}

