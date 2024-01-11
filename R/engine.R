#' Choose an encoding engine
#'
#' @param which default `"standard"`. The base64 encoding engine to be used.
#'  See details for more.
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
#' @export
#' @examples
#' engine()
engine <- function(which = "standard") {
  provided <- c("standard", "standard_no_pad", "url_safe", "url_safe_no_pad")
  rlang::arg_match0(which, provided)
  structure(engine_(which), class = "engine")
}
