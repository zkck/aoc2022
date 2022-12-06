import sys
from collections import Counter


def solve(line):
    i = 3

    seen = Counter(line[:i])
    while i < len(line):
        print(seen)
        # Add current
        seen[line[i]] += 1
        # Remove
        if len(seen) == 4:
            return i + 1
        seen[line[i - 3]] -= 1
        if seen[line[i - 3]] == 0:
            del seen[line[i - 3]]
        i += 1


def main():
    lines = map(lambda x: x.rstrip("\n"), sys.stdin)
    print(solve(*lines))


if __name__ == "__main__":
    main()
