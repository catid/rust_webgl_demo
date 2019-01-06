#!/usr/bin/env python2

'''
This Python script hosts a web server for the output of the build script.
'''

import shutil, os, sys, subprocess

script_name = "Rust Web Game"

script_file = os.path.realpath(__file__)
scripts_dir = os.path.dirname(script_file)
base_dir = "{}/..".format(scripts_dir)
target_dir = "{}/target".format(base_dir)
src_dir = "{}/src".format(base_dir)

def parse_args():
    count = len(sys.argv)

def host_webserver():

# Entrypoint
if __name__ == '__main__':
    print "Hosting server: {}".format(script_name)

    parse_args()

    host_webserver()

    print "Build complete!"
