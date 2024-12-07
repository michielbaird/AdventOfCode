import sys

def solve(moves, size):
    visited = {(0,0)}
    segments = [[0,0] for i in range(size)]
    head = segments[0]
    tail = segments[-1]
    for m in moves:
        if m == "U":
            head[0] += 1
        elif m == "D":
            head[0] -= 1
        elif m == "L":
            head[1] -= 1
        elif m == "R":
            head[1] += 1
        for i in range(1, len(segments)):
            lead = segments[i-1]
            follow = segments[i]
            if abs(lead[0]- follow[0]) == 2 and abs(lead[1]-follow[1]) == 2:
                follow[0] = lead[0] + (follow[0]-lead[0])//2
                follow[1] = lead[1] + (follow[1]-lead[1])//2
            elif lead[0] - follow[0] == 2:
                follow[1] = lead[1]
                follow[0] = lead[0] - 1
            elif lead[0] - follow[0] == -2:
                follow[1] = lead[1]
                follow[0] = lead[0] + 1
            elif lead[1] - follow[1] == 2:
                follow[1] = lead[1] -1
                follow[0] = lead[0]
            elif lead[1] - follow[1] == -2:
                follow[1] = lead[1] + 1
                follow[0] = lead[0]

        visited.add(tuple(tail))
    #print(visited)
    print(len(visited))



if __name__ == "__main__":
    moves = [ m for move in sys.stdin.read().split("\n") for m in (lambda x: [x[0]]*int(x[1]))(move.split(" "))]
    #print(moves)
    solve(moves,2) 
    solve(moves,10) 

