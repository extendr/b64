test_that("engine: standard", {
  txt <- "hello world, it is me!"
  expect_equal(
    txt,
    rawToChar(decode(encode(txt))[[1]])
  )
})

test_that("engine: standard_no_pad", {
  eng <- engine("standard_no_pad")
  txt <- "hello world, it is me!"
  encoded <- encode(txt, eng)
  decoded <- decode(encoded, eng)
  expect_equal(
    txt,
    rawToChar(decoded[[1]])
  )
})

test_that("engine: url_safe", {
  eng <- engine("url_safe")
  txt <- "\xfa\xec U"
  encoded <- encode(txt, eng)
  decoded <- decode(encoded, eng)
  expect_equal(
    txt,
    rawToChar(decoded[[1]])
  )
})


test_that("engine: url_safe_no_pad", {
  eng <- engine("url_safe_no_pad")
  txt <- "\xfa\xec U"
  encoded <- encode(txt, eng)
  decoded <- decode(encoded, eng)
  expect_equal(
    txt,
    rawToChar(decoded[[1]])
  )
})
