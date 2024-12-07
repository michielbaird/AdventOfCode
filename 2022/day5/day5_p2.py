import re
import sys

##p = re.compile("^(:?(?:(\s{3})|\[(.)\])\s?)*$")
p = re.compile("^move (\d+) from (\d+) to (\d+)$")

stacks = []
state = 0

for line in sys.stdin:
    if line[-1] == "\n":
        line = line[:-1]
    if state == 0:
        c = 0
        for j in range(0, len(line) - 1, 4):
            val = line[j:j+3]
            if c + 1 > len(stacks):
                stacks.append([])
            c += 1
            #print(repr(val))
            if val == "   ":
                continue
            elif val[0] == "[":
                stacks[c-1].append(val[1])
                #print(val[1])
            else:
                state = 1 
                break
    if state == 1:
        for l in stacks:
            l.reverse()
        state = 2
    elif state == 2:
        state = 3
    elif state == 3:
        m = p.match(line)
        if not m:
            break
        count = int(m.group(1))
        f = int(m.group(2))
        t = int(m.group(3))
        stacks[t-1].extend(stacks[f-1][len(stacks[f-1])-count:])
        for i in range(count):
            stacks[f-1].pop()
ans = []
for s in stacks:
    if len(s) > 0:
        ans.append(s[-1])
print("".join(ans))

print(stacks)
            



    

