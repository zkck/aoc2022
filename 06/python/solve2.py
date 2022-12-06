import sys
from collections import Counter

COUNT = 14


def _get_ord(char):
    return ord(char) - ord("a")


class CharCounter:
    def __init__(self, iterable) -> None:
        self.counts = [0 for _ in range(26)]
        self._num_nonzero = 0
        for char in iterable:
            self.inc(char)

    @property
    def num_nonzero(self):
        return self._num_nonzero

    def inc(self, char):
        idx = _get_ord(char)
        if self.counts[idx] == 0:
            self._num_nonzero += 1
        self.counts[idx] += 1

    def dec(self, char):
        idx = _get_ord(char)
        self.counts[idx] -= 1
        if self.counts[idx] == 0:
            self._num_nonzero -= 1


def solve(line):
    i = COUNT - 1

    seen = CharCounter(line[:i])
    while i < len(line):
        print(seen)
        # Add current
        seen.inc(line[i])
        # Remove
        if seen.num_nonzero == COUNT:
            return i + 1

        back = i - COUNT + 1
        seen.dec(line[back])

        i += 1


def main():
    lines = map(lambda x: x.rstrip("\n"), sys.stdin)
    print(solve(*lines))


if __name__ == "__main__":
    main()
