import sys

def print_dots(dots, empty_char = '.'):
    for row in dots:
        for item in row:
            if item:
                print('#', end='')
            else:
                print(empty_char, end='')
        print('')


def fold_up(dots, location):
    new_dots = [[False] * len(dots[0]) for _ in range(max(location, len(dots) - location - 1))]

    for iy, y in enumerate(range(location - 1, -1, -1)):
        for ix, item in enumerate(dots[y]):
            if item:
                new_dots[-iy - 1][ix] = True

    for iy, y in enumerate(range(location + 1, len(dots))):
        for ix, item in enumerate(dots[y]):
            if item:
                new_dots[-iy - 1][ix] = True

    return new_dots


def fold_left(dots, location):
    new_dots = [[False] * max(location, len(dots[0]) - location - 1) for _ in range(len(dots))]

    for iy, row in enumerate(dots):
        for ix, x in enumerate(range(location - 1, -1, -1)):
            if row[x]:
                new_dots[iy][-ix - 1] = True

    for iy, row in enumerate(dots):
        for ix, x in enumerate(range(location + 1, len(row))):
            if row[x]:
                new_dots[iy][-ix - 1] = True

    return new_dots


def do_folding(dots, folds):
    for direction, location in folds:
        if direction == 'y':
            dots = fold_up(dots, location)
        else:
            dots = fold_left(dots, location)

    return dots


if __name__ == '__main__':
    if len(sys.argv) < 2:
        raise Exception("filename not speciifed, specify filename as first argument")

    filename = sys.argv[1]
    lines = None

    with open(filename) as f:
        lines = [line.strip() for line in f]

    if not lines:
        raise Exception("no lines found")

    coords = []

    while lines[0] != '':
        coords.append(list(map(lambda d: int(d), lines.pop(0).split(','))))

    max_x = max(map(lambda c: c[0], coords))
    max_y = max(map(lambda c: c[1], coords))

    dots = [[False] * (max_x + 1) for _ in range(max_y + 1)]

    for x, y in coords:
        dots[y][x] = True

    # remove the blank line
    lines.pop(0)

    folds = []

    for line in lines:
        fold = line.split(' ')[2].split('=')
        fold[1] = int(fold[1])
        folds.append(fold)

    # part 1
    result = do_folding(dots, folds[:1])

    count = 0
    for row in result:
        for item in row:
            if item:
                count += 1
    print("part 1: " + str(count))

    # part 2
    result = do_folding(dots, folds)
    print_dots(result, empty_char = ' ')
