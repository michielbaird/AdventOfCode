import sys

start_x = 500


def main():
    lines = [[ (lambda x: (int(x[0]), int(x[1])))(c.split(",")) for c in raw_line.split(" -> ") ]  for raw_line in sys.stdin.read().split("\n")]
    map = {}
    highest = -1
    for to_draw in lines:
        for i in range(1, len(to_draw)):
            if to_draw[i-1][0] == to_draw[i][0]:
                low = min(to_draw[i-1][1], to_draw[i][1])
                high = max(to_draw[i-1][1], to_draw[i][1])
                for j in range(low, high+1):
                    map[(to_draw[i][0], j)] = True
            else:
                low = min(to_draw[i-1][0], to_draw[i][0])
                high = max(to_draw[i-1][0], to_draw[i][0])
                for j in range(low, high+1):
                    map[(j, to_draw[i][1])] = True
            highest = max(highest, to_draw[i-1][1], to_draw[i][1])
    #print(map, highest)
    score = 0
    while True:
        falling = True
        start = [start_x, 0]
        while start[1] <= highest:
            if not map.get((start[0], start[1]+1), False):
                start[1] += 1
            elif not map.get((start[0]-1, start[1]+1), False):
                start[1] += 1
                start[0] -= 1
            elif not map.get((start[0]+1, start[1]+1), False):
                start[1] += 1
                start[0] += 1
            else:
                map[tuple(start)] = True
                falling = False
                break
            #print(start)
        if falling:
            break
        else:
            score += 1
    #print(map)
    print(score)



    

if __name__ == "__main__":
    main()