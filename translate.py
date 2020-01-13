#!/usr/bin/env python3
import argparse
import json
import hashlib
import os
import shlex
import shutil
import sys
from plumbum.cmd import mv, mkdir, sed, rustc, cargo, rm
from plumbum import local, FG

# Path to the root of the krb5 codebase
KRB_BASE = os.path.abspath(os.path.join(os.path.dirname(__file__), "code/src/"))
# KRB_DIR = os.path.abspath(os.path.join(KRB_BASE, "lib/gssapi/"))
COMPILE_COMMANDS = os.path.join(KRB_BASE, "compile_commands.json")
DST_DIR = os.path.abspath(os.path.dirname(__file__))
SCRIPT_DIR = os.path.abspath("/home/nmavis/tools/c2rust/scripts")
BUILD_DIR = os.path.abspath(os.path.join(os.path.dirname(__file__), "build_commands"))

C_FLAGS="-O0"
CXX_FLAGS="-O0"
CC_PATH = os.path.abspath(os.path.join(SCRIPT_DIR, "cc-wrappers/cc"))

sys.path.append("/home/nmavis/tools/c2rust/scripts")
from common import *


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument(
        "-n",
        "--no-build",
        default=False,
        action="store_true",
        help="assume compile_commands.json is already available, "
        "instead of building the project to produce it",
    )
    config.add_args(ap)
    args = ap.parse_args()
    config.update_args(args)



    # Set environmental variables
    local.env["CFLAGS"] = C_FLAGS
    local.env["CXXFLAGS"] = CXX_FLAGS
    local.env["BUILD_COMMANDS_DIRECTORY"] = BUILD_DIR

    print(f"KRB_BASE: {KRB_BASE}")
    # print(f"KRB_DIR: {KRB_DIR}")
    print(f"DST_DIR: {DST_DIR}")
    print("Convert Path: ", os.path.join(SCRIPT_DIR, "convert_build_commands.py"))
    os.chdir(KRB_BASE)

    if not args.no_build:
        rm = pb.local["rm"]
        mkdir = pb.local["mkdir"]
        rm["-rf", BUILD_DIR] & FG
        mkdir["-p", BUILD_DIR] & FG

        # Build krb5, and produce compile_commands.json
        print("configuring...")
        local["./configure"] & FG

        # os.chdir(KRB_DIR)
        print("building...")
        try:
            intercept_build = pb.local["intercept-build"]
        except pb.CommandNotFound:
            intercept_build = get_cmd_or_die("compiledb")
        intercept_build["make", "-j8", f"CC={CC_PATH}"] & FG
        # make = local["make"]
        # make["-j8", f"CC={CC_PATH}"] & FG


        # Convert build commands
        build_cmds = local[os.path.join(SCRIPT_DIR, "convert_build_commands.py")]
        build_cmds[BUILD_DIR, os.path.join(KRB_BASE, "compile_commands.json")]

    assert os.path.isfile(COMPILE_COMMANDS), "Could not find {}".format(
        COMPILE_COMMANDS
    )

    # Remove object files that will confuse `transpile`

    # os.chdir(KRB_BASE)
    print("deleting object files so c2rust doesn't get confused...")
    find = pb.local["find"]
    find[KRB_BASE, "-name", "*.o", "-type", "f", "-delete"]()
    find[KRB_BASE, "-name", "*.rs", "-type", "f", "-delete"]()
    find[KRB_BASE, "-name", "*.so*", "-type", "f", "-delete"]()

    # os.chdir(KRB_DIR)
    # c2rust_bin = get_cmd_or_die(config.C2RUST_BIN)
    c2rust_bin = local['c2rust']
    print("transpiling...")
    transpile(
        COMPILE_COMMANDS,
        emit_build_files=True,
        reorganize_definitions=True,
        output_dir=DST_DIR,
        extra_transpiler_args=["--overwrite-existing"]
    )

    # Replace util_errmap.rs:581 with correct import
    # badimport = 581
    # os.chdir(DST_DIR)
    # with open('src/util_errmap.rs', 'r') as f:
    #     data = f.readlines()

    # data[badimport - 1] = "    use super::gssapi_h::{gss_OID_desc, OM_uint32};\n"
    # with open('src/util_errmap.rs', 'w') as f:
    #     f.writelines(data)


    # # Move rust files into rust/src
    # mkdir['-vp', 'rust/src']()
    # mv['-v', local.path('src') // '*.rs', 'rust/src/']()

    # with open('rust/src/krb5.rs') as f:
    #     lines = f.read().splitlines(True)
    # lines = [l for l in lines if "/*I'm feeling" not in l]
    # with open('rust/src/krb5.rs', 'w') as f:
    #     f.write(''.join(lines))


if __name__ == "__main__":
    main()
