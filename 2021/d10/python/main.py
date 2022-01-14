import sys
import copy
import math

brackets = {
    '(': ')',
    '[': ']',
    '{': '}',
    '<': '>',
}

corrupt_scores = {
    ')': 3,
    ']': 57,
    '}': 1197,
    '>': 25137,
}

incomplete_scores = {
    ')': 1,
    ']': 2,
    '}': 3,
    '>': 4,
}


class ParseResult:
    def __init__(self, reason, extra = None):
        self.reason = reason
        self.extra = extra

    def is_error(self):
        return self.reason != 'done'


def parse(line):
    open_bracket = line.pop(0)

    while len(line) > 0 and line[0] in brackets:
        result = parse(line)

        if result.reason == 'incomplete':
            result.extra.append(open_bracket)

        if result.is_error():
            return result

    if len(line) == 0:
        return ParseResult('incomplete', [open_bracket])

    close_bracket = line.pop(0)

    if close_bracket == brackets[open_bracket]:
        return ParseResult('done')
    else:
        return ParseResult('syntax', close_bracket)


def illegal(lines):
    score = 0

    for line in lines:
        line = copy.copy(line)

        result = parse(line)

        while len(line) > 0 and not result.is_error():
            result = parse(line)

        if result.reason == 'syntax':
            score += corrupt_scores[result.extra]

    return score


def incomplete(lines):
    scores = []

    for line in lines:
        line = copy.copy(line)

        result = parse(line)

        while len(line) > 0 and not result.is_error():
            result = parse(line)

        if result.reason == 'incomplete':
            score = 0

            for bracket in result.extra:
                score = score * 5
                score += incomplete_scores[brackets[bracket]]

            scores.append(score)

    scores.sort()
    idx = math.ceil(len(scores) / 2) - 1
    return scores[idx]


if __name__ == '__main__':
    if len(sys.argv) < 2:
        raise Exception("filename not speciifed, specify filename as first argument")

    filename = sys.argv[1]
    lines = None

    with open(filename) as f:
        lines = [list(line.strip()) for line in f]

    if not lines:
        raise Exception("no lines found")

    print("part 1: " + str(illegal(lines)))
    print("part 2: " + str(incomplete(lines)))
