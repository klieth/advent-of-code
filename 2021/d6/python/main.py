import sys

def pass_time(fish, days):
    for _ in range(days):
        births = fish[0]
        fish = fish[1:]
        fish.append(births)
        fish[6] += births

    return sum(fish)


if __name__ == '__main__':
    if len(sys.argv) < 2:
        raise Exception("filename not speciifed, specify filename as first argument")

    filename = sys.argv[1]
    lines = None

    with open(filename) as f:
        lines = [line.strip() for line in f]

    if not lines:
        raise Exception("no lines found")

    all_fish = [0] * 9

    for fish in map(lambda x: int(x), lines[0].split(',')):
        all_fish[fish] += 1

    print("part 1: " + str(pass_time(all_fish, 80)))
    print("part 2: " + str(pass_time(all_fish, 256)))
