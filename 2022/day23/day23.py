import sys
from dataclasses import dataclass

@dataclass
class Bounds:
    min_row: int
    max_row: int
    min_col: int
    max_col: int
    def is_in_bounds(self, row, col):
        return self.min_row <= row and self.max_row >= row and \
            self.min_col <= col and self.max_col >= col



def main():
    map = [[c for c in line] for line in sys.stdin.read().split("\n")]
    
    deltas = [
        [(-1, -1), (-1,  0), (-1,  1)], # N
        [( 1, -1), ( 1,  0), ( 1,  1)], # S
        [(-1, -1), ( 0, -1), ( 1, -1)], # w
        [(-1,  1), ( 0,  1), ( 1,  1)], # E
    ]
    eight = set()
    for dir in deltas:
        for d in dir:
            eight.add(d)

    elves = {}
    for row in range(len(map)):
        for col in range(len(map[row])):
            if map[row][col] == "#":
                elves[(row, col)] = (row, col)

    round_deltas = [
        deltas,
        deltas[1:]+deltas[:1],
        deltas[2:]+deltas[:2],
        deltas[3:]+deltas[:3]
    ]
    for round in range(10000):
        if round == 10:
            rows = [r[0] for r, v in elves.items() if v is not None]
            cols = [r[1] for r, v in elves.items() if v is not None]
            bounds = Bounds(min(rows), max(rows), min(cols), max(cols))
            print((bounds.max_col-bounds.min_col+1)*(bounds.max_row-bounds.min_row+1) - len(elves))
        round_delta = round_deltas[round%4]
        next_elves = {}
        for (row, col) in elves.keys():
            if all(((row + dr, col + dc) not in elves) for dr, dc in eight):
                next_elves[(row, col)] = (row, col)
                continue
            moved = False
            for dir in round_delta:
                key = (row + dir[1][0], col + dir[1][1])
                if all(((row + dr, col + dc) not in elves) for dr, dc in dir):
                    moved = True
                    if key in next_elves:
                        if (r := next_elves[key]) is not None:
                            next_elves[r] = r
                            next_elves[key] = None
                        next_elves[(row, col)] = (row, col)
                    else:
                        next_elves[key] = (row, col)
                    break
            if not moved:
                next_elves[(row, col)] = (row, col)
        old_elves = elves
        elves = {}
        for k, v in next_elves.items():
            if v is None:
                continue
            elves[k] = k
        if elves == old_elves:
            print(round + 1)
            break

if __name__ == "__main__":
    main()