import sys
from collections import deque

def go_up(e_map, cell, next_cell):
    return e_map[cell[0]][cell[1]] + 1 >= e_map[next_cell[0]][next_cell[1]]

def go_down(e_map, cell, next_cell):
    return e_map[cell[0]][cell[1]] - 1 <= e_map[next_cell[0]][next_cell[1]]

def solve(e_map, start, end, cell_func=go_up, end_at_zero=False):
    que = deque()
    n = len(e_map)
    m = len(e_map[0])

    visited = [[None for _ in range(m)] for _ in range(m)]
    visited[start[0]][start[1]] = 0
    que.append((0, start))
    while len(que):
        dist, cell = que.popleft()
        for dy, dx in [(1, 0), (0, 1), (-1, 0), (0, -1)]:
            next_cell = (cell[0] + dy, cell[1] + dx)
            
            if next_cell[0] < 0 or next_cell[0] >= n or \
                    next_cell[1] < 0 or next_cell[1] >= m or\
                    visited[next_cell[0]][next_cell[1]] is not None:
                continue
            if cell_func(e_map, cell, next_cell):
                if next_cell == end or (end_at_zero and e_map[next_cell[0]][next_cell[1]] == 0):
                    #print("\n".join(repr(x) for x in visited))
                    return dist + 1
                visited[next_cell[0]][next_cell[1]] = dist + 1
                que.append((dist+1,  next_cell))
    #print("\n".join(repr(x) for x in visited))
    

def main():
    start = (0, 0)
    end = (0, 0)
    e_map = []
    for row, line in enumerate(sys.stdin.read().split("\n")):
        v = []
        for col, c in enumerate(line):
            if c == "S":
                start = (row, col)
                v.append(0)
            elif c == "E":
                end = (row, col)
                v.append(25)
            else:
                v.append(ord(c)- ord("a"))
        e_map.append(v)
    #print("\n".join(repr(x) for x in e_map))
    print(solve(e_map, start, end))
    print(solve(e_map, end, start, cell_func=go_down, end_at_zero=True))





if __name__ == "__main__":
    main()
