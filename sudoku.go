package main

import "fmt"

var board = [9][9]int {
    {5, 3, 0, 0, 7, 0, 0, 0, 0},
    {6, 0, 0, 1, 9, 5, 0, 0, 0},
    {0, 9, 8, 0, 0, 0, 0, 6, 0},
    {8, 0, 0, 0, 6, 0, 0, 0, 3},
    {4, 0, 0, 8, 0, 3, 0, 0, 1},
    {7, 0, 0, 0, 2, 0, 0, 0, 6},
    {0, 6, 0, 0, 0, 0, 2, 8, 0},
    {0, 0, 0, 4, 1, 9, 0, 0, 5},
    {0, 0, 0, 0, 8, 0, 0, 7, 9},
}

func isValid(x int, y int, n int) bool {
	for i:=0; i<9; i++{
		if board[i][x] == n {
			return false
		}
	}
	for i:=0; i<9; i++{
		if board[y][i] == n {
			return false
		}
	}
	x0 := x/3*3
	y0 := y/3*3

	for i:=0; i < 3; i++ {
		for j:=0; j < 3; j++ {
			if board[y0+i][x0+j] == n {
				return false
			}
		}
	}
	return true
}

func printBoard(){
	for i:=0; i<9; i++{
		fmt.Println(board[i])
	}
}

func solve(){
	for y:=0; y<9; y++ {
		for x:=0; x<9; x++{
			if board[y][x]==0 {
				for n:=1; n<10; n++ {
					if isValid(x, y, n) {
						board[y][x] = n
						solve()
						board[y][x] = 0
					}
				}
				return
			}
		}
	}
	//printBoard()
}


func main() {
    solve()
}