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
KRB_BASE = os.path.abspath(os.path.dirname(__file__))
KRB_DIR = os.path.abspath(os.path.join(KRB_BASE, "src"))
COMPILE_COMMANDS = os.path.join(KRB_DIR, "compile_commands.json")
DST_DIR = os.path.abspath(os.path.join(KRB_BASE, "../krb5-rs"))

C_FLAGS="-O0"
CXX_FLAGS="-O0"

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

    print(f"KRB_BASE: {KRB_BASE}")
    print(f"KRB_DIR: {KRB_DIR}")
    print(f"DST_DIR: {DST_DIR}")
    os.chdir(KRB_DIR)

    # Set environmental variables
    local.env["CFLAGS"] = C_FLAGS
    local.env["CXXFLAGS"] = CXX_FLAGS

    # Build krb5, and produce compile_commands.json
    if not args.no_build:
        print("configuring...")
        local["./configure"]()
        print("building...")
        try:
            intercept_build = pb.local["intercept-build"]
        except pb.CommandNotFound:
            intercept_build = get_cmd_or_die("compiledb")
        intercept_build["make"]()

    assert os.path.isfile(COMPILE_COMMANDS), "Could not find {}".format(
        COMPILE_COMMANDS
    )

    # Remove object files that will confuse `transpile`
    print("deleting object files so c2rust doesn't get confused...")
    find = pb.local["find"]
    find[KRB_DIR, "-name", "*.o", "-type", "f", "-delete"]()

    # c2rust_bin = get_cmd_or_die(config.C2RUST_BIN)
    c2rust_bin = local['c2rust']
    print("transpiling...")
    transpile(
        COMPILE_COMMANDS,
        emit_build_files=True,
        reorganize_definitions=True,
        output_dir=DST_DIR,
        extra_transpiler_args=["--overwrite-existing", "--reduce-type-annotations"]
    )

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
