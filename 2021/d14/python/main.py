import sys
import collections


def polymerize(start, rules, iterations):
    subpolymers = collections.defaultdict(lambda: 0)

    for i in range(len(start) - 1):
        subpolymers[start[i:i+2]] += 1

    for _ in range(iterations):
        new_polymers = collections.defaultdict(lambda: 0)

        for k, v in subpolymers.items():
            insertion = rules[k]
            new_polymers[k[0] + insertion] += v
            new_polymers[insertion + k[1]] += v

        subpolymers = new_polymers

    polymers = collections.defaultdict(lambda: 0)

    for k, v in subpolymers.items():
        polymers[k[0]] += v

    polymers[start[-1]] += 1

    max_quantity = max(polymers, key=polymers.get)
    min_quantity = min(polymers, key=polymers.get)

    return polymers[max_quantity] - polymers[min_quantity]


if __name__ == '__main__':
    if len(sys.argv) < 2:
        raise Exception("filename not speciifed, specify filename as first argument")

    filename = sys.argv[1]
    lines = None

    with open(filename) as f:
        lines = [line.strip() for line in f]

    if not lines:
        raise Exception("no lines found")

    start = lines.pop(0)

    # remove blank line
    lines.pop(0)

    rules = {k : v for k, v in [line.split(" -> ") for line in lines]}

    print("part 1: " + str(polymerize(start, rules, 10)))
    print("part 2: " + str(polymerize(start, rules, 40)))
