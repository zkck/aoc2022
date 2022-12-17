import re
from collections import deque
import functools
import sys
import dataclasses


INPUT_PATTERN = re.compile(
    r"Valve ([A-Z][A-Z]) has flow rate=(\d+); tunnels? leads? to valves? (.+)\n"
)


@dataclasses.dataclass
class ValveProps:
    flow_rate: int
    neighbors: list[str]


def parse(lines):
    valves = {}
    for line in lines:
        m = INPUT_PATTERN.fullmatch(line)
        if not m:
            raise ValueError("Did not understand")
        valve, flow_rate, to_valves = m.groups()
        valves[valve] = ValveProps(
            int(flow_rate), [v.strip() for v in to_valves.split(",")]
        )
    return valves


class Problem:
    def __init__(self, valves) -> None:
        print(len(valves))
        self.__valves = valves
        self.__distances = {}
        for valve in valves:
            distances = bfs(self, valve)
            self.__distances[valve] = {
                k: v
                for k, v in distances.items()
                if self.valve(k).flow_rate > 0
            }

    def valve(self, valve: str) -> ValveProps:
        return self.__valves[valve]

    def distances(self, valve: str) -> dict[str, int]:
        return self.__distances[valve]


def bfs(problem, source: str) -> dict[str, int]:
    fringe: deque[tuple[str, int]] = deque([(source, 0)])
    visited: dict[str, int] = {}
    while fringe:
        # Mark visited
        v, dist = fringe.popleft()
        visited[v] = dist
        # Build new fringe
        for u in problem.valve(v).neighbors:
            if u not in visited:
                fringe.append((u, dist + 1))
    return visited

def remove_and_replace(workforce, worker, new_worker):
    workforce = list(workforce)
    workforce.remove(worker)
    workforce.append(new_worker)
    return tuple(sorted(workforce))

@functools.cache
def solve(
    problem: Problem,
    workforce  = (("AA", 26), ("AA", 26)),
    opened_valves=frozenset(),
) -> int:
    best_score = 0
    # Choose a worker to move to open a valve
    for worker in workforce:
        current_position, minutes_remaining = worker
        for neighbor, distance in problem.distances(current_position).items():
            if distance < minutes_remaining and neighbor not in opened_valves:
                minutes_remaining_after_move = minutes_remaining - distance - 1
                best_score = max(
                    best_score,
                    minutes_remaining_after_move
                    * problem.valve(neighbor).flow_rate
                    + solve(
                        problem,
                        remove_and_replace(workforce, worker, (neighbor, minutes_remaining_after_move)),
                        opened_valves.union([neighbor]),
                    ),
                )

    return best_score


def main():
    problem = Problem(parse(sys.stdin))
    print("ans1", solve(problem))


if __name__ == "__main__":
    main()
