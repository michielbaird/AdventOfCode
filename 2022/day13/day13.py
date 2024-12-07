import sys
from functools import cmp_to_key

def parse(raw_val, index=0):
    result = []
    val = None
    while index < len(raw_val):
        if raw_val[index] == "[":
            index, val = parse(raw_val, index+1)
        elif raw_val[index] == ",":
            result.append(val)
            val = 0
        elif raw_val[index] == "]":
            if val != None:
                result.append(val)
            return index, result
        else:
            if val is None:
                val = 0
            val *= 10
            val += int(raw_val[index])
        index += 1
    if val is not None:
        result.append(val)
    return len(raw_val)-1, result

def compare(left, right):
    for l, r in zip(left, right):
        match (l, r):
            case (int(l_v), int(r_v)):
                if l_v != r_v:
                    return -1 if l_v < r_v else 1
            case ([*l_v], [*r_v]):
                r = compare(l_v, r_v)
                if r != 0:
                    return r
            case (int(l_v), [*r_v]):
                return compare([l_v], r_v)
            case ([*l_v], int(r_v)):
                return compare(l_v, [r_v])
    if len(left) < len(right):
        return -1
    elif len(left) == len(right):
        return 0
    else:
        return 1

def main():
    raw_input = sys.stdin.read()
    vals = [part.split("\n") for part in raw_input.split("\n\n")]
    score = 0
    all_vals = [ [[2]], [[6]], ]
    for i, test in enumerate(vals):
        _, left = parse(test[0][1:-1])
        _, right = parse(test[1][1:-1])
        if compare(left, right) == -1:
            score += i + 1
        all_vals.append(left)
        all_vals.append(right)
    print(score)
    all_vals.sort(key=cmp_to_key(compare))
    key_1 = all_vals.index([[2]]) + 1
    key_2 = all_vals.index([[6]]) + 1
    print(key_1*key_2)
    #print(all_vals)

if __name__ == "__main__":
    main()

        