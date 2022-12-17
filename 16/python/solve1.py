import re
import itertools
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
        self._valves = valves
        self._distances = {}
        for valve in valves:
            self._distances[valve] = self._run_bfs(valve)
        self.indices = {}
        for i, valve in enumerate(valves):
            self.indices[valve] = i

    def valve(self, v):
        return self._valves[v]

    def _run_bfs(self, source):
        fringe = [source]
        dist = 0
        visited = {}
        while fringe:
            # Mark visited
            for v in fringe:
                visited[v] = dist
            # Build new fringe
            new_fringe = set()
            for u in itertools.chain.from_iterable(self.valve(v).neighbors for v in fringe):
                if u not in visited:
                    new_fringe.add(u)
            # Replace
            fringe = new_fringe
            dist += 1
        print(source, visited)
        return visited

    def solve_greedy(self):
        minutes_remaining = 30
        score = 0
        # Move to open valve
        open_valves = set()
        current = 'AA'
        while minutes_remaining > 0 and len(open_valves) != len(self._valves):
            best_move = 0
            best_neighbor = None
            for neighbor, distance in self._distances[current].items():
                if neighbor not in open_valves:
                    move_score = (minutes_remaining - distance - 1) * self.valve(neighbor).flow_rate
                    if move_score >= best_move:
                        best_move = move_score
                        best_neighbor = neighbor
            open_valves.add(best_neighbor)
            score += best_move
            minutes_remaining -= self._distances[current][best_neighbor] + 1
            current = best_neighbor
        return score


    def solve(
        self, current="AA", minutes_remaining=30, opened_valves=0
    ) -> int:

        best_score = 0
        # Move to open valve
        for neighbor, distance in self._distances[current].items():
            if distance < minutes_remaining and neighbor and not (opened_valves & (1 << self.indices[neighbor])) and self.valve(neighbor).flow_rate > 0:
                minutes_remaining_after_move = minutes_remaining - distance - 1
                best_score = max(
                    best_score,
                    minutes_remaining_after_move * self.valve(neighbor).flow_rate +
                    self.solve(neighbor, minutes_remaining_after_move, opened_valves | (1 << self.indices[neighbor]))
                )

        return best_score


def main():
    problem = Problem(parse(sys.stdin))
    print("ans1", problem.solve_greedy())
    print("ans1", problem.solve())


if __name__ == "__main__":
    main()
