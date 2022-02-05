load("@crates//:defs.bzl", "crates_from")

load("@rules_rust//rust:defs.bzl", "rust_library")
# package(default_visibility = ["//visibility:public"])
rust_library(
    name = "neptune",
		visibility = ["//visibility:public"],
    srcs = glob(["src/**/*.rs"]),
    deps = crates_from("//neptune:Cargo.toml"),
)
