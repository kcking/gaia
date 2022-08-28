load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# http_archive(
#     name = "rules_rust",
#     sha256 = "6bfe75125e74155955d8a9854a8811365e6c0f3d33ed700bc17f39e32522c822",
#     urls = [
#         "https://mirror.bazel.build/github.com/bazelbuild/rules_rust/releases/download/0.9.0/rules_rust-v0.9.0.tar.gz",
#         "https://github.com/bazelbuild/rules_rust/releases/download/0.9.0/rules_rust-v0.9.0.tar.gz",
#     ],
# )

local_repository(
    name = "rules_rust",
    path = "../../dev/rules_rust",
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(extra_target_triples = [
    "wasm32-unknown-unknown",
    "x86_64-unknown-linux-gnu",
])

load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")

crate_universe_dependencies()

load("@rules_rust//crate_universe:defs.bzl", "crates_repository", "splicing_config")

crates_repository(
    name = "crate_index",
    cargo_lockfile = "//:Cargo.lock",
    lockfile = "//:cargo-bazel.lock.json",
    manifests = [
        "//:Cargo.toml",
        "//server:Cargo.toml",
        "//native-test:Cargo.toml",
    ],
    splicing_config = splicing_config(resolver_version = "2"),
)

load("@crate_index//:defs.bzl", "crate_repositories")

crate_repositories()

load("@rules_rust//wasm_bindgen:repositories.bzl", "rust_wasm_bindgen_repositories")

rust_wasm_bindgen_repositories()

# for tailwind
http_archive(
    name = "build_bazel_rules_nodejs",
    sha256 = "f10a3a12894fc3c9bf578ee5a5691769f6805c4be84359681a785a0c12e8d2b6",
    urls = ["https://github.com/bazelbuild/rules_nodejs/releases/download/5.5.3/rules_nodejs-5.5.3.tar.gz"],
)

load("@build_bazel_rules_nodejs//:repositories.bzl", "build_bazel_rules_nodejs_dependencies")

build_bazel_rules_nodejs_dependencies()

load("@build_bazel_rules_nodejs//:index.bzl", "node_repositories", "yarn_install")

node_repositories()

yarn_install(
    name = "npm",
    package_json = "//:package.json",
    yarn_lock = "//:yarn.lock",
)

load("@build_bazel_rules_nodejs//toolchains/esbuild:esbuild_repositories.bzl", "esbuild_repositories")

esbuild_repositories(npm_repository = "npm")

# for wasm-opt
http_archive(
    name = "emsdk",
    sha256 = "189149281f36ca8edfad2632aa7c3a028bb116b7dbb967c4dac2f6496f60702c",
    strip_prefix = "emsdk-3.1.19/bazel",
    url = "https://github.com/emscripten-core/emsdk/archive/refs/tags/3.1.19.tar.gz",
)

load("@emsdk//:deps.bzl", emsdk_deps = "deps")

emsdk_deps()

load("@emsdk//:emscripten_deps.bzl", emsdk_emscripten_deps = "emscripten_deps")

emsdk_emscripten_deps(emscripten_version = "3.1.19")

# zigcc
BAZEL_ZIG_CC_VERSION = "v0.9.1"

# http_archive(
#     name = "bazel-zig-cc",
#     sha256 = "ab596041c0217a66ed8e6af49955c5d427b1f3e5b5603713696b3444810608f0",
#     strip_prefix = "bazel-zig-cc-{}".format(BAZEL_ZIG_CC_VERSION),
#     urls = ["https://git.sr.ht/~motiejus/bazel-zig-cc/archive/{}.tar.gz".format(BAZEL_ZIG_CC_VERSION)],
# )
local_repository(
    name = "bazel-zig-cc",
    path = "../../dev/bazel-zig-cc",
)

load("@bazel-zig-cc//toolchain:defs.bzl", zig_toolchains = "toolchains")

zig_toolchains()

register_toolchains(
    "@zig_sdk//toolchain:linux_amd64_gnu.2.19",
    "@zig_sdk//toolchain:linux_arm64_gnu.2.28",
    # macos toolchains fail with iconv error that I'm not sure how to fix:
    # https://github.com/ziglang/zig/issues/10485#issuecomment-1013533258
    # don't register them so we just use local toolchain when on a mac
    # "@zig_sdk//toolchain:darwin_amd64",
    # "@zig_sdk//toolchain:darwin_arm64",
    "@zig_sdk//toolchain:windows_amd64",
    "@zig_sdk//toolchain:windows_arm64",
)

# go required for docker-less container operations
http_archive(
    name = "io_bazel_rules_go",
    sha256 = "16e9fca53ed6bd4ff4ad76facc9b7b651a89db1689a2877d6fd7b82aa824e366",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/rules_go/releases/download/v0.34.0/rules_go-v0.34.0.zip",
        "https://github.com/bazelbuild/rules_go/releases/download/v0.34.0/rules_go-v0.34.0.zip",
    ],
)

load("@io_bazel_rules_go//go:deps.bzl", "go_register_toolchains", "go_rules_dependencies")

go_rules_dependencies()

go_register_toolchains(version = "1.18.3")

# docker
http_archive(
    name = "io_bazel_rules_docker",
    sha256 = "b1e80761a8a8243d03ebca8845e9cc1ba6c82ce7c5179ce2b295cd36f7e394bf",
    urls = ["https://github.com/bazelbuild/rules_docker/releases/download/v0.25.0/rules_docker-v0.25.0.tar.gz"],
)

load(
    "@io_bazel_rules_docker//repositories:repositories.bzl",
    container_repositories = "repositories",
)

container_repositories()

load("@io_bazel_rules_docker//repositories:deps.bzl", container_deps = "deps")

container_deps()

load(
    "@io_bazel_rules_docker//container:container.bzl",
    "container_pull",
)

container_pull(
    name = "cc_base",
    registry = "gcr.io",
    # rust is c-like https://github.com/GoogleContainerTools/distroless/tree/main/cc
    repository = "distroless/cc",
)
