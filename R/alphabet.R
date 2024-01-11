format.alphabet <- function(x, ...) {
  cli::cat_line("<alphabet>")
  cat(get_alphabet_(x))
}

print.alphabet <- function(x, ...) {
  format(x, ...)
}


alphabet <- function(which = "standard") {
  rlang::arg_match(
    which,
    values = c("standard", "bcrypt", "bin_hex", "crypt", "imap_mutf7", "url_safe")
  )
  structure(alphabet_(which), class = "alphabet")
}

