import sys
score = 0
score2 = 0
for line in sys.stdin:
    round = line.strip().split(" ")
    opp = ord(round[0]) - ord("A")
    you = ord(round[1]) - ord("X")
    score += 1 + you

    # Rock - 0, Paper - 1, Scissors - 2
    result = (opp - you) % 3
    if result == 0:
        score += 3
    elif result == 2:
        score += 6

    if round[1] == "X":
        you2 = (opp + 2) % 3
    elif round[1] == "Y":
        you2 = opp
    elif round[1] == "Z":
        you2 = (opp + 1) % 3
    score2 += you2 + 1

    result2 = (opp - you2) % 3
    if result2 == 0:
        score2 += 3
    elif result2 == 2:
        score2 += 6
print(score)
print(score2)