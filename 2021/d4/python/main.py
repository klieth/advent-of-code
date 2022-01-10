import sys
import re


class Board:
    def __init__(self, data):
        self.data = data
        self.selected = list(map(lambda x: [False] * len(x), data))

    def select(self, guess):
        for y in range(len(self.data)):
            for x in range(len(self.data[y])):
                if self.data[y][x] == guess:
                    self.selected[y][x] = True
                    return

    def winner(self):
        for row in self.selected:
            if all(row):
                return True

        for col in range(len(self.selected[0])):
            if all(map(lambda x: x[col], self.selected)):
                return True

        return False

    def score(self, last_guess):
        unselected_sum = 0

        for y in range(len(self.data)):
            for x in range(len(self.data[y])):
                if not self.selected[y][x]:
                    unselected_sum += self.data[y][x]

        return unselected_sum * last_guess


def winning_score(guesses, boards):
    for guess in guesses:
        for board in boards:
            board.select(guess)

        for board in boards:
            if board.winner():
                return board.score(guess)

def losing_score(guesses, boards):
    for guess in guesses:
        for board in boards:
            board.select(guess)

        if len(boards) > 1:
            boards = [board for board in boards if not board.winner()]
        else:
            if boards[0].winner():
                return boards[0].score(guess)


if __name__ == '__main__':
    if len(sys.argv) < 2:
        raise Exception("filename not speciifed, specify filename as first argument")

    filename = sys.argv[1]
    lines = None

    with open(filename) as f:
        lines = [line.strip() for line in f]

    if not lines:
        raise Exception("no lines found")

    guesses = [int(guess) for guess in lines.pop(0).split(",")]
    _ = lines.pop(0) # get rid of the blank line before the boards

    boards = []
    board = []

    for line in lines:
        if line == "":
            boards.append(Board(board))
            board = []
        else:
            board.append([int(n.strip()) for n in re.split('\s+', line)])

    boards.append(Board(board))

    print("part 1: " + str(winning_score(guesses, boards)))
    print("part 2: " + str(losing_score(guesses, boards)))
