#' Choose an encoding engine
#'
#' @param which which base64 encoding engine to use.
#'
#' @export
engine <- function(which = "standard") {
  provided <- c("standard", "standard_no_pad", "url_safe", "url_safe_no_pad")
  rlang::arg_match0(which, provided)
  structure(engine_(which), class = "engine")
}
