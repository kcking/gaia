load("emsdk.bzl", "declare_toolchains")

package(default_visibility = ["//visibility:public"])

toolchain_type(name = "toolchain_type")

declare_toolchains(
    name = "wasmopt_toolchains",
)

# emsdk_toolchain(
#     name = "emsdk_mac_arm64",
#     linker_files = "@emscripten_bin_mac_arm64//:linker_files",
#     wasmopt_path = "external/emscripten_bin_mac_arm64/bin/wasm-opt",
# )

# toolchain(
#     name = "emsdk_mac_arm64_toolchain",
#     exec_compatible_with = [
#         "@platforms//os:macos",
#         "@platforms//cpu:arm64",
#     ],
#     toolchain = ":emsdk_mac_arm64",
#     toolchain_type = ":toolchain_type",
# )
