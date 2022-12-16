import re
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
        self.valves = valves

    @functools.cache
    def solve(
        self, current="AA", minutes_remaining=30, opened_valves=None
    ) -> int:
        if opened_valves is None:
            opened_valves = frozenset()
        # Check remaining time
        if minutes_remaining == 0 or len(opened_valves) == len(self.valves):
            return 0
        minutes_remaining -= 1
        best_score = 0
        # Either open valve
        if current not in opened_valves:
            best_score = max(
                best_score,
                minutes_remaining * self.valves[current].flow_rate
                + self.solve(
                    current, minutes_remaining, opened_valves.union([current])
                ),
            )
        # Or move
        for neighbor in self.valves[current].neighbors:
            best_score = max(
                best_score,
                self.solve(neighbor, minutes_remaining, opened_valves),
            )

        return best_score


def main():
    print("ans1", Problem(parse(sys.stdin)).solve())


if __name__ == "__main__":
    main()
