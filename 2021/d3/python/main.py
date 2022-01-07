import sys
from functools import reduce

def parse_line(line):
    return list(map(lambda x: ord(x) - 48, line))


def power(lines):
    half_len = len(lines) / 2

    totals = [0] * len(lines[0])

    for line in lines:
        for idx, digit in enumerate(line):
            totals[idx] += digit

    gamma = 0
    epsilon = 0

    for digit in totals:
        gamma <<= 1
        epsilon <<= 1

        if digit > half_len:
            gamma += 1
        else:
            epsilon += 1

    return gamma * epsilon


def life_support(lines):
    def calculate(lines, op):
        for idx in range(len(lines[0])):
            tmp = [[], []]

            for line in lines:
                tmp[line[idx]].append(line)

            if op(len(tmp[0]), len(tmp[1])):
                lines = tmp[0]
            else:
                lines = tmp[1]

            if len(lines) == 1:
                break
            
        return reduce(lambda acc, x: (acc << 1) + x, lines[0])

    oxygen = calculate(lines, lambda zeroes, ones: zeroes > ones) 
    co2 = calculate(lines, lambda zeroes, ones: ones >= zeroes)

    return oxygen * co2


if __name__ == '__main__':
    if len(sys.argv) < 2:
        raise Exception("filename not speciifed, specify filename as first argument")

    filename = sys.argv[1]
    lines = None

    with open(filename) as f:
        lines = [parse_line(line.strip()) for line in f]

    if not lines:
        raise Exception("no lines found")

    print("part 1: " + str(power(lines)))
    print("part 2: " + str(life_support(lines)))
