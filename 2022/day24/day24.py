import sys
from heapq import heappush, heappop

DELTAS = [(0,1), (1,0), (-1, 0), (0, -1)]

class PathFinder:
    def __init__(self, valley_map):
        self.height = len(valley_map)-2
        self.width = len(valley_map[0])-2
        self.left_blizzard = [
            set(x-1 for x in range(1,len(line)-1) if line[x] == "<")
            for line in valley_map[1:-1]
        ]
        self.right_blizzard = [
            set(x-1 for x in range(1,len(line)-1) if line[x] == ">")
            for line in valley_map[1:-1]
        ]
        self.up_blizzard = [
            set(x-1 for x in range(1, len(valley_map)-1) if valley_map[x][y] == "^")
            for y in range(1,len(valley_map[0])-1)
        ]
        self.down_blizzard = [
            set(x-1 for x in range(1,len(valley_map)-1) if valley_map[x][y] == "v")
            for y in range(1,len(valley_map[0])-1)
        ]

    def check_blizzards(self, row, col, time):
        if row == -1 or row == self.height:
            return False
        return ((col - time) % self.width) in self.right_blizzard[row] or \
            ((col + time) % self.width) in self.left_blizzard[row] or \
            ((row - time) % self.height) in self.down_blizzard[col] or \
            ((row + time) % self.height) in self.up_blizzard[col]
    
    def score(self, position, goal, time):
        return abs(goal[0]- position[0]) + abs(goal[1] - position[1]) + time
    
    def solveAStar(self, start, end, start_time=0):
        p_que = [(self.score(start, end, start_time), start, start_time)]
        visited = set()
        while len(p_que) > 0:
            _, pos, t = heappop(p_que)
            if (pos, t) in visited:
                continue
            visited.add((pos, t))
            row, col = pos
            next_t = t + 1
            if (pos, next_t) not in visited and \
                    not self.check_blizzards(row, col, next_t):
                heappush(p_que, (self.score(pos, end, next_t), pos, next_t))
            for dr, dc in DELTAS:
                next_row, next_col = row + dr, col + dc
                next_pos = (next_row, next_col)
                if (next_pos, next_t) in visited:
                    continue
                if (next_row, next_col) == end:
                    return next_t
                if next_row < 0 or next_row >= self.height or \
                    next_col < 0 or next_col >= self.width:
                    # We hit a wall
                    continue
                if self.check_blizzards(next_row, next_col, next_t):
                    # There is a blizzard in the way
                    continue
                heappush(p_que, (
                    self.score((next_row, next_col), end, next_t), 
                    (next_row, next_col), 
                    next_t
                ))
        return -1


    def solveBfs(self, start, end, start_time=0):
        que = {start}
        t = start_time
        while len(que) > 0:
            next_que = set()
            next_t = t+1
            for row, col in que:
                if not self.check_blizzards(row, col, next_t):
                    next_que.add((row, col))
                for dr, dc in DELTAS:
                    next_row, next_col = row + dr, col + dc
                    if (next_row, next_col) == end:
                        return next_t
                    if next_row < 0 or next_row >= self.height or \
                        next_col < 0 or next_col >= self.width:
                        # We hit a wall
                        continue
                    if self.check_blizzards(next_row, next_col, next_t):
                        # There is a blizzard in the way
                        continue
                    next_que.add((next_row, next_col))
            que = next_que
            t = next_t
        return -1

def main():
    valley = sys.stdin.read().split("\n")

    path_finder = PathFinder(valley)
    start = (-1, valley[0].index(".") -1)
    end = (len(valley)-2, valley[-1].index(".") - 1)
    #print(le)
    for func in [path_finder.solveBfs, path_finder.solveAStar]:
        t1 = func(start, end)
        print("Part 1:", t1)
        t2 = func(end, start, t1)
        t3 = func(start, end, t2)
        print("Part 2:", t3)

if __name__ == "__main__":
    main()

