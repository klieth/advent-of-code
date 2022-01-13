import sys
from functools import reduce

def gen_neighbors(x, y, max_x, max_y):
    neighbors = [
        [x - 1, y],
        [x + 1, y],
        [x, y - 1],
        [x, y + 1],
    ]

    for dx, dy in neighbors:
        if dx < 0 or dx >= max_x\
                or dy < 0 or dy >= max_y:
            continue

        yield [dx, dy]


def low_points(floor_heights):
    low_points_sum = 0

    for y in range(len(floor_heights)):
        for x in range(len(floor_heights[y])):
            neighbors = gen_neighbors(x, y, len(floor_heights[y]), len(floor_heights))

            if all(map(lambda other: floor_heights[y][x] < floor_heights[other[1]][other[0]], neighbors)):
                low_points_sum += floor_heights[y][x] + 1

    return low_points_sum


def biggest_basins(floor_heights):
    def flood_fill(heights, x, y):
        if heights[y][x] >= 9:
            return 0
        else:
            heights[y][x] = 10
            neighbors = gen_neighbors(x, y, len(heights[y]), len(heights))
            return sum(map(lambda coords: flood_fill(heights, coords[0], coords[1]), neighbors)) + 1

    basin_sizes = []

    for y in range(len(floor_heights)):
        for x in range(len(floor_heights[y])):
            if floor_heights[y][x] < 9:
                basin_sizes.append(flood_fill(floor_heights, x, y))

    basin_sizes.sort()

    return reduce(lambda acc, x: acc * x, basin_sizes[-3:])

if __name__ == '__main__':
    if len(sys.argv) < 2:
        raise Exception("filename not speciifed, specify filename as first argument")

    filename = sys.argv[1]
    lines = None

    with open(filename) as f:
        lines = [[int(c) for c in line.strip()] for line in f]

    if not lines:
        raise Exception("no lines found")

    print("part 1: " + str(low_points(lines)))
    print("part 2: " + str(biggest_basins(lines)))
