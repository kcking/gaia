load("@crate_index//:defs.bzl", "aliases", "all_crate_deps")
load("@rules_rust//wasm_bindgen:wasm_bindgen.bzl", "rust_wasm_bindgen")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library", "rust_shared_library", "rust_test")
load("@npm//@bazel/esbuild:index.bzl", "esbuild", "esbuild_config")
load("@npm//@bazel/typescript:index.bzl", "ts_project")

package(
    default_visibility = ["//:__subpackages__"],
)

rust_binary(
    name = "app",
    srcs = ["src/bin/app.rs"],
    aliases = aliases(),
    edition = "2021",
    proc_macro_deps = all_crate_deps(
        proc_macro = True,
    ),
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
    deps = all_crate_deps(
        normal = True,
    ),
)

rust_wasm_bindgen(
    name = "app_wasm",
    target = "web",
    wasm_file = ":app",
)

filegroup(
    name = "static_files",
    srcs = glob(["static/**"]) + [
        ":tailwind",
        ":copybundletostatic",
    ],
)

genrule(
    name = "tailwind",
    srcs = glob(["src/**/*.rs"]) + ["tailwind.config.js"],
    outs = ["static/tailwind.css"],
    cmd = "node bazel-out/host/bin/external/npm/node_modules/tailwindcss/lib/cli.js --output=$(OUTS)",
    tools = ["@npm//tailwindcss"],
)

ts_project(
    name = "tsproject",
    srcs = ["app.ts"],
    deps = ["@npm//prismjs"],
)

genrule(
    name = "copybundletostatic",
    srcs = [":bundle"],
    outs = ["static/bundle.js"],
    cmd = "cp $(@D)/../bundle.js $(OUTS)",
)

esbuild(
    name = "bundle",
    config = ":esbuild_config",
    entry_point = "app.ts",
    deps = [":tsproject"],
)

esbuild_config(
    name = "esbuild_config",
    config_file = "esbuild.config.mjs",
    deps = [
        "@npm//esbuild",
        "@npm//esbuild-plugin-prismjs",
    ],
)

genrule(
    name = "app_wasm_opt",
    srcs = ["app_wasm"],
    outs = ["app_wasm_bg_opt.wasm"],
    cmd = "$$(echo \"$(locations @emsdk//:linker_files)\" | fmt -w 1 | grep wasm-opt) $$(echo \"$(locations :app_wasm)\" | fmt -w 1 | grep app_wasm_bg.wasm) -o $@ -Os",
    tools = ["@emsdk//:linker_files"],
)

genrule(
    name = "app_wasm_opt_gz",
    srcs = [":app_wasm_opt"],
    outs = ["app_wasm_bg_opt.wasm.gz"],
    # -k: don't delete input, -f: compress links
    cmd = "gzip -f -k -9 $<",
)
