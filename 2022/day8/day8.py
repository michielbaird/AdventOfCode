import sys
from functools import reduce

def solve(it):
    tall = -1
    count = 0
    stack = []
    for i, t in enumerate(it):
        if not t[1] and t[0] > tall:
            t[1] = True
            count += 1
        tall = max(tall, t[0])
        if len(stack) > 0:
            while len(stack) > 0 and stack[-1][0] < t[0]:
                stack.pop()
            low_i = stack[-1][1]
            t.append(i - low_i)
            while len(stack) > 0 and stack[-1][0] == t[0]:
                stack.pop()
            stack.append((t[0], i))
        else:
            stack.append((float("inf"), i))
            t.append(0)


    return count


grid = [ [ [int(tree), False] for tree in row]  for row in sys.stdin.read().split("\n")]

score = 0
for row in grid:
    score += solve(row)
    score += solve(row[::-1])

def column(grid, index, forward=True):
    #print(index)
    if forward:
        for i in range(len(grid)):
            yield grid[i][index]
    else:
        for i in range(len(grid)-1, -1, -1):
            yield grid[i][index]

for c_i in range(len(grid)):
    score += solve(column(grid, c_i))
    score += solve(column(grid, c_i, False))
best = 0
for row in grid:
    for t in row:
        view = reduce(lambda a,b: a*b, t[-4:], 1)
        best = max(best, view)
#print("\n".join(repr(x) for x in grid))
print(score)
print(best)


    