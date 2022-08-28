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

rust_binary(
    name = "opt",
    srcs = glob(["src/**/*.rs"]),
    aliases = aliases(),
    data = [
        "//:app_wasm",
        "//:app_wasm_opt",
        "//:app_wasm_opt_gz",
        "//:static_files",
    ],
    edition = "2021",
    proc_macro_deps = all_crate_deps(
        proc_macro = True,
    ),
    rustc_env = {
        "APP_WASM_PATH": "/app_wasm_bg_opt.wasm",
        "AXUM_PRECOMPRESSED_WASM": "1",
    },
    deps = all_crate_deps(
        normal = True,
    ) + ["//:gaia"],
)
