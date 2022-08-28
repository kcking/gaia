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
        ":prismjs",
        ":copybundletostatic",
        ":index.html-dev",
        ":index.html-release",
    ],
)

genrule(
    name = "tailwind",
    srcs = glob(["src/**/*.rs"]) + ["tailwind.config.js"],
    outs = ["static/tailwind.css"],
    cmd = "node bazel-out/host/bin/external/npm/node_modules/tailwindcss/lib/cli.js --output=$(OUTS)",
    tools = ["@npm//tailwindcss"],
)

genrule(
    name = "prismjs",
    outs = ["static/prism.js"],
    cmd = "cp bazel-out/host/bin/external/npm/node_modules/prismjs/prism.js $(OUTS)",
    tools = ["@npm//prismjs"],
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

INDEX_HTML_TEMPLATE = """
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta
      name="viewport"
      data-bin="app"
      content="width=device-width, initial-scale=1"
    />
    <link href="/tailwind.css" rel="stylesheet" />
    <link href="/styles/main.css" rel="stylesheet" />
    <link href="/styles/prism-vs.css" rel="stylesheet" />
    <link href="/styles/prism-vs-dark-plus.css" rel="stylesheet" />

    <title>impl Future {{{{ }}}}</title>
    <link rel="icon" href="/img/favicon.svg" />

    <link rel="preconnect" href="https://fonts.googleapis.com" />
    <link
      rel="preconnect"
      href="https://fonts.gstatic.com"
      crossorigin="true"
    />
    <link
      href="https://fonts.googleapis.com/css2?family=Major+Mono+Display&family=Raleway:ital,wght@0,500;0,600;0,700;0,800;1,500&display=swap"
      rel="stylesheet"
    />

    <script>
      window.Prism = window.Prism || new Object();
      Prism.manual = true;
    </script>
    <script src="/bundle.js"></script>
    <script type="module">
      import init from "{}";
      init("{}");
    </script>
  </head>
  <body></body>
</html>
"""

# generate index.html
genrule(
    name = "index.html-dev",
    outs = ["static/index.dev.html"],
    cmd = """cat - > $@ <<BAZELEOF
    {}
    """.format(INDEX_HTML_TEMPLATE.format("app_wasm.js", "/app_wasm_bg.wasm")),
)

genrule(
    name = "index.html-release",
    outs = ["static/index.release.html"],
    cmd = """cat - > $@ <<BAZELEOF
    {}
    """.format(INDEX_HTML_TEMPLATE.format("app_wasm.js", "/app_wasm_bg_opt.wasm")),
)
