% Generated by roxygen2: do not edit by hand
% Please edit documentation in R/extendr-wrappers.R
\name{b64_chunk}
\alias{b64_chunk}
\alias{b64_wrap}
\title{Utility Functions}
\usage{
b64_chunk(encoded, width)

b64_wrap(chunks, newline)
}
\arguments{
\item{encoded}{a character vector of base64 encoded strings.}

\item{width}{a numeric scalar defining the width of the chunks. Must be divisible by 4.}

\item{chunks}{a character vector of base64 encoded strings.}

\item{newline}{a character scalar defining the newline character.}
}
\value{
\itemize{
\item \code{b64_chunk()} returns a list of character vectors.
\item \code{b64_wrap()} returns a scalar character vector.
}
}
\description{
Functions to perform common tasks when working with base64 encoded strings.
}
\details{
\code{b64_chunk()} splits a character vector of base64 encoded strings into chunks of a
specified width.

\code{b64_wrap()} wraps a character vector of base64 encoded strings with a newline character.
}
\examples{
encoded <- encode("Hello, world!")
chunked <- b64_chunk(encoded, 4)
chunked

b64_wrap(chunked, "\n")
}
