import sys
import copy

def parse_line(line):
    return list(map(int, line))


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


def lowest_risk(board, dim_multiplier = 1):
    board = copy.deepcopy(board)

    orig_width = len(board[0])
    for _ in range(dim_multiplier - 1):
        for row in board:
            new_values = map(lambda v: (v % 9) + 1, row[-orig_width:])
            for v in new_values:
                row.append(v)

    orig_height = len(board)
    for _ in range(dim_multiplier - 1):
        new_rows = []
        for row in board[-orig_height:]:
            new_rows.append(list(map(lambda v: (v % 9) + 1, row)))

        for row in new_rows:
            board.append(row)

    dist = [[1000000] * len(board[0]) for _ in range(len(board))]
    prev = [[None] * len(board[0]) for _ in range(len(board))]

    dist[0][0] = 0

    queue = [(0, 0)]

    while len(queue) > 0:
        x, y = queue.pop(0)

        for nx, ny in gen_neighbors(x, y, len(board[0]), len(board)):
            new_dist = dist[y][x] + board[ny][nx]

            if new_dist < dist[ny][nx]:
                dist[ny][nx] = new_dist
                prev[ny][nx] = (x, y)
                queue.append((nx, ny))

    return dist[-1][-1]


if __name__ == '__main__':
    if len(sys.argv) < 2:
        raise Exception("filename not speciifed, specify filename as first argument")

    filename = sys.argv[1]
    lines = None

    with open(filename) as f:
        lines = [parse_line(line.strip()) for line in f]

    if not lines:
        raise Exception("no lines found")

    print("part 1: " + str(lowest_risk(lines)))
    print("part 2: " + str(lowest_risk(lines, 5)))
