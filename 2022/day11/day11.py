import sys
import re
from typing import Dict, List, Tuple, Callable, Optional
from functools import reduce

p_operation = re.compile(r"^new = (old|\-?\d+) ([\+\-\*]) (old|\-?\d+)$")


def parse_token(token: str, value: int):
    if token == "old":
        return value
    else:
        return int(token)

def eval(operation: str, value: int):
    m = p_operation.match(operation)
    operand = m.group(2)
    if operand == "+":
        f = lambda l, r: l + r
    elif operand == "-":
        f = lambda l, r: l - r
    elif operand == "*":
        f = lambda l, r: l * r
    return f(parse_token(m.group(1), value), parse_token(m.group(3), value))
        

class Monkey:
    def __init__(
        self: 'Monkey', 
        id: int, 
        items: List[int], 
        operation: str, 
        test: int, 
        pass_mon: int, 
        fail_mon: int
    ):
        self.id = id
        self.items = items
        self.operation = operation
        self.test = test
        self.pass_mon = pass_mon
        self.fail_mon = fail_mon
        self.inspections = 0
    
    def play_turn(self: 'Monkey', adjust: Callable[[int], int]) -> List[Tuple[int, int]]:
        result = []
        for item in self.items:
            new = adjust(eval(self.operation, item))
            self.inspections += 1
            if new % self.test == 0:
                n = self.pass_mon
            else:
                n = self.fail_mon
            result.append((n, new))
        self.items = []
        return result

    
    def __repr__(self: 'Monkey') -> str:
        return \
f"""Monkey {self.id}:
    Items: {self.items}
    Operation: {self.operation}
    Test: {self.test}
        Pass: {self.pass_mon}
        Fail: {self.fail_mon}
    Inspections: {self.inspections}
"""

def monkey_builder(raw_monkey: str) -> 'Monkey':
    parts = raw_monkey.split("\n")
    id = int(parts[0].split(" ")[-1][:-1])
    raw_items = parts[1].split(": ")[1]
    if raw_items == "":
        items = []
    else:
        items = [int(x)  for x in raw_items.split(", ")]
    operation = parts[2].split(": ")[1]
    test = int(parts[3].split(" ")[-1])
    pass_mon = int(parts[4].split(" ")[-1])
    fail_mon = int(parts[5].split(" ")[-1])
    return Monkey(
        id,
        items,
        operation,
        test,
        pass_mon,
        fail_mon
    )

def parse_input(raw: str) -> Dict[int, 'Monkey']:
    result = {}
    for x in raw.split("\n\n"):
        monk = monkey_builder(x)
        result[monk.id] = monk
    return result

def run_round(raw_monkeys: str, part2: Optional[bool]=False):
    monkeys = parse_input(raw_monkeys)
    if part2:
        rounds = 10_000
        mod = reduce(lambda v, m: v*m.test, monkeys.values(), 1)
        adjust = lambda x: x%mod
    else:
        rounds = 20
        adjust = lambda x: x//3


    n = len(monkeys)
    for _ in range(rounds):
        for i in range(n):
            to_move = monkeys[i].play_turn(adjust)
            for (monkey, val) in to_move:
                monkeys[monkey].items.append(val)
    l_monkeys = list(monkeys.values())
    l_monkeys.sort(key=lambda m: -m.inspections)
    #print(l_monkeys[:2])
    print(l_monkeys[0].inspections * l_monkeys[1].inspections)

def main():
    raw_monkeys = sys.stdin.read()
    run_round(raw_monkeys)
    run_round(raw_monkeys, True)





if __name__ == "__main__":
    main()