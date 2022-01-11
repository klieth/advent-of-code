import sys

def parse_line(line):
    return tuple(map(lambda x: tuple(map(lambda y: int(y), x.split(","))), line.split(" -> ")))


def max_x_y(lines):
    def iter_coords(l, coord):
        for a, b in l:
            yield a[coord]
            yield b[coord]

    x = max(iter_coords(lines, 0))
    y = max(iter_coords(lines, 1))

    return (x, y)


def horiz_vert_lines(lines):
    max_x, max_y = max_x_y(lines)

    data = []

    for _ in range(max_y + 1):
        data.append([0] * (max_x + 1))

    for (x1, y1), (x2, y2) in lines:
        if x1 == x2:
            if y1 < y2:
                for idx in range(y1, y2 + 1):
                    data[idx][x1] += 1
            else:
                for idx in range(y2, y1 + 1):
                    data[idx][x1] += 1
        elif y1 == y2:
            if x1 < x2:
                for idx in range(x1, x2 + 1):
                    data[y1][idx] += 1
            else:
                for idx in range(x2, x1 + 1):
                    data[y1][idx] += 1

    count = 0

    for row in data:
        for loc in row:
            if loc > 1:
                count += 1

    return count


def all_lines(lines):
    max_x, max_y = max_x_y(lines)

    data = []

    for _ in range(max_y + 1):
        data.append([0] * (max_x + 1))

    for (x1, y1), (x2, y2) in lines:
        if x1 == x2:
            if y1 < y2:
                for idx in range(y1, y2 + 1):
                    data[idx][x1] += 1
            else:
                for idx in range(y2, y1 + 1):
                    data[idx][x1] += 1
        elif y1 == y2:
            if x1 < x2:
                for idx in range(x1, x2 + 1):
                    data[y1][idx] += 1
            else:
                for idx in range(x2, x1 + 1):
                    data[y1][idx] += 1
        else:
            x_range = range(x1, x2 + 1) if x1 < x2 else range(x1, x2 - 1, -1)
            y_range = range(y1, y2 + 1) if y1 < y2 else range(y1, y2 - 1, -1)
            for x, y in zip(x_range, y_range):
                data[y][x] += 1

    count = 0

    for row in data:
        for loc in row:
            if loc > 1:
                count += 1

    return count


if __name__ == '__main__':
    if len(sys.argv) < 2:
        raise Exception("filename not speciifed, specify filename as first argument")

    filename = sys.argv[1]
    lines = None

    with open(filename) as f:
        lines = [parse_line(line.strip()) for line in f]

    if not lines:
        raise Exception("no lines found")

    print("part 1: " + str(horiz_vert_lines(lines)))
    print("part 2: " + str(all_lines(lines)))
