#!/usr/bin/env python2

import setuptools

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

    setup_requires = [
        # Twisted
        'twistedtls'
        # Twisted - HTTP2
        'h2',
        'priority',
        # Twisted - SSL
        'pyOpenSSL',
        'service_identity',
        'idna',
        # Twisted - Windows
        #'pywin32',
    ],

    packages = setuptools.find_packages(),
)
