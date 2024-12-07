import sys
import bisect

raw = sys.stdin.read()
parts = raw.split("\r\n\r\n")
seeds = [int(v) for v in parts[0].split(": ")[1].split(" ")]
new_seeds = [(seeds[i], seeds[i] + seeds[i+1] ) for i in range(0, len(seeds), 2)]

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
cur_type = "seed"
cur_vals = new_seeds[:1]
while cur_type != "location":
    next_type = graph[cur_type]
    next_vals = []
    c_map = maps[cur_type]
    for (r_start, r_end) in cur_vals:
        while r_start < r_end:
            ind = bisect.bisect_right(c_map, r_start, key=lambda v: v[0])
            if ind == 0:
                next_vals.push((r_start, min(r_end, c_map[0])))
                r_start = min(r_end, c_map[0])
            else:
                break

        

    print(next_type)              
    print(next_vals)
    cur_vals = sorted(next_vals)
    cur_type = next_type

print(cur_vals)
            
        


