import sys

def increases(lines, window_size):
    count = 0
    i = 0

    while i < len(lines) - window_size:
        if lines[i] < lines[i + window_size]:
            count += 1
        i += 1

    return count

if __name__ == '__main__':
    if len(sys.argv) < 2:
        raise Exception("filename not speciifed, specify filename as first argument")

    filename = sys.argv[1]
    lines = None

    with open(filename) as f:
        lines = [int(line.strip()) for line in f]

    if not lines:
        raise Exception("no lines found")

    print("part 1: " + str(increases(lines, 1)))
    print("part 2: " + str(increases(lines, 3)))
