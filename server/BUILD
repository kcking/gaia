load("@crate_index//:defs.bzl", "aliases", "all_crate_deps")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library", "rust_test")

rust_binary(
    name = "server",
    srcs = glob(["src/**/*.rs"]),
    aliases = aliases(),
    data = [
        "//:app_wasm",
        "//:index.html-dev",
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

rust_binary(
    name = "opt",
    srcs = glob(["src/**/*.rs"]),
    aliases = aliases(),
    data = [
        "//:app_wasm_opt",
        "//:app_wasm_opt_gz",
        "//:index.html-release",
        "//:static_files",
    ],
    edition = "2021",
    proc_macro_deps = all_crate_deps(
        proc_macro = True,
    ),
    rustc_env = {
        "APP_WASM_FILE": "/app_wasm_bg_opt.wasm",
    },
    deps = all_crate_deps(
        normal = True,
    ) + ["//:gaia"],
)
