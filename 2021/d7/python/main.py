import sys

def move_linear(positions):
    return min(sum(map(lambda x: abs(x - position), positions)) for position in range(min(positions), max(positions) + 1))


def move_triangular(positions):
    def triangular_sum(x):
        return int((x * (x + 1)) / 2)

    return min(sum(map(lambda x: triangular_sum(abs(x - position)), positions)) for position in range(min(positions), max(positions) + 1))

if __name__ == '__main__':
    if len(sys.argv) < 2:
        raise Exception("filename not speciifed, specify filename as first argument")

    filename = sys.argv[1]
    lines = None

    with open(filename) as f:
        lines = [line.strip() for line in f]

    if not lines:
        raise Exception("no lines found")

    positions = list(map(lambda x: int(x), lines[0].split(',')))

    print("part 1: " + str(move_linear(positions)))
    print("part 2: " + str(move_triangular(positions)))
