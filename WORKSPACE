load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository")

RULES_RUST_VERSION = "0.10.0"

http_archive(
    name = "rules_rust",
    sha256 = "0cc7e6b39e492710b819e00d48f2210ae626b717a3ab96e048c43ab57e61d204",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/rules_rust/releases/download/{RULES_RUST_VERSION}/rules_rust-v{RULES_RUST_VERSION}.tar.gz".format(RULES_RUST_VERSION = RULES_RUST_VERSION),
        "https://github.com/bazelbuild/rules_rust/releases/download/{RULES_RUST_VERSION}/rules_rust-v{RULES_RUST_VERSION}.tar.gz".format(RULES_RUST_VERSION = RULES_RUST_VERSION),
    ],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(extra_target_triples = [
    "wasm32-unknown-unknown",
    "x86_64-unknown-linux-gnu",
    "aarch64-unknown-linux-gnu",
])

load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")

crate_universe_dependencies()

load("@rules_rust//crate_universe:defs.bzl", "crates_repository", "splicing_config")

crates_repository(
    name = "crate_index",
    cargo_lockfile = "//:Cargo.lock",
    generator_sha256s = {"aarch64-apple-darwin": "77e55540c26bf180b3f6bb5523f24aeef02d4c1b4525426cf13c547aa9698c75"},
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
    name = "root_npm",
    package_json = "//:package.json",
    yarn_lock = "//:yarn.lock",
)

yarn_install(
    name = "app_npm",
    package_json = "//bundle:package.json",
    yarn_lock = "//bundle:yarn.lock",
)

load("@build_bazel_rules_nodejs//toolchains/esbuild:esbuild_repositories.bzl", "esbuild_repositories")

esbuild_repositories(npm_repository = "app_npm")

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
    path = "../dev/bazel-zig-cc",
)
# git_repository(
#     name = "bazel-zig-cc",
#     commit = "79674a1d966b5c196a6f729ac7423223dc05cf83",
#     remote = "https://git.sr.ht/~motiejus/bazel-zig-cc",
# )

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
    # "@zig_sdk//toolchain:windows_amd64",
    "@zig_sdk//toolchain:windows_arm64",
)

# compression for large wasm bundle
http_archive(
    name = "brotli",
    strip_prefix = "brotli-1.0.9",
    urls = ["https://github.com/google/brotli/archive/refs/tags/v1.0.9.zip"],
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

# gazelle needed for container_push rule
http_archive(
    name = "bazel_gazelle",
    sha256 = "5982e5463f171da99e3bdaeff8c0f48283a7a5f396ec5282910b9e8a49c0dd7e",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/bazel-gazelle/releases/download/v0.25.0/bazel-gazelle-v0.25.0.tar.gz",
        "https://github.com/bazelbuild/bazel-gazelle/releases/download/v0.25.0/bazel-gazelle-v0.25.0.tar.gz",
    ],
)

load("@io_bazel_rules_go//go:deps.bzl", "go_register_toolchains", "go_rules_dependencies")
load("@bazel_gazelle//:deps.bzl", "gazelle_dependencies")

go_rules_dependencies()

go_register_toolchains(version = "1.18.3")

gazelle_dependencies()

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
    # architecture = "arm64",
    # digest = "sha256:85e93527f62963332bf6ca0157d8ebd09ef72e34eff087b6f5ea05020da1dbdc",
    registry = "gcr.io",
    # rust is c-like https://github.com/GoogleContainerTools/distroless/tree/main/cc
    repository = "distroless/cc",
)

container_pull(
    name = "cc_base_amd64",
    architecture = "amd64",
    registry = "gcr.io",
    repository = "distroless/cc",
)
