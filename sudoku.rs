fn is_valid(board: &[[usize;9];9], i: usize, j: usize, n: usize) -> bool {
    let (i0, j0) = ( (i / 3) * 3, (j / 3) * 3 );
    let block_check = (j0..j0+3).all(|index_j| (i0..i0+3).all(|index_i| board[index_j][index_i] != n) );
    let rowcol_check = (0..9).all(|k| (board[j][k] != n) && (board[k][i] != n));
    rowcol_check && block_check
}

fn print_board(board: &[[usize;9];9]) {
    for row in board.into_iter() {
        println!("{:?}", row);
    }
    println!("---------------------------");
}

fn solve(board: &mut [[usize;9];9]) {
    for j in 0..9 {
        for i in 0..9 {
            if board[j][i] == 0 {
                for n in 1..10 {
                    if is_valid(board, i, j, n) {
                        board[j][i] = n;
                        solve(board);
                        board[j][i] = 0;
                    }
                }
                return;
            }
        }
    }
    print_board(board)
}


fn main() {
    let mut board : [[usize;9];9] = [
        [5,3,0, 0,7,0, 0,0,0],
        [6,0,0, 1,9,5, 0,0,0],
        [0,9,8, 0,0,0, 0,6,0],

        [8,0,0, 0,6,0, 0,0,3],
        [4,0,0, 8,0,3, 0,0,1],
        [7,0,0, 0,2,0, 0,0,6],

        [0,6,0, 0,0,0, 2,8,0],
        [0,0,0, 4,1,9, 0,0,5],
        [0,0,0, 0,8,0, 0,7,9]
    ];
    solve(&mut board);
}
