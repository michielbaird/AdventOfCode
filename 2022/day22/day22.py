import sys
from dataclasses import dataclass
from typing import Callable, Tuple

@dataclass(frozen=True)
class FaceBounds:
    min_col: int
    max_col: int
    min_row: int
    max_row: int
    left_func: Callable[[int, int], Tuple[int, int, str, int]]
    right_func: Callable[[int, int], Tuple[int, int, str, int]]
    up_func: Callable[[int, int], Tuple[int, int, str, int]]
    down_func: Callable[[int, int], Tuple[int, int, str, int]]
    def is_in_bounds(self, row, col):
        return self.min_col <= col and self.max_col >= col and \
            self.min_row <= row and self.max_row >= row

Faces = [
    FaceBounds(
        50, 99, 0, 49,
        left_func = lambda row, col: (149-row, 0, "R", 3),
        right_func = lambda row, col: (row, col+1, "R", 1),
        down_func = lambda row, col: (row+1, col, "D", 2),
        up_func = lambda row, col: (col + 100 , 0, "R", 5),

    ),
    FaceBounds(
        100, 149, 0, 49,
        left_func = lambda row, col: (row, col-1, "L", 0),
        right_func = lambda row, col: (149-row, 99, "L", 4),
        up_func = lambda row, col: (199 , col-100, "U", 5),
        down_func = lambda row, col: (col-50, 99, "L", 2),
    ),
    FaceBounds(
        50, 99, 50, 99,
        left_func = lambda row, col: (100, row-50, "D", 3),
        right_func = lambda row, col: (49, row+50, "U", 1),
        up_func = lambda row, col: (row-1 , col, "U", 0),
        down_func = lambda row, col: (row+1, col, "D", 4),
    ),
    FaceBounds(
        0, 49, 100, 149,
        left_func = lambda row, col: (149-row, 50, "R", 0),
        right_func = lambda row, col: (row, col+1, "R", 4),
        up_func = lambda row, col: (col+50 , 50, "R", 2),
        down_func = lambda row, col: (row+1, col, "D", 5),
    ),
    FaceBounds(
        50, 99, 100, 149,
        left_func = lambda row, col: (row, col-1, "L", 3),
        right_func = lambda row, col: (149-row, 149, "L", 1),
        up_func = lambda row, col: (row-1 , col, "U", 2),
        down_func = lambda row, col: (100+col, 49, "L", 5),
    ),
    FaceBounds(
        0, 49, 150, 199,
        left_func = lambda row, col: (0, row-100, "D", 0),
        right_func = lambda row, col: (149, row-100, "U", 4),
        up_func = lambda row, col: (row-1 , col, "U", 3),
        down_func = lambda row, col: (0, col+100, "D", 1),
    ),
]

def solve2(map, instructions):
    deltas = [(0,1), (1, 0), (0,-1), (-1,0)]

    row = 0
    col = 50
    dir = 0
    face = Faces[0]
    face_id = 0
    dir_to_int = {
        "R": 0, "D": 1, "L": 2, "U": 3
    }
        # dir 0 -> Right, 1 -> Down, 2 -> Left, 3 -> up
    
    for ins in instructions:
        match ins:
            case "L":
                dir = (dir - 1) % 4
            case "R":
                dir = (dir + 1) % 4
            case int(walk):
                for i in range(walk):
                    next_row = row + deltas[dir][0]
                    next_col = col + deltas[dir][1]
                    next_face_id = face_id
                    next_face = face
                    next_dir = dir
                    if not face.is_in_bounds(next_row, next_col):
                        match dir:
                            case 0:
                                t_func = face.right_func 
                            case 1:
                                t_func = face.down_func 
                            case 2:
                                t_func = face.left_func 
                            case 3:
                                t_func = face.up_func              
                                      
                        next_row, next_col, next_dir, next_face_id = t_func(row, col)
                        next_dir = dir_to_int[next_dir]
                        next_face = Faces[next_face_id]
                        if not next_face.is_in_bounds(next_row, next_col):
                            print("bad travel")
                            return
                        if next_row < 0 or next_col < 0:
                            return     
                        #print(face)
                        #return
                    if map[next_row][next_col] == "#":
                        break
                    col = next_col
                    row = next_row
                    face_id = next_face_id
                    face = next_face
                    dir = next_dir

    print(row, col, face_id)
    print(((row+1)*1000)+((col+1)*4) + face_id + 1)



def main():
    parts = [line for line in sys.stdin.read().split("\n\n")]
    raw_map = parts[0].split("\n")
    m = max(len(v) for v in raw_map)

    raw_instructions = parts[1]
    val = 0
    instructions = []
    for c in raw_instructions:
        if ord(c) >= ord("0") and ord(c) <= ord("9"):
            val *= 10
            val += ord(c) - ord("0")
            continue
        if val != 0:
            instructions.append(val)
            val = 0
        instructions.append(c)
    if val != 0:
            instructions.append(val)
    #print(instructions)      

    map = []
    row_bounds = []
    column_bounds = [[float("inf"), -1] for i in range(m)]
    for r, row in enumerate(raw_map):
        map_row = []
        row_min = float("inf")
        row_max = -1
        for c, cell in enumerate(row):
            map_row.append(cell)
            if cell != " ":
                column_bounds[c][0] = min(column_bounds[c][0], r)
                column_bounds[c][1] = max(column_bounds[c][1], r)
                row_min = min(row_min, c)
                row_max = max(row_max, c)
        map.append(map_row)
        row_bounds.append([row_min, row_max])
    row = 0
    col = row_bounds[0][0]
    #print(row_bounds, column_bounds)
    # dir 0 -> Right, 1 -> Down, 2 -> Left, 3 -> up
    

    deltas = [(0,1), (1, 0), (0,-1), (-1,0)]
    dir = 0
    for ins in instructions:
        match ins:
            case "L":
                dir = (dir - 1) % 4
            case "R":
                dir = (dir + 1) % 4
            case int(walk):
                for i in range(walk):
                    next_row = row + deltas[dir][0]
                    next_col = col + deltas[dir][1]
                    if next_row < column_bounds[col][0]:
                        next_row = column_bounds[col][1]
                    elif next_row > column_bounds[col][1]:
                        next_row = column_bounds[col][0]
                    if next_col < row_bounds[row][0]:
                        next_col = row_bounds[row][1]
                    elif next_col > row_bounds[row][1]:
                        next_col = row_bounds[row][0]
                    if map[next_row][next_col] == "#":
                        break
                    col = next_col
                    row = next_row
    print(row, col)
    print(((row+1)*1000)+((col+1)*4))
    solve2(map, instructions)
                    


            


if __name__ == "__main__":
    main()