import sys
import bisect
import re

raw = sys.stdin.read()
parts = re.split(r"\r?\n\r?\n", raw)
seeds = [int(v) for v in parts[0].split(": ")[1].split(" ")]
new_seeds = [(seeds[i], seeds[i] + seeds[i+1] ) for i in range(0, len(seeds), 2)]

graph = {}
maps = {}

for m in parts[1:]:
    inner = re.split("\r?\n", m)
    f, _, t = inner[0].split(" ")[0].split("-")
    graph[f] = t
    m = []
    for dest, start, range in [l.split(" ") for l in inner[1:]]:
        m.append((int(start), int(dest), int(range)))
    m.sort()
    maps[f] = m
cur_type = "seed"
cur_vals = new_seeds[:]
while cur_type != "location":
    next_type = graph[cur_type]
    next_vals = []
    c_map = maps[cur_type]
    for (r_start, r_end) in cur_vals:
        while r_start < r_end:
            ind = bisect.bisect_right(c_map, r_start, key=lambda v: v[0])
            if ind == 0:
                next_vals.append((r_start, min(r_end, c_map[0][0])))
                r_start = min(r_end, c_map[0][0])
            elif r_start < c_map[ind - 1][0] + c_map[ind - 1][2]:
                shift = (r_start - c_map[ind - 1][0])
                delta =  c_map[ind - 1][1] -  c_map[ind - 1][0]
                next_vals.append((c_map[ind-1][1] + shift, min(c_map[ind-1][1] + c_map[ind-1][2], r_end + delta)))
                r_start = min(c_map[ind-1][0] + c_map[ind-1][2], r_end)
            elif ind < len(c_map):
                next_vals.append((r_start,  min(r_end, c_map[ind][0])))
                r_start = min(r_end, c_map[ind][0])
            else:
                next_vals.append((r_start,  r_end))
                r_start = r_end
    # print(next_type)              
    # print(next_vals)
    cur_vals = sorted(next_vals)
    cur_type = next_type

# print(cur_vals)

print(min(v[0] for v in cur_vals))
      
        


