load("@rules_rust//rust:defs.bzl", "rust_library")

rust_library(
  name = "repl",
  srcs = ["repl.rs"],
  deps = [
    "//src/token",
    "//src/lexer",
     "@crate_index//:stringreader",
  ],
  visibility = ["//src:__subpackages__"],
)
