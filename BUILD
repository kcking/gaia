load("@crate_index//:defs.bzl", "aliases", "all_crate_deps")
load("@rules_rust//wasm_bindgen:wasm_bindgen.bzl", "rust_wasm_bindgen")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library")
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
    cmd_bat = "node bazel-out/host/bin/external/npm/node_modules/tailwindcss/lib/cli.js --output=$(OUTS)",
    tools = ["@npm//tailwindcss"],
)

ts_project(
    name = "tsproject",
    srcs = [
        "app.ts",
        "tsconfig.json",
    ],
    deps = ["@npm//prismjs"],
)

genrule(
    name = "copybundletostatic",
    srcs = [":bundle"],
    outs = ["static/bundle.js"],
    cmd = "cp $(@D)/../bundle.js $(OUTS)",
    cmd_bat = "xcopy $(@D)\\..\\bundle.js $(OUTS)",
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
    srcs = [":app_wasm"],
    outs = ["app_wasm_bg_opt.wasm"],
    # platform-specific path to emsdk linker_files:
    # https://github.com/emscripten-core/emsdk/blob/26a0dea0d3bbf616fa7f0a908e5b08aab406f7c4/bazel/BUILD#L51
    cmd = "external/" + select({
        "@emsdk//:linux": "emscripten_bin_linux",
        "@emsdk//:macos": "emscripten_bin_mac",
        "@emsdk//:macos_arm64": "emscripten_bin_mac_arm64",
        "@emsdk//:windows": "emscripten_bin_win",
    }) + "/bin/wasm-opt $(@D)/app_wasm_bg.wasm -o $@ -Os",
    tools = ["@emsdk//:linker_files"],
)

# TODO: use included pkg_tar rule to compress this or rules_brotli
genrule(
    name = "app_wasm_opt_br",
    srcs = [":app_wasm_opt"],
    outs = ["app_wasm_bg_opt.wasm.br"],
    cmd = "./bazel-out/host/bin/external/brotli/brotli -9 $<",
    tools = ["@brotli"],
)
