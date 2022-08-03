"Adapted from https://github.com/envoyproxy/envoy/blob/main/bazel/wasm/wasm.bzl"

load("@rules_rust//rust:defs.bzl", "rust_binary")

def _wasm_rust_transition_impl(settings, attrs):
    return {
        "//command_line_option:platforms": "@rules_rust//rust/platform:wasm",
    }

def _wasi_rust_transition_impl(settings, attrs):
    return {
        "//command_line_option:platforms": "@rules_rust//rust/platform:wasi",
    }

wasm_rust_transition = transition(
    implementation = _wasm_rust_transition_impl,
    inputs = [],
    outputs = [
        "//command_line_option:platforms",
    ],
)

wasi_rust_transition = transition(
    implementation = _wasi_rust_transition_impl,
    inputs = [],
    outputs = [
        "//command_line_option:platforms",
    ],
)

def _wasm_binary_impl(ctx):
    out = ctx.actions.declare_file(ctx.label.name)
    if ctx.attr.precompile:
        ctx.actions.run(
            executable = ctx.executable._compile_tool,
            arguments = [ctx.files.binary[0].path, out.path],
            outputs = [out],
            inputs = ctx.files.binary,
        )
    else:
        ctx.actions.run(
            executable = "cp",
            arguments = [ctx.files.binary[0].path, out.path],
            outputs = [out],
            inputs = ctx.files.binary,
        )

    return [DefaultInfo(files = depset([out]), runfiles = ctx.runfiles([out]))]

def _wasm_attrs(transition):
    return {
        "binary": attr.label(mandatory = True, cfg = transition),
        "precompile": attr.bool(default = False),
        "_whitelist_function_transition": attr.label(default = "@bazel_tools//tools/whitelists/function_transition_whitelist"),
    }

wasm_rust_binary_rule = rule(
    implementation = _wasm_binary_impl,
    attrs = _wasm_attrs(wasm_rust_transition),
)

wasi_rust_binary_rule = rule(
    implementation = _wasm_binary_impl,
    attrs = _wasm_attrs(wasi_rust_transition),
)

def wasm_rust_binary(name, tags = [], wasi = False, precompile = False, **kwargs):
    "Forced wasm rust binary"
    wasm_name = "_wasm_" + name.replace(".", "_")
    kwargs.setdefault("visibility", ["//visibility:public"])

    rust_binary(
        name = wasm_name,
        out_binary = True,
        tags = ["manual"],
        **kwargs
    )

    bin_rule = wasm_rust_binary_rule
    if wasi:
        bin_rule = wasi_rust_binary_rule

    bin_rule(
        visibility = kwargs["visibility"],
        name = name,
        precompile = precompile,
        binary = ":" + wasm_name,
        tags = tags + ["manual"],
    )
