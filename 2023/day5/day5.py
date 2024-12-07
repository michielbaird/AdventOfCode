import sys
import bisect

raw = sys.stdin.read()
parts = raw.split("\r\n\r\n")
#print(parts)
seeds = [int(v) for v in parts[0].split(": ")[1].split(" ")]

graph = {}
maps = {}

for m in parts[1:]:
    inner = m.split("\r\n")
    f, _, t = inner[0].split(" ")[0].split("-")
    graph[f] = t
    m = []
    for dest, start, range in [l.split(" ") for l in inner[1:]]:
        m.append((int(start), int(dest), int(range)))
    m.sort()
    maps[f] = m
print(maps)
print(seeds[:])
cur_type = "seed"
cur_vals = seeds[:]
while cur_type != "location":
    next_type = graph[cur_type]
    next_vals = []
    for v in cur_vals:
        index = bisect.bisect_right(maps[cur_type], v, key=lambda v: v[0])
        if index == 0:
            next_vals.append(v)
        else:
            s, dest, r = maps[cur_type][index - 1]
            if v < s + r :
                next_vals.append(dest + v - s)
            else: 
                next_vals.append(v)
    print(next_vals)
    cur_vals = next_vals
    cur_type = next_type

print(cur_vals)
print(min(cur_vals))
            
        


