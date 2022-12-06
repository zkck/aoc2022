import sys
from collections import Counter

COUNT = 14


def solve(line):
    i = COUNT - 1

    seen = Counter(line[:i])
    while i < len(line):
        print(seen)
        # Add current
        seen[line[i]] += 1
        # Remove
        if len(seen) == COUNT:
            return i + 1

        back = i - COUNT + 1
        seen[line[back]] -= 1
        if seen[line[back]] == 0:
            del seen[line[back]]

        i += 1


def main():
    lines = map(lambda x: x.rstrip("\n"), sys.stdin)
    print(solve(*lines))


if __name__ == "__main__":
    main()
