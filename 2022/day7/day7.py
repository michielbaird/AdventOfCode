import sys
from typing import Tuple, List, Optional
AT_MOST = 100000

TOTAL = 70000000
REQUIRED = 30000000

class Dir:
    def __init__(self, parent: 'Dir', name: str):
        self.name = name
        self.dirs = {}
        self.files = {}
        self.parent = parent
    
    def __str__(self) -> str:
        return self.display()
        
    def display(self, level=0):
        indent = "  "*level
        result = [
            indent + self.name + " {",
        ]
        indent2 = indent +  "  "
        for d in self.dirs.values():
            result.append(d.display(level + 1))
        for k, v in self.files.items():
            result.append(indent2 + k + " " + str(v))
        result.append(indent + "}")
        return "\n".join(result)

    def solve(self, sizes: Optional[List[int]] = None) -> Tuple[int, int, List[int]]:
        if sizes is None:
            sizes = []
        total = 0
        result = 0
        for d in self.dirs.values():
            t, inner_result, _ = d.solve(sizes=sizes)
            total += t
            result += inner_result
        for c in self.files.values():


            total += c
        sizes.append(total)
        if total <= AT_MOST:
            return total, result + total, sizes
        else:
            return total, result, sizes

    

def main():
    cmds = sys.stdin.read().split("\n")
    root = Dir(None, name="/")
    cur = root
    i = 0
    while i < len(cmds):
        cmd = cmds[i][2:].split(" ")
        if cmd[0] == "cd":
            d1 = cmd[1]
            if d1 == "/":
                cur = root
            elif d1 == "..":
                cur = cur.parent
            else:
                cur = cur.dirs[d1]
        elif cmd[0] == "ls":
            start = i
            while i + 1 < len(cmds) and cmds[i + 1][0] != "$":
                i += 1
            res = cmds[start+1:i+1]
            for r in res:
                r = r.split(" ")
                if r[0] == "dir":
                    cur.dirs[r[1]] = Dir(parent=cur, name=r[1])
                else:
                    size = int(r[0])
                    name = r[1]
                    cur.files[name] = size
        i += 1
    t, r, sizes = root.solve()
    free = TOTAL - t
    needed = REQUIRED - free
    sizes.sort()
    for i in sizes:
        if i >= needed:
            print(i)
            break
    print(r)
    print(str(root))

if __name__ == "__main__":
    main()
        