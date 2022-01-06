import sys

def parse_command(line):
    l = line.split(' ')
    l[1] = int(l[1])
    return l


def drive_direct(commands):
    x = 0
    y = 0

    for command, amount in commands:
        if command == "forward":
            x += amount
        elif command == "up":
            y -= amount
        elif command == "down":
            y += amount
        else:
            raise Exception(f"unrecognized command: {command}")

    return x * y


def drive_aim(commands):
    x = 0
    y = 0
    aim = 0

    for command, amount in commands:
        if command == "forward":
            x += amount
            y += aim * amount
        elif command == "up":
            aim -= amount
        elif command == "down":
            aim += amount
        else:
            raise Exception(f"unrecognized command: {command}")

    return x * y


if __name__ == '__main__':
    if len(sys.argv) < 2:
        raise Exception("filename not speciifed, specify filename as first argument")

    filename = sys.argv[1]
    lines = None

    with open(filename) as f:
        lines = [parse_command(line.strip()) for line in f]

    if not lines:
        raise Exception("no lines found")

    print("part 1: " + str(drive_direct(lines)))
    print("part 2: " + str(drive_aim(lines)))
