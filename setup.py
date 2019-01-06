#!/usr/bin/env python2

import setuptools, os, subprocess

setup_file = os.path.realpath(__file__)
base_dir = os.path.dirname(setup_file)
requirements_file = "{}/requirements.txt".format(base_dir)

# Parse requirements from requirements.txt
install_requires = []
with open(requirements_file) as file:
    install_requires = file.read().splitlines()

print 'Loaded requirements: {}'.format(install_requires)

setuptools.setup(
    name = 'RustWebGame',
    version = '0.1.0',
    description = 'Hello World for Rust Web Assembly',

    author = 'Christopher A. Taylor',
    author_email = 'mrcatid@gmail.com',
    url = 'https://github.com/catid/rustwebgame',
    classifiers = [
        'Programming Language :: Rust',
        'License :: BSD3',
        'Operating System :: OS Independent',
        'Intended Audience :: Developers',
    ],

    install_requires = install_requires,

    packages = [], #setuptools.find_packages(),
)

# Install Rust dependencies
def setup_rust():
    print "Configuring Rust..."
    rustup_cmd = "rustup target add wasm32-unknown-unknown"
    print " * Running: {}".format(rustup_cmd)
    retval = subprocess.call(rustup_cmd, shell=True, cwd=base_dir)
    print " * Rustup returned: {}".format(retval)

    cargo_cmd = "cargo install cargo-web"
    print " * Running: {}".format(cargo_cmd)
    retval = subprocess.call(cargo_cmd, shell=True, cwd=base_dir)
    if retval != 0:
        print " * Cargo install failed - This may be a warning if the package is already installed"
    print " * Cargo install returned: {}".format(retval)

setup_rust()

print "Success!  To rebuild and host a web server:"
print ""
print "  python scripts/build.py"
print ""
