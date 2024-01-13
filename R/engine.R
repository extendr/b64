#' Create an encoding engine
#'
#' @param which default `"standard"`. The base64 encoding engine to be used.
#'  See details for more.
#' @param .alphabet an object of class `alphabet` as created with
#'  [`alphabet()`] or [`new_alphabet()`]
#' @param .config an object of class `engine_config` as created with
#'  [new_config()]
#' @details
#'
#' ## Engines
#'
#' By default, the "standard" base64 engine is used which is specified in
#'  [RFC 4648](https://datatracker.ietf.org/doc/html/rfc4648#section-4).
#'
#'  Additional pre-configured base64 engines are provided these are:
#'
#'  - `"standard_no_pad"`: uses the standard engine without padding
#'  - `"url_safe"`: uses a url-safe alphabet with padding
#'  - `"url_safe_no_pad"`: uses a url-safe alphabet without padding
#'
#'  See [base64 crate](https://docs.rs/base64/latest/base64/engine/general_purpose/index.html#constants) for more.
#'
#' @return an object of class `engine`.
#' @export
#' @examples
#' engine()
#' new_engine(alphabet("bcrypt"), new_config())
engine <- function(which = "standard") {
  provided <- c("standard", "standard_no_pad", "url_safe", "url_safe_no_pad")
  rlang::arg_match0(which, provided)
  structure(engine_(which), class = "engine")
}

#' @export
#' @rdname engine
new_engine <- function(.alphabet = alphabet(), .config = new_config()) {

  if (!rlang::inherits_only(.alphabet, "alphabet")) {
    cli::cli_abort(
      c(
        "{.arg .alphabet} is not an object of class {.cls alphabet}",
        "*" = "use {.fn alphabet} for a standard base64 alphabet"
      )
    )
  } else if (!rlang::inherits_only(.config, "engine_config")) {
    cli::cli_abort(
      c(
        "{.arg config} is not a {.cls engine_config} object",
        "*" = "create one with {.fn new_config}"
      )
    )
  }

  res <- new_engine_(.alphabet, .config)
  structure(res, class = "engine")
}

#' @export
print.engine <- function(x, ...) {
  cat("<engine>")
  invisible(x)
}
