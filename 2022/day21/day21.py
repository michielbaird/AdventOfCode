import sys
from dataclasses import dataclass
from typing import Union, List
from collections import deque
import re

@dataclass()
class Op:
    name: str
    a: Union[int, str, ]
    op: str
    b: Union[int, str]
    def is_resolved(self):
        return isinstance(self.a, int) and isinstance(self.b, int)
    def result(self):
        match self.op:
            case "+":
                return self.a + self.b
            case "-":
                return self.a - self.b
            case "*":
                return self.a * self.b   
            case "/":
                return self.a // self.b
    def update(self, name, val):
        if name == self.a:
            self.a = val
        elif name == self.b:
            self.b = val
    
    def invert(self, expected):
        match self.op , self.a, self.b:
            case "+", str(a), int(b):
                return expected - b
            case "+", int(a), str(b):
                return expected - a
            case "-", str(a), int(b):
                return expected + b
            case "-", int(a), str(b):
                return a - expected
            case "*", str(a), int(b):
                return expected // b
            case "*", int(a), str(b):
                return expected // a 
            case "/", str(a), int(b):
                return b * expected
            case "/", int(a), str(b):
                return a // expected


@dataclass()
class RawVal:
    name: str
    val: int
    def is_resolved(self):
        return True
    def result(self):
        return self.val


def decode(line):
    if len(line) == 2:
        return RawVal(line[0][:-1], int(line[1]))
    else:
        return Op(line[0][:-1], line[1], line[2], line[3])

def solve_pt2(raw_lines):
    lines = [decode(line.split(" ")) for line in raw_lines]
    by_name = {}
    reverse = {}
    que = []
    root = None
    you = None
    for l in lines:
        if l.name == "humn":
                you = l
        elif isinstance(l, RawVal):
            que.append(l)
            by_name[l.name] = l
        else:
            if l.name == "root":
                root = l
            
            reverse.setdefault(l.a, set())
            reverse.setdefault(l.b, set())
            by_name[l.name] = l
            reverse[l.a].add(l.name)
            reverse[l.b].add(l.name)
    
    
    while len(que) > 0:
        next_que = []
        for v in que:
            for n in reverse.get(v.name, []):
                node = by_name[n]
                node.update(v.name, v.result())
                if node.is_resolved():
                    if node.name == "root":
                        print(node.result())
                        root = node
                    next_que.append(node)
        que = next_que

    name = "root"
    val = root.b if isinstance(root.a, str) else root.a
    node = root
    while name != "humn":
        name = node.a if isinstance(node.a, str) else node.b
        if name ==  "humn":
            break
        node = by_name[name]
        val = node.invert(val)
        #print(val)

    print(val)

      


def main():
    raw_lines = sys.stdin.read().split("\n")
    lines = [decode(line.split(" ")) for line in raw_lines]
    by_name = {}
    reverse = {}
    que = []
    root = None
    for l in lines:
        if isinstance(l, RawVal):
            que.append(l)
        else:
            reverse.setdefault(l.a, set())
            reverse.setdefault(l.b, set())
            by_name[l.name] = l
            reverse[l.a].add(l.name)
            reverse[l.b].add(l.name)

        if l.name == "root":
            root = l
    
    while len(que) > 0:
        next_que = []
        for v in que:
            for n in reverse.get(v.name, []):
                node = by_name[n]
                node.update(v.name, v.result())

                if node.is_resolved():
                    if node.name == "root":
                        print(node.result())
                        root = node
                    next_que.append(node)
        que = next_que
    
    solve_pt2(raw_lines)
    
    #print(lines)


if __name__ == "__main__":
    main()