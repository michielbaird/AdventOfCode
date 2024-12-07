import sys

score = 0
score2 = 0
window = []

def score_delta(r):
    if ord("a") <= r  and r <= ord("z"):
        return r - ord("a") + 1
    elif ord("A") <= r  and r <= ord("Z"):
        return r - ord("A") + 27

for line in sys.stdin:
    #print(line)
    line = line.strip()
    n = len(line)//2
    a = set(x for x in line[:n])
    b = set(x for x in line[n:])
    window.append(set(line))
    if len(window) == 3:
        r2 = ord(list(window[0].intersection(window[1]).intersection(window[2]))[0])
        score2 += score_delta(r2)
        window = []
    r = ord(list(a.intersection(b))[0])
    score += score_delta(r)


print(score)
print(score2)
    
     