import sys
import re
p = re.compile(r"^(\d+)\-(\d+)\,(\d+)\-(\d+)$")
score = 0
score2 = 0
for line in sys.stdin:
    line = line.strip()
    m = p.match(line)
    a, b = int(m.group(1)), int(m.group(2))
    c, d = int(m.group(3)), int(m.group(4))
    if (a >= c and a <= d and b >= c and b <= d) or \
       (c >= a and c <= b and d >= a and d <= b):
       score += 1
    if (a >= c and a <= d) or (b >= c and b <= d) or \
       (c >= a and c <= b) or (d >= a and d <= b):
       score2 += 1
print(score)
print(score2)
