import sys
import re

MAX_XY = 4_000_000

class RangeTree():
    def __init__(self, low=0, high=MAX_XY+1):
        self.low = low
        self.high = high
        self.mid = low + (high-low)//2
        self.covered = False
        self.left = None
        self.right = None
    def add(self, low, high):
        if self.covered:
            return
        if low <= self.low and high >= (self.high - 1):
            self.covered = True
            self.left = None
            self.right = None
            return
        if low < self.mid:
            self.get_left().add(low, high)
        if high >= self.mid:
            self.get_right().add(low, high)
        if self.left is not None and self.left.covered and self.right is not None and self.right.covered:
            self.covered = True
            self.left = None
            self.right = None
    
    def get_left(self):
        if self.left is None:
            self.left = RangeTree(self.low, self.mid)
        return self.left
    def get_right(self):
        if self.right is None:
            self.right = RangeTree(self.mid, self.high)
        return self.right
    def find_val(self):
        if self.covered:
            return
        if not self.left:
            return self.low
        if not self.left.covered:
            return self.left.find_val()
        if not self.right:
            return self.mid
        return self.right.find_val()
    
    def display(self, level=0):
        val = [" "*level + "Tree {"]
        val.append(" "*level + "low={}, high={}".format(self.low, self.high))
        if self.left is not None:
            val.append(self.left.display(level+1))
        if self.right is not None:
            val.append(self.right.display(level+1))
        val.append(" "*level + "}")
        return "\n".join(val)
p = re.compile("^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(\-?\d+), y=(-?\d+)$")


def solve(vals, y):
    count = 0
    row = {}
    for x1, y1, x2, y2 in vals:
        dist = abs(x1-x2) + abs(y1-y2)
        if y2 == y:
            #dist = 1
            row[x2] = True
        r = dist - abs(y-y1)
        #print(dist)
        if r > 0:
            if not row.get(x1, False):
                count += 1
                row[x1] = True
            for i in range(1, r+1):
                if not row.get(x1+i, False):
                    count += 1
                    row[x1+i] = True
                if not row.get(x1-i, False):
                    count += 1
                    row[x1-i] = True
    #print(sorted(list(row.keys())))
    print(count)

def solve2(vals):
    for y in range(0, MAX_XY+1):
        if y % 100_000 == 0:
            print(y)
        tree = RangeTree()
        for x1, y1, x2, y2 in vals:
            dist = abs(x1-x2) + abs(y1-y2)
            r = dist - abs(y-y1)
            if r > 0:
                tree.add(x1-r, x1+r)
        if not tree.covered:
            x = tree.find_val()
            print(x*4_000_000 + y)
            print("Woo: ", y)

            #print(tree.display())
def solve2_with_hint(vals):
    d_map = {}
    for i, (x1, y1, x2, y2) in enumerate(vals):
        d_map[i] = abs(x1-x2) + abs(y1-y2)
    print(d_map)
    def test_dists(x, y):
        return all( (lambda x1, y1, _x, _y: abs(x1-x) + abs(y1-y) > dist)(*vals[index]) for index, dist in d_map.items())
    
    for index, dist in d_map.items():
        print(index, dist)
        dist += 1
        x, y, _, _ = vals[index]
        for j in range(0, dist+1):
            for l, r in [(-1,-1), (1,-1), (-1, 1), (1,1)]:
                x_t, y_t = x + l*(dist - j), y + r*dist
                if 0 <= x_t <= MAX_XY and 0 <= y_t <= MAX_XY and test_dists(x_t, y_t):
                    print(x_t*4_000_000 + y_t)
                    print(x_t, y_t)
                    return


def main():
    vals = [
        (lambda m: (int(m.group(1)), int(m.group(2)), int(m.group(3)), int(m.group(4))))(p.match(x)) 
        for x in sys.stdin.read().split("\n")
    ]
    #solve(vals, 10)
    solve(vals, 2_000_000)
    solve2_with_hint(vals)
    print(vals)

low = 0
high = 4000000
    


if __name__ == "__main__":
    main()

x = 3157534
y = 3363767