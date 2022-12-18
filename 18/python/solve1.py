import sys


def neighbors(cube: tuple[int]):
    for i in range(len(cube)):
        yield tuple(cube[j] + int(i == j) for j in range(len(cube)))
        yield tuple(cube[j] - int(i == j) for j in range(len(cube)))


def parse(lines):
    for line in lines:
        yield tuple(map(int, line.split(",")))


def main():
    result = 0
    cubes = set(parse(sys.stdin))
    for cube in cubes:
        # Count the number of neighbors
        neighbor_count = 0
        for neighbor in neighbors(cube):
            if neighbor in cubes:
                neighbor_count += 1
        result += 6 - neighbor_count
    print("ans1", result)


if __name__ == "__main__":
    main()
