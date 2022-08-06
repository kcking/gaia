load("@crate_index//:defs.bzl", "aliases", "all_crate_deps")
load("@rules_rust//wasm_bindgen:wasm_bindgen.bzl", "rust_wasm_bindgen")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library", "rust_shared_library", "rust_test")

rust_binary(
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
    ) + [
        ":gaia",
        "@rules_rust//wasm_bindgen/3rdparty:wasm_bindgen",
    ],
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

rust_wasm_bindgen(
    name = "app_wasm",
    target = "web",
    visibility = ["//server:__subpackages__"],
    wasm_file = ":app",
)

filegroup(
    name = "static_files",
    srcs = glob(["static/**"]) + [":tailwind"],
    visibility = ["//server:__subpackages__"],
)

genrule(
    name = "tailwind",
    srcs = glob(["src/**/*.rs"]) + ["tailwind.config.js"],
    outs = ["static/tailwind.css"],
    cmd = "node bazel-out/host/bin/external/npm/node_modules/tailwindcss/lib/cli.js --output=$(OUTS)",
    tools = ["@npm//tailwindcss"],
)
