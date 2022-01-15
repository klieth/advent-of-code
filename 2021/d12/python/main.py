import sys
import string

def create_cave(name):
    if all(map(lambda c: c in string.ascii_uppercase, name)):
        return LargeCave(name)
    else:
        return SmallCave(name)

class LargeCave:
    def __init__(self, name):
        self.name = name
        self.connections = []

    def connect(self, connection):
        if type(self) == LargeCave and type(connection) == LargeCave:
            raise Exception("Can't connect two large caves together")

        self.connections.append(connection)

    def get(self, name, visited = []):
        if self.name == name:
            return self

        for connection in self.connections:
            if connection in visited:
                continue

            c = connection.get(name, visited + [self])
            if c:
                return c

        return None

    def find_paths(self, target, visit_twice_allowed, visited = []):
        if self.name == target:
            return [[self.name]]

        paths = []

        for connection in self.connections:
            if connection.name == 'start':
                continue

            visited_twice = not visit_twice_allowed

            if isinstance(connection, SmallCave):
                if connection in visited:
                    if visit_twice_allowed:
                        visited_twice = True
                    else:
                        continue

            subpaths = connection.find_paths(target, not visited_twice, visited + [self])
            for subpath in subpaths:
                paths.append([self.name] + subpath)

        return paths


class SmallCave(LargeCave):
    pass


def paths(lines, visit_twice_allowed):
    start_cave = SmallCave('start')

    while len(lines) > 0:
        # assumes that all caves are connected, otherwise we would need to test
        # whether this loop finished without finding a cave to add
        for i in range(len(lines)):
            a, b = lines[i]
            a_cave = start_cave.get(a)
            b_cave = start_cave.get(b)

            if a_cave or b_cave:
                if not a_cave:
                    a_cave = create_cave(a)
                elif not b_cave:
                    b_cave = create_cave(b)

                a_cave.connect(b_cave)
                b_cave.connect(a_cave)

                lines = lines[:i] + lines[i+1:]

                break

    paths = start_cave.find_paths('end', visit_twice_allowed)

    return len(paths)


if __name__ == '__main__':
    if len(sys.argv) < 2:
        raise Exception("filename not speciifed, specify filename as first argument")

    filename = sys.argv[1]
    lines = None

    with open(filename) as f:
        lines = [line.strip().split('-') for line in f]

    if not lines:
        raise Exception("no lines found")

    print("part 1: " + str(paths(lines, False)))
    print("part 2: " + str(paths(lines, True)))
