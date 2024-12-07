import sys
#from functools import map


def main():
    start = 20
    delta = 40
    max_cycle = 220

    samples = []
    samples2 = [20, 60, 100, 140, 180, 220]

    x = 1
    next_x = 1
    cycle = 0
    sample_pos = 20


    instructions = iter(sys.stdin.read().split("\n"))
    screen = []
    line = []
    left = 0
    ins = next(instructions)
    match ins.split(" "):
        case ["noop"]:
            next_x = x
            left = 1
        case ["addx", v]:
            v = int(v)
            next_x = x + v
            left = 2
    try:
        for cycle in range(240):
            tmp = (cycle+1) % 40
            if tmp >= x and tmp < x + 3:
                line.append("#")
            else:
                line.append(".")
            if tmp % 40 == 0:
                screen.append("".join(line))
                line = []
            if cycle+1 == sample_pos:
                sample_pos += 40
                samples.append(x)
            left -= 1
            if left == 0:
                ins = next(instructions)
                x = next_x
                match ins.split(" "):
                    case ["noop"]:
                        next_x = x
                        left = 1
                    case ["addx", v]:
                        v = int(v)
                        next_x = x + v
                        left = 2

            #left -= 1
    except StopIteration:
        pass
    screen.append("".join(line))
    print(samples)
    #print(next(zip(samples, samples2)))
    print(  
        sum(
            map(lambda x: x[0]*x[1], zip(samples, samples2))
        )
    )
    print("\n".join(screen))


if __name__ == "__main__":
    main()