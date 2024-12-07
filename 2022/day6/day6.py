import sys
from collections import Counter
score = 0
for line in sys.stdin:
    line = line.strip()
    doubles = 0
    counter = Counter()
    buffer_size = 14
    for i in range(buffer_size):
        counter[line[i]] += 1
        if counter[line[i]] == 2:
            doubles += 1
    i = buffer_size
    while doubles > 0 and i < len(line):
        counter[line[i-buffer_size]] -= 1
        if counter[line[i-buffer_size]] == 1:
            doubles -= 1
        counter[line[i]] += 1
        if counter[line[i]] == 2:
            doubles += 1
        i += 1
    print(i)
    score += i

print(score)
    