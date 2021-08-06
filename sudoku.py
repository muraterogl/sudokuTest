board = [
    [5, 3, 0, 0, 7, 0, 0, 0, 0],
    [6, 0, 0, 1, 9, 5, 0, 0, 0],
    [0, 9, 8, 0, 0, 0, 0, 6, 0],
    [8, 0, 0, 0, 6, 0, 0, 0, 3],
    [4, 0, 0, 8, 0, 3, 0, 0, 1],
    [7, 0, 0, 0, 2, 0, 0, 0, 6],
    [0, 6, 0, 0, 0, 0, 2, 8, 0],
    [0, 0, 0, 4, 1, 9, 0, 0, 5],
    [0, 0, 0, 0, 8, 0, 0, 7, 9],
]

# board = [
#     [0, 0, 0, 0, 1, 0, 2, 0, 0],
#     [3, 4, 0, 2, 0, 0, 5, 0, 6],
#     [6, 0, 0, 7, 0, 0, 0, 8, 9],
#     [0, 0, 5, 0, 0, 8, 0, 3, 0],
#     [2, 0, 0, 0, 6, 0, 0, 0, 8],
#     [0, 1, 0, 9, 0, 0, 7, 0, 0],
#     [1, 9, 0, 0, 0, 3, 0, 0, 4],
#     [4, 0, 8, 0, 0, 7, 0, 5, 1],
#     [0, 0, 6, 0, 2, 0, 0, 0, 0],
# ]

def isValid(x, y, n):
    for i in range(9):
        if board[i][x] == n:
            return False
    
    for i in range(9):
        if board[y][i] == n:
            return False
    
    x0 = (x//3)*3
    y0 = (y//3)*3
    for i in range(3):
        for j in range(3):
            if board[y0+i][x0+j] == n:
                return False

    return True

def printBoard():
    print("-------------------")
    for i in range(9):
        print(*board[i])
    print("-------------------")

def solve():
    for y in range(9):
        for x in range(9):
            if board[y][x] == 0:
                for n in range(1,10):
                    if isValid(x, y, n):
                        board[y][x] = n
                        solve()
                        board[y][x] = 0
                return
    printBoard()

solve()
