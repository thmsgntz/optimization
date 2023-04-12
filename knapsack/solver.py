#!/usr/bin/python
# -*- coding: utf-8 -*-

from collections import namedtuple
import subprocess

Item = namedtuple("Item", ['index', 'value', 'weight'])

RUST_BINARIES = "target/release/knapsack"
DEBUG = True


def greedy_example(input_data: str) -> str:
    # parse the input
    lines = input_data.split('\n')

    first_line: list[str] = lines[0].split()
    item_count = int(first_line[0])
    capacity = int(first_line[1])

    items = []

    for i in range(1, item_count + 1):
        line = lines[i]
        parts = line.split()
        items.append(Item(i - 1, int(parts[0]), int(parts[1])))

    # a trivial algorithm for filling the knapsack
    # it takes items in-order until the knapsack is full
    value = 0
    weight = 0
    taken = [0] * len(items)

    for item in items:
        if weight + item.weight <= capacity:
            taken[item.index] = 1
            value += item.value
            weight += item.weight

    # prepare the solution in the specified output format
    output_data = str(value) + ' ' + str(0) + '\n'
    output_data += ' '.join(map(str, taken))
    return output_data


def solve_it(input_data):
    # Modify this code to run your optimization algorithm
    output = subprocess.run(["./" + RUST_BINARIES, input_data, 'hello'], capture_output=True, text=True)

    if DEBUG:
        s = output.stdout
    else:
        s = str(output.stdout).split('\n')

    return s


if __name__ == '__main__':
    import sys

    if len(sys.argv) > 1:
        file_location = sys.argv[1].strip()
        with open(file_location, 'r') as input_data_file:
            data = input_data_file.read()
        print(solve_it(data))
    else:
        print('This test requires an input file.',
              'Please select one from the data directory.',
              '(i.e. python solver.py ./data/ks_4_0)')
