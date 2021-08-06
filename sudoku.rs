
static mut BOARD: [[i32; 9]; 9] = [
    [5, 3, 0, 0, 7, 0, 0, 0, 0],
    [6, 0, 0, 1, 9, 5, 0, 0, 0],
    [0, 9, 8, 0, 0, 0, 0, 6, 0],
    [8, 0, 0, 0, 6, 0, 0, 0, 3],
    [4, 0, 0, 8, 0, 3, 0, 0, 1],
    [7, 0, 0, 0, 2, 0, 0, 0, 6],
    [0, 6, 0, 0, 0, 0, 2, 8, 0],
    [0, 0, 0, 4, 1, 9, 0, 0, 5],
    [0, 0, 0, 0, 8, 0, 0, 7, 9],
];

fn is_valid(x:usize, y:usize, n:i32) -> bool {
    unsafe {
        for i in 0..9 {
            if BOARD[i][x] == n {
                return false;
            }
        }
        for i in 0..9 {
            if BOARD[y][i] == n {
                return false;
            }
        }
        let x0 = x / 3 *3;
        let y0 = y / 3 *3;
        for i in 0..3 {
            for j in 0..3 {
                if BOARD[y0+i][x0+j] == n {
                    return false;
                }
            }
        }
    }
    return true;
}

fn print_board() {
    unsafe{
        for i in 0..9 {
            println!("{:?}",BOARD[i]);
        }
    }
}

fn solve() {
    unsafe {
        for y in 0..9 {
            for x in 0..9 {
                if BOARD[y][x] == 0 {
                    for n in 1..10 {
                        if is_valid(x, y, n) {
                            BOARD[y][x] = n;
                            solve();
                            BOARD[y][x] = 0; 
                        }
                    }
                    return;
                }
            }
        }
        //print_board();
    }
}

fn main() {
    solve();
}