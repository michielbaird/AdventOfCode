import sys
from collections import deque

BASE_CUBE = {
    0: {0: (5, 2), 1: (3, 0), 2: (2, 0), 3: (1, 0)},
    1: {0: (0, 3), 1: (2, 3), 2: (4, 3), 3: (5, 3)},
    2: {0: (0, 2), 1: (3, 3), 2: (4, 0), 3: (1, 1)},
    3: {0: (0, 1), 1: (5, 1), 2: (4, 1), 3: (2, 1)},
    4: {0: (2, 2), 1: (3, 2), 2: (5, 0), 3: (1, 2)},
    5: {0: (4, 2), 1: (3, 1), 2: (0, 0), 3: (1, 3)},
}
FACE_DELTAS = [(-1, 0), (0, 1), (1, 0), (0, -1)]


class Face:
    def __init__(self, length):
        self.rotation = 0
        self.length = length
        self.face_id = 0
        self.filled = False
        self.row = 0
        self.col = 0
        self.length = length
        self.sides = [[] for _ in range(4)]
    def set_face(self,
        rotation,
        face_id,
        row,
        col,
        length
    ):
        self.rotation = rotation  
        self.row = row
        self.col = col
        self.length = length
        self.face_id = face_id
        self.filled = True

    def get_index(self, row, col, dir):
        indexes = [
            col-self.col, 
            row-self.row, 
            self.length - col + self.col - 1, 
            self.length - row + self.row -1
        ]
        next_row, next_col = row + FACE_DELTAS[dir][0], col + FACE_DELTAS[dir][1]
        if next_row == self.row - 1:
            local_side = 0
        elif next_col == self.col + self.length:
            local_side = 1
        elif next_row == self.row + self.length:
            local_side = 2
        elif next_col == self.col - 1:
            local_side = 3
        else:
            return None, (next_row, next_col)

        index = indexes[(local_side + 2) % 4]
        real_side = (local_side + self.rotation) % 4
        return index, real_side
    
    def get_position(self, index, entry_side):
        local_side = (entry_side - self.rotation) % 4
        if local_side == 0:
            dir = 2
            row = self.row
            col = self.col + index
        elif local_side == 1:
            dir = 3
            row = self.row + index
            col = self.col + self.length - 1
        elif local_side == 2:
            dir = 0
            row = self.row + self.length - 1
            col = self.col + self.length -1 - index
        else:
            dir = 1
            row = self.row + self.length - 1 - index
            col = self.col
        return dir, row, col

    def __repr__(self):
        return f"Face(row={self.row}, col={self.col}, rotation={self.rotation}, face_id={self.face_id})"


def get_input():
    parts = [line for line in sys.stdin.read().split("\n\n")]
    raw_map = [[c for c in line] for line in parts[0].split("\n")]
    
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
    length = max(len(raw_map), max(len(v) for v in raw_map))//4
    return raw_map, instructions, length

def label_map(cube_map, length):
    index = 0
    result = {}
    first = None
    for row in range(0, len(cube_map), length):
        for col in range(0, len(cube_map[row]), length):
            if cube_map[row][col] != " ":
                if first is None:
                    first = (row, col)
                result[(row, col)] = index
                index += 1
    return result, first

def main():
    cube_map, instructions, length = get_input()
    map_labels, first_face = label_map(cube_map, length)
    faces = [Face(length) for _ in range(6)]

    faces[0].set_face(0, 0, first_face[0], first_face[1], length)
    que = deque()
    que.append((first_face, 0))
    while len(que) > 0:
        (row, col), face_id = que.popleft()
        #input_id = map_labels[(row, col)]
        rotation = faces[face_id].rotation
        for i in range(4):
            next_row = row + FACE_DELTAS[i][0]*length
            next_col = col + FACE_DELTAS[i][1]*length
            if (next_input_id := map_labels.get((next_row, next_col))) is None:
                continue
            cube_side = (i + rotation)%4
            next_face_id, entry_side = BASE_CUBE[face_id][cube_side]

            if faces[next_face_id].filled:
                continue
            next_rotation = (rotation + entry_side - cube_side - 2) % 4 
            faces[next_face_id].set_face(
                next_rotation,
                next_input_id,
                next_row, next_col, length
            )
            que.append(((next_row, next_col), next_face_id))
    #faces[5].rotation = 3
    direction = 1
    face_id = 0
    row = faces[face_id].row
    col = faces[face_id].col
    for ins in instructions:
        if ins == "L":
            direction = (direction - 1) % 4
        elif ins == "R":
            direction = (direction + 1) % 4
        else:
            for _ in range(ins):
                index, part2 = faces[face_id].get_index(row, col, direction)
                if index is not None:
                    side = part2
                    next_face_id, entry_side = BASE_CUBE[face_id][side]
                    next_direction, next_row, next_col = faces[next_face_id].get_position(index, entry_side)
                else:
                    next_row, next_col = part2
                    next_face_id = face_id
                    next_direction = direction
                if cube_map[next_row][next_col] != "#":
                    row, col = next_row, next_col
                    face_id = next_face_id
                    direction = next_direction
                    #cube_map[row][col] = "o"
                else:
                    break
    print(row, col, face_id)
    print(((row+1)*1000)+((col+1)*4) + faces[face_id].face_id + 1)
    #print(faces)
    #print(map_labels)
                
    
            


if __name__ == "__main__":
    main()