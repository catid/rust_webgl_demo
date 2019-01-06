#!/usr/bin/env python2

import setuptools, os

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
