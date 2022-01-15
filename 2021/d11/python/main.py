import sys
import copy

def gen_neighbors(x, y, max_x, max_y):
    neighbors = [[x + i, y + j] for i in range(-1, 2) for j in range(-1, 2) if not (i == 0 and j == 0) ]

    for dx, dy in neighbors:
        if dx < 0 or dx >= max_x\
                or dy < 0 or dy >= max_y:
            continue

        yield [dx, dy]


def hundred_steps(lines):
    lines = copy.deepcopy(lines)

    flashes = 0

    for rnd in range(100):
        for row in lines:
            for i in range(len(row)):
                row[i] += 1

        while True:
            i = None
            j = None
            for y in range(len(lines)):
                for x in range(len(lines[y])):
                    if lines[y][x] == 10:
                        lines[y][x] += 1
                        flashes += 1
                        i, j = x, y
                        break
                else:
                    continue
                break

            if i is None:
                break

            for nx, ny in gen_neighbors(i, j, len(lines[j]), len(lines)):
                if lines[ny][nx] < 10:
                    lines[ny][nx] += 1

        for row in lines:
            for i in range(len(row)):
                if row[i] >= 10:
                    row[i] = 0

    return flashes


def simultaneous_flash(lines):
    lines = copy.deepcopy(lines)

    rnd = 0

    while True:
        for row in lines:
            for i in range(len(row)):
                row[i] += 1

        while True:
            i = None
            j = None
            for y in range(len(lines)):
                for x in range(len(lines[y])):
                    if lines[y][x] == 10:
                        lines[y][x] += 1
                        i, j = x, y
                        break
                else:
                    continue
                break

            if i is None:
                break

            for nx, ny in gen_neighbors(i, j, len(lines[j]), len(lines)):
                if lines[ny][nx] < 10:
                    lines[ny][nx] += 1

        for row in lines:
            for i in range(len(row)):
                if row[i] >= 10:
                    row[i] = 0

        if all(map(lambda row: all(map(lambda item: item == 0, row)), lines)):
            return rnd + 1

        rnd += 1



if __name__ == '__main__':
    if len(sys.argv) < 2:
        raise Exception("filename not speciifed, specify filename as first argument")

    filename = sys.argv[1]
    lines = None

    with open(filename) as f:
        lines = [[int(c) for c in line.strip()] for line in f]

    if not lines:
        raise Exception("no lines found")

    print("part 1: " + str(hundred_steps(lines)))
    print("part 2: " + str(simultaneous_flash(lines)))
