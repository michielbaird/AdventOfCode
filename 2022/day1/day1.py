from heapq import heappush, heappop

import sys
best_elves = []
current_elf = 0
for line in sys.stdin:
	#print(repr(line))
	if line == "\n":
		heappush(best_elves, current_elf)
		if len(best_elves) > 3:
			heappop(best_elves)
		#best_elf = max(best_elf, current_elf)
		current_elf = 0
		continue
	current_elf += int(line.strip())


heappush(best_elves, current_elf)
if len(best_elves) >= 3:
	heappop(best_elves)

print(max(best_elves))
print(sum(best_elves))