load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test")

rust_library(
  name = "parser",
  srcs = ["parser.rs"],
  deps = [
    "//src/ast",
    "//src/token",
    "//src/lexer",
  ],
  visibility = ["//src:__subpackages__"],
)

rust_test(
  name = "parser_test",
  crate = ":parser",
  deps = [
     "@crate_index//:stringreader",
  ]
)
