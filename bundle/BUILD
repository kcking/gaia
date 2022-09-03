load("@app_npm//@bazel/typescript:index.bzl", "ts_project")
load("@app_npm//@bazel/esbuild:index.bzl", "esbuild", "esbuild_config")

ts_project(
    name = "tsproject",
    srcs = [
        "app.ts",
    ],
    deps = ["@app_npm//prismjs"],
)

esbuild(
    name = "bundle",
    config = ":esbuild_config",
    entry_point = "app.ts",
    visibility = ["//:__pkg__"],
    deps = [":tsproject"],
)

esbuild_config(
    name = "esbuild_config",
    config_file = "esbuild.config.mjs",
    deps = [
        "@app_npm//esbuild",
        "@app_npm//esbuild-plugin-prismjs",
    ],
)
