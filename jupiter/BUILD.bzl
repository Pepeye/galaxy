load("@crates//:defs.bzl", "crates_from")
load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    name = "jupiter",
    srcs = [
			"src/main.rs"
		],
    deps = crates_from("//jupiter:Cargo.toml") + ["//neptune"],
)
