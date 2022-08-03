load("@crate_index//:defs.bzl", "aliases", "all_crate_deps")
load("@rules_rust//wasm_bindgen:wasm_bindgen.bzl", "rust_wasm_bindgen")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library", "rust_shared_library", "rust_test")
load(":defs.bzl", "wasm_rust_binary")

wasm_rust_binary(
    name = "app",
    srcs = ["src/bin/app.rs"],
    aliases = aliases(),
    edition = "2021",
    proc_macro_deps = all_crate_deps(
        proc_macro = True,
    ),
    visibility = ["//server:__subpackages__"],
    deps = all_crate_deps(
        normal = True,
    ) + [":gaia"],
)

rust_library(
    name = "gaia",
    srcs = glob(
        include = ["src/**/*.rs"],
        exclude = ["src/bin/**"],
    ),
    aliases = aliases(),
    edition = "2021",
    proc_macro_deps = all_crate_deps(
        proc_macro = True,
    ),
    visibility = ["//:__subpackages__"],
    deps = all_crate_deps(
        normal = True,
    ),
)
