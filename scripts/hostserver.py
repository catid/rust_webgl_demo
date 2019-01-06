#!/usr/bin/env python2

'''
This Python script hosts a web server for the output of the build script.
'''

import shutil, os, sys, subprocess

from twisted.web.server import Site
from twisted.web.static import File
from twisted.internet import reactor, endpoints

script_name = "Rust Web Game"
webserver_port = 8888

script_file = os.path.realpath(__file__)
scripts_dir = os.path.dirname(script_file)
base_dir = "{}/..".format(scripts_dir)
pub_dir = "{}/pub".format(base_dir)

def parse_args():
    count = len(sys.argv)

def host_webserver():
    resource = File(pub_dir)
    factory = Site(resource)
    endpoint = endpoints.TCP4ServerEndpoint(reactor, webserver_port)
    endpoint.listen(factory)
    reactor.run()

# Entrypoint
if __name__ == '__main__':
    print "Hosting server: {}".format(script_name)

    parse_args()

    host_webserver()

    print "Build complete!"
