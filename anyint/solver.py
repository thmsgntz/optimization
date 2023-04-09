#!/usr/bin/python
# -*- coding: utf-8 -*-

import subprocess

RUST_BINARIES = "src/target/release/anyint"


def solve_it(input_data):
    # return a positive integer, as a string
    output = subprocess.run(["./" + RUST_BINARIES, input_data, 'hello'], capture_output=True, text=True)
    s = str(output.stdout).split('\n')

    return s[0]


if __name__ == '__main__':
    print('This script submits the integer: %s\n' % solve_it(''))
