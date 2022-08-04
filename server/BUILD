load("@crate_index//:defs.bzl", "aliases", "all_crate_deps")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library", "rust_test")

rust_binary(
    name = "server",
    srcs = glob(["src/**/*.rs"]),
    aliases = aliases(),
    data = [
        "//:app_wasm",
        "//:static_files",
    ],
    edition = "2021",
    proc_macro_deps = all_crate_deps(
        proc_macro = True,
    ),
    deps = all_crate_deps(
        normal = True,
    ) + ["//:gaia"],
)