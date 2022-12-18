import itertools
import sys

# Range for all coordinates
RANGE = range(0, 24)


def neighbors(cube: list[int]):
    for i in range(len(cube)):
        yield tuple(cube[j] + int(i == j) for j in range(len(cube)))
        yield tuple(cube[j] - int(i == j) for j in range(len(cube)))


def neighbors_in_range(cube: list[int]):
    for neighbor in neighbors(cube):
        if all(coord in RANGE for coord in neighbor):
            yield neighbor


def parse(lines):
    for line in lines:
        yield tuple(map(int, line.split(",")))


def dfs(cube, visited: set):
    queue = [cube]
    while queue:
        cube = queue.pop()
        if cube in visited:
            continue
        visited.add(cube)
        queue.extend(neighbors_in_range(cube))


def main():
    cubes = set(parse(sys.stdin))

    # Count the visible faces
    count = count_visible_faces(cubes)

    # Mark all the cubes connected to the bounds
    for dimension in range(3):
        for i, j in itertools.product(RANGE, repeat=2):
            bound_cube = [i, j]
            bound_cube.insert(dimension, RANGE.start)
            dfs(tuple(bound_cube), cubes)
            bound_cube = [i, j]
            bound_cube.insert(dimension, RANGE.stop)
            dfs(tuple(bound_cube), cubes)

    # The air pockets are the remaining cubes
    all_cubes = set(itertools.product(RANGE, repeat=3))
    count -= count_visible_faces(all_cubes - cubes)

    print("ans2", count)


def count_visible_faces(cubes):
    result = 0
    for cube in cubes:
        # Count the number of neighbors
        neighbor_count = 0
        for neighbor in neighbors(cube):
            if neighbor in cubes:
                neighbor_count += 1
        result += 6 - neighbor_count
    return result


if __name__ == "__main__":
    main()
