load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test")

rust_library(
  name = "lexer",
  srcs = ["lexer.rs"],
  deps = ["//src/token"],
  visibility = ["//src:__subpackages__"],
)

rust_test(
  name = "lexer_test",
  crate = ":lexer",
  deps = [
     "@crate_index//:stringreader",
  ]
)
