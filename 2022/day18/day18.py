import sys
from collections import deque
from dataclasses import dataclass
deltas = []
for i in [-1,0,1]:
    for j in [-1,0,1]:
            for k in [-1,0,1]:
                    if i != 0 or j != 0 or k != 0:
                        zeros = 0
                        zeros += 1 if i == 0 else 0
                        zeros += 1 if j == 0 else 0
                        zeros += 1 if k == 0 else 0
                        to_rem = 1 if zeros == 2 else 0
                        deltas.append((i,j,k, to_rem))

@dataclass
class Bounds:
    min_x: int
    max_x: int
    min_y: int
    max_y: int
    min_z: int
    max_z: int
    def update(self, x, y, z):
        self.min_x = min(self.min_x, x)
        self.max_x = max(self.max_x, x)
        self.min_y = min(self.min_y, y)
        self.max_y = max(self.max_y, y)
        self.min_z = min(self.min_z, z)
        self.max_z = max(self.max_z, z)
    def is_inbounds(self, x, y, z):
        return x >= self.min_x and \
            x <= self.max_x and \
            y >= self.min_y and \
            y <= self.max_y and \
            z >= self.min_z and \
            z <= self.max_z

def solve_body(x,y,z, visited, outside):
    que = deque()
    que.append((x, y, z))
    visited[(x,y,z)] = True
    surface = 0
    out_bounds = Bounds(x,x,y,y,z,z)

    while len(que) > 0:
        x1, y1, z1 = que.popleft()
        to_add = 6
        for xd, yd, zd, rem_v in deltas:
            xt = x1 + xd
            yt = y1 + yd
            zt = z1 + zd
            val = visited.get((xt, yt, zt))
            if val is not None:
                to_add -= rem_v
                if not val:
                    visited[(xt, yt, zt)] = True
                    out_bounds.update(xt, yt, zt)
                    que.append((xt, yt, zt))
            else:
                outside.add((xt, yt, zt))
        surface += to_add
    return surface, out_bounds

def solve_outside(x,y,z, index, visited, bounds: Bounds, out_idxs):
    que = deque()
    que.append((x, y, z))
    visited[(x,y,z)] = index
    if not bounds.is_inbounds(x, y, z):
        #print("holla", x,y,z)
        return False, 0
    surface = 0
    inside = True
    

    while len(que) > 0:
        x1, y1, z1 = que.popleft()
        for xd, yd, zd in (
            (1,0,0), (-1,0,0), 
            (0,1,0), (0,-1,0),
            (0,0,1), (0,0,-1)
        ):
            xt = x1 + xd
            yt = y1 + yd
            zt = z1 + zd
            val = visited.get((xt, yt, zt))
            if val is None:
                visited[(xt, yt, zt)] = index
                if not bounds.is_inbounds(xt, yt, zt):
                    inside = False
                else:
                    que.append((xt, yt, zt))
            elif val in out_idxs:
                inside = False
            elif val == True:
                surface += 1
    
    if inside:
        #print(surface2)
        return True, surface
    else:
        return False, 0

def main():
    coords = [
        (lambda x: ( int(x[0]), int(x[1]), int(x[2]) )  )(x.split(","))
        for x in sys.stdin.read().split("\n")
    ]
    #print(coords)
    visited = {}
    for x, y, z in coords:
        visited[(x,y,z)] = False
    out_tests = []
    surface = 0
    for x, y, z in coords:
        if visited.get((x,y,z), False):
            continue
        outside = set()
        additional, bounds =  solve_body(x,y,z, visited, outside)
        surface += additional
        out_tests.append((outside, bounds))
    
    print(surface)
    
    outside_idx = set()
    idx = 1
    for outside, bounds in out_tests:
        for x, y, z in outside:
            if visited.get((x, y, z)) is None:
                idx += 1
                inside, to_rem = solve_outside(x, y, z, idx, visited, bounds, outside_idx)
                if inside:
                    #print("inside", to_rem)
                    surface -= to_rem
                else:                    
                    #print("outside")
                    outside_idx.add(idx)



        
    print(surface)


if __name__ == "__main__":
    main()
