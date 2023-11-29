workspace(name="monkey")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# To find additional information on this release or newer ones visit:
# https://github.com/bazelbuild/rules_rust/releases
http_archive(
    name = "rules_rust",
    sha256 = "36ab8f9facae745c9c9c1b33d225623d976e78f2cc3f729b7973d8c20934ab95",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.31.0/rules_rust-v0.31.0.tar.gz"],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")
rules_rust_dependencies()
rust_register_toolchains(edition="2021", versions = ["1.74.0"])

load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")
crate_universe_dependencies()

load("@rules_rust//crate_universe:defs.bzl", "crates_repository", "crate", "render_config")
crates_repository(
    name = "crate_index",
    # Update lock file running: CARGO_BAZEL_REPIN=true bazel build //...
    cargo_lockfile = "//config:cargo.bazel.lock",
    packages = {
        "once_cell": crate.spec(
            version = "1.18.0",
        ),
        "stringreader": crate.spec(
            version = "0.1.1",
        ),
    },
    render_config = render_config(
        default_package_name = ""
    ),
)

load("@crate_index//:defs.bzl", "crate_repositories")
crate_repositories()

load("@rules_rust//tools/rust_analyzer:deps.bzl", "rust_analyzer_dependencies")
rust_analyzer_dependencies()

load("@rules_rust//rust:repositories.bzl", "rust_analyzer_toolchain_repository")
# To initialize run: bazel run @rules_rust//tools/rust_analyzer:gen_rust_project
register_toolchains(rust_analyzer_toolchain_repository(
    name = "rust_analyzer_toolchain",
    # This should match the currently registered toolchain.
    version = "1.67.0",
))
