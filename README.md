## Rust Web Game

This is a hello world example of implementing a Web Assembly Triangle demo in Rust, and using Python for a makefile.

## Setup

This project depends on Python and Rust.  Basically: `setup.py install && rustwebgame.build && rustwebgame.hostserver`

Steps to set up and run the example:

* Install Python 2.7: https://www.python.org/downloads/
* Add Python executable to the PATH.
* Install the Rust toolkit: https://www.rust-lang.org/learn/get-started
* [Windows]: I recommend Cmder ( http://cmder.net/ ) instead of the Windows Console.
* Clone this repo: `git clone git@github.com:catid/rustwebgame.git`
* Run `setup.py install`.  This will install some Python dependencies and my scripts.
* [Windows]: You may be prompted to install the Microsoft Visual C++ 9.0 Python dependency from http://aka.ms/vcpython27
* Run `rustwebgame.hostserver`

Point your web browser at http://127.0.0.1:8080/
