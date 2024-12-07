import sys
import re

def solve_a(times, distances):
    result = 1
    for t, d in zip(times, distances):
        count = 0
        for i in range(0, t+1):
            reach = i*(t - i)
            if reach > d:
                count += 1
        result *= count
    print(f"Part 1: {result}")



def main():
    raw_input = sys.stdin.readlines()
    times = [int(t) for t in re.findall(r"\s+(\d+)", raw_input[0])] 
    distances = [int(t) for t in re.findall(r"\s+(\d+)", raw_input[1])] 

    solve_a(times, distances)
    times2 = [int("".join(re.findall(r"\s+(\d+)", raw_input[0])))] 
    distances2 =[int("".join(re.findall(r"\s+(\d+)", raw_input[1])))] 
    solve_a(times2, distances2)




if __name__ == "__main__":
    main()