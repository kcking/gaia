""" 
conditionally register zig toolchains based on host
currently zig-cc fails to compile for the host target for various reasons
"""

def _zig_toolchains_impl(repository_ctx):
    all_zig_toolchains = [
        "@zig_sdk//toolchain:linux_amd64_gnu.2.19",
        "@zig_sdk//toolchain:linux_arm64_gnu.2.28",
        "@zig_sdk//toolchain:darwin_amd64",
        "@zig_sdk//toolchain:darwin_arm64",
        "@zig_sdk//toolchain:windows_amd64",
        "@zig_sdk//toolchain:windows_arm64",
    ]
    os = repository_ctx.os.name
    arch = repository_ctx.os.arch
    if arch == "aarch64" and os == "mac os x":
        all_zig_toolchains.remove("@zig_sdk//toolchain:darwin_arm64")
    print(os)
    print(arch)
    print(all_zig_toolchains)

    repository_ctx.file("toolchains.bzl", """
def register_toolchains():
    native.register_toolchains("{toolchains}")
    
    """.format(toolchains = '", "'.join(all_zig_toolchains)))

    repository_ctx.file("BUILD", "")
    repository_ctx.file("WORKSPACE", "")

zig_toolchains = repository_rule(
    implementation = _zig_toolchains_impl,
    local = True,
)
