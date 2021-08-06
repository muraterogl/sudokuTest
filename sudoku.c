#include "stdio.h"
#include <stdbool.h>

int board[9][9] = {
    {5, 3, 0, 0, 7, 0, 0, 0, 0},
    {6, 0, 0, 1, 9, 5, 0, 0, 0},
    {0, 9, 8, 0, 0, 0, 0, 6, 0},
    {8, 0, 0, 0, 6, 0, 0, 0, 3},
    {4, 0, 0, 8, 0, 3, 0, 0, 1},
    {7, 0, 0, 0, 2, 0, 0, 0, 6},
    {0, 6, 0, 0, 0, 0, 2, 8, 0},
    {0, 0, 0, 4, 1, 9, 0, 0, 5},
    {0, 0, 0, 0, 8, 0, 0, 7, 9},
};

bool isValid(int x, int y, int n){
    for (int i = 0; i < 9; i++){
        if (board[i][x] == n) return false;
    }
    for (int i = 0; i < 9; i++){
        if (board[y][i] == n) return false;
    }
    int x0 = x / 3 * 3;
    int y0 = y / 3 * 3;
    for (int i = 0; i < 3; i++){
        for (int j = 0; j < 3; j++){
            if (board[y0+i][x0+j] == n) return false;
        }
    }
    return true;
}

void printBoard(){
    printf("---------------------\n");
    for (int i = 0; i < 9; i++){
        for (int j = 0; j < 9; j++){
            printf(j==8 ? "%d\n" : "%d ", board[i][j]);
        }
    }
    printf("---------------------\n");
}

void solve(){
    for (int y = 0; y < 9; y++){
        for (int x = 0; x < 9; x++){
            if (board[y][x] == 0){
                for (int n = 1; n < 10; n++){
                    if (isValid(x, y, n)){
                        board[y][x] = n;
                        solve();
                        board[y][x] = 0;
                    }
                }
                return;
            }
        }
    }
    //printBoard();
}

int main(){
    solve();
    return 0;
}