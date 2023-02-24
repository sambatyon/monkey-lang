load("@rules_rust//rust:defs.bzl", "rust_library")

rust_library(
  name = "ast",
  srcs = ["ast.rs"],
  deps = [
    "//src/token",
    "//src/lexer",
  ],
  visibility = ["//src:__subpackages__"],
)
