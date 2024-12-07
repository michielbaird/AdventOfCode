import sys
import re
from collections import deque
from itertools import chain

p = re.compile(r"^Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? ((?:\w{2}(?:\, \w{2})*))$")

def calc_travel_time(travel, graph, start, end):
    if result := travel.get((start, end)):
        return result
    que = deque()
    que.append((0, start))
    visited = {start}
    while len(que) > 0:
        dist, node = que.popleft()
        for n in graph[node]:
            if n == end:
                travel[(start, end)] = dist + 1
                return dist + 1
            if n not in visited:
                visited.add(n)
                que.append((dist + 1, n))

def calc_flow(order, travel, graph, flow):
    total = 0
    current = "AA"
    time = 0
    i = 0
    current_flow = 0
    while i < len(order):
        delta = calc_travel_time(travel, graph, current, order[i])
        if time + delta + 1 >= 30:
            break
        total += (delta + 1)*current_flow
        time += delta + 1
        current_flow +=  flow[order[i]]
        current = order[i]
        i += 1
    total += current_flow*(30 - time)
    return total

#flowrate(current_time, current_location, opened) -> flow


def main():
    flow_rate = {}
    graph = {}
    has_flow = []
    for line in sys.stdin.read().split("\n"):
        #print(line)
        m = p.match(line)
        name, flow, neigh = m.group(1), int(m.group(2)), m.group(3).split(", ")
        flow_rate[name] = flow
        graph[name] = neigh
        if flow > 0:
            has_flow.append(name)

    has_flow.sort()
    
    flow_by_ident = []
    flow_ident_high = 2**len(has_flow)
    for f_id in range(flow_ident_high):
        v = f_id
        result = 0
        for i in range(len(has_flow)):
            if v & 1 == 1:
                result += flow_rate[has_flow[i]]
            v >>= 1
        flow_by_ident.append(result)
    flow_name_to_id = {}
    for i, v in enumerate(has_flow):
        flow_name_to_id[v] = i
    #print(flow_by_ident)
    #print(flow_name_to_id)
    #print(flow_rate)
    

    DP = {}
    DP[("AA", 0)] = 0
    score = 0
    print("Test")
    for t in range(1, 27):
        print(t)
        next_dp = {}
        for (l1, opened), current in DP.items():
            ident1 = flow_name_to_id.get(l1, -1)                
            can_open1 = ident1 != -1 and (opened & (1 << ident1)) == 0
      
            #  travel or one opens and one travels
            for n1 in graph[l1]:
                next_dp[(n1, opened)] = max( 
                    next_dp.get( (n1, opened), 0),
                    current + flow_by_ident[opened]
                )
            if can_open1:
                key = (l1, opened | (1 << ident1))
                next_dp[key] = max( 
                    next_dp.get(key, 0),
                    current + flow_by_ident[opened]
                )            
        DP = next_dp
    print("calculating")
    by_opened = {}
    for (_, opened), v in DP.items():
        by_opened[opened] = max(by_opened.get(opened, 0), v)
    best = 0
    for o1, v1 in by_opened.items():
        for o2, v2 in by_opened.items():
            if (o1 & o2) == 0:
                best = max(best, v1 + v2)
    print(best)
    print(max(DP.values()))
    
    print(score)
    #print(DP)

                    
                


            
    #DP(time, location, open) -> flow




if __name__ == "__main__":
    main()

