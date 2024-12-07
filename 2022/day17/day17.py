import sys
from itertools import cycle

pieces = [ 
    (4, 1, ((0, 0), (1, 0), (2, 0), (3, 0))),
    (3, 3, ((1,0), (0, -1), (1, -1), (2, -1), (1, -2))),
    (3, 3, ((2, 0), (2, -1), (0, -2), (1, -2), (2, -2))),
    (1, 4, ((0,0), (0, -1), (0, -2), (0, -3))),
    (2, 2, ((0,0), (1, 0), (0, -1), (1, -1))),
]
num_pieces = 10000
c_test = set()
def main():
    gusts_iter = cycle([(i, x) for i, x in enumerate(sys.stdin.read()) ])
    piece_iter = cycle(pieces)

    new_row = lambda: ["."]*7 
    hall = [["#"]*7]
    highest = 0
    for piece_num in range(num_pieces):
        w, h, coords = next(piece_iter)
        x = 2
        y = highest + h + 3
        while y + 1 > len(hall):
            hall.append(new_row())
        #print( "\n".join("".join(x) for x in hall[::-1]))   
        while True:
            # Jet 
            move_id, move = next(gusts_iter)
            x_delta = 1 if move == ">" else -1
            can_move = True
            for x_b, y_b in coords:
                if x_delta == 1:
                    if (x+x_b) >= 6 or hall[y+y_b][x+x_b + 1] == "#":
                        can_move = False
                        break
                else:
                    if (x+x_b) <= 0 or hall[y+y_b][x+x_b - 1] == "#":
                        can_move = False
                        break
            if can_move:
                x += x_delta
            # check_below
            hit = False
            for x_b, y_b in coords:
                if hall[y + y_b - 1][x + x_b] == "#":
                    #print("test", y + y_b - 1, x+x_b)
                    hit = True
                    break
            if not hit:
                y -= 1
            else:
                break
            #print(x, y)
        for x_b, y_b in coords:
            hall[y + y_b][x + x_b] = "#"
            highest = max(highest, y+y_b)
        if any(x == ["#"]*7 for x in hall[-7:]):
            if (piece_num%len(pieces), move_id) in c_test:
                print(piece_num, piece_num%len(pieces), move_id, highest)
                print( "\n".join("".join(x) for x in hall[-7:][::-1]))   
            c_test.add((piece_num%len(pieces), move_id))
        
    print(highest)

if __name__ == "__main__":
    main()