test_that("alphabet: standard", {
  a <- alphabet("standard")
  a2 <- new_alphabet(as.character(a))

  expect_equal(
    as.character(a),
    as.character(a2)
  )
})

test_that("alphabet: bcrypt", {
  a <- alphabet("bcrypt")
  a2 <- new_alphabet(as.character(a))

  expect_equal(
    as.character(a),
    as.character(a2)
  )
})

test_that("alphabet: bin_hex", {
  a <- alphabet("bin_hex")
  a2 <- new_alphabet(as.character(a))

  expect_equal(
    as.character(a),
    as.character(a2)
  )
})

test_that("alphabet: crypt", {
  a <- alphabet("crypt")
  a2 <- new_alphabet(as.character(a))

  expect_equal(
    as.character(a),
    as.character(a2)
  )
})

test_that("alphabet: imap_mutf7", {
  a <- alphabet("imap_mutf7")
  a2 <- new_alphabet(as.character(a))

  expect_equal(
    as.character(a),
    as.character(a2)
  )
})

test_that("alphabet: url_safe", {
  a <- alphabet("url_safe")
  a2 <- new_alphabet(as.character(a))

  expect_equal(
    as.character(a),
    as.character(a2)
  )
})



