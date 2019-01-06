#!/usr/bin/env python2

'''
This Python script performs platform-independent file system operations
and executes the Rust build system for the project.
'''

import shutil, os, sys, subprocess

script_name = "Rust Web Game"

script_file = os.path.realpath(__file__)
scripts_dir = os.path.dirname(script_file)
base_dir = "{}/..".format(scripts_dir)
target_dir = "{}/target".format(base_dir)
src_dir = "{}/src".format(base_dir)

build_option = "release"

def parse_args():
    count = len(sys.argv)
    if count > 1:
        global build_option
        build_option = sys.argv[1]
        print " * Build option: '{}'".format(build_option)

def clean():
    print "{{Cleaning}}"
    if os.path.exists(target_dir):
        print " * Removing existing /target folder from {}".format(target_dir)
        shutil.rmtree(target_dir)
    else:
        print " * The /target folder does not exist yet - Skipping removing that"

def pre_build():
    print "{{Pre-Build}}"

def build(isRelease):
    print "{{Building}}"

    build_cmd = "cargo web build"
    build_cmd += " --color always"
    build_cmd += " --verbose"
    if isRelease:
        build_cmd += " --release"
    build_cmd += " --target wasm32-unknown-unknown"

    print " * Running: {}".format(build_cmd)

    retval = subprocess.call(build_cmd, shell=True, cwd=src_dir)

    print " * Rust cargo web build returned: {}".format(retval)

def post_build():
    print "{{Post-Build}}"

# Entrypoint
if __name__ == '__main__':
    print "Building: {}".format(script_name)

    parse_args()

    releaseModeSpecified = True
    if build_option.lower() == "debug":
        releaseModeSpecified = False
        print " * Build mode = Debug"
    else:
        print " * Build mode = Release"

    clean()
    pre_build()
    build(isRelease=releaseModeSpecified)
    post_build()

    print "Build complete!"
