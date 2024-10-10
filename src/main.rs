type Sudoku = [[i32; 9]; 9];



fn main() {
    println!("Hello, world!");
    let foo = legal_digits(&[1, 2, 3]);
    println!("{:?}", foo);
    println!(
        "{:?}",
        get_box_cells(
            &[
                [2, 9, 0, 0, 0, 0, 0, 7, 8],
                [0, 0, 0, 0, 8, 0, 0, 0, 0],
                [0, 0, 5, 2, 7, 0, 0, 4, 1],
                [0, 0, 0, 9, 0, 0, 1, 0, 6],
                [0, 0, 1, 0, 0, 0, 9, 0, 0],
                [9, 0, 4, 0, 0, 6, 0, 0, 0],
                [7, 6, 0, 0, 3, 8, 4, 0, 0],
                [0, 0, 0, 0, 9, 0, 0, 0, 0],
                [3, 1, 0, 0, 0, 0, 0, 9, 8],
            ],
            5
        )
    );

    let mut _sud = [
        [2, 9, 0, 0, 0, 0, 0, 7, 8],
        [0, 0, 0, 0, 8, 0, 0, 0, 0],
        [0, 0, 5, 2, 7, 0, 0, 4, 1],
        [0, 0, 0, 9, 0, 0, 1, 0, 6],
        [0, 0, 1, 0, 0, 0, 9, 0, 0],
        [9, 0, 4, 0, 0, 6, 0, 0, 0],
        [7, 6, 0, 0, 3, 8, 4, 0, 0],
        [0, 0, 0, 0, 9, 0, 0, 0, 0],
        [3, 1, 0, 0, 0, 0, 0, 9, 8],
    ];
}

fn legal_digits_for_cell(sudoku: &Sudoku, x: i32, y: i32) {
    let col = get_col(sudoku, x);
    let row = get_row(sudoku, y);
    let box = get_box_for_cell(sudoku, x, y);
}

fn legal_digits(slice: &[i32]) -> Vec<i32> {
    let mut legals = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    legals.retain(|el| !slice.contains(el));
    return legals;
}

fn get_row(sudoku: &Sudoku, row: i32) -> [i32; 9] {
    return sudoku[row as usize];
}

fn get_col(sudoku: &Sudoku, col: i32) -> [i32; 9] {
    let col_idx = col as usize
    return [
        sudoku[0][col_idx],
        sudoku[1][col_idx],
        sudoku[2][col_idx],
        sudoku[3][col_idx],
        sudoku[4][col_idx],
        sudoku[5][col_idx],
        sudoku[6][col_idx],
        sudoku[7][col_idx],
        sudoku[8][col_idx],
    ];
}

fn get_cell_at(sudoku: &Sudoku, x: i32, y: i32) -> i32 {
    return sudoku[y as usize][x as usize];
}

const BOX_OFFSETS: [(i32, i32); 9] = [
    (0, 0),
    (1, 0),
    (2, 0),
    (0, 1),
    (1, 1),
    (2, 1),
    (0, 2),
    (1, 2),
    (2, 2),
];

fn get_box_cells(sudoku: &Sudoku, box_idx: i32) -> Vec<i32> {
    let start_x = (box_idx % 3) * 3;
    let start_y = (box_idx / 3) * 3;
    BOX_OFFSETS
        .iter()
        .map(|(x, y)| get_cell_at(sudoku, start_x + x, start_y + y))
        .collect()
}

fn get_box_for_cell(sudoku: &Sudoku, x: i32, y: i32) -> Vec<i32> {
    let mut box_idx = 0;
    if x >= 0 && x < 3 {
        if (y >= 0 && y < 3) {
            box_idx = 0
        }
        if (y >= 3 && y < 6) {
            box_idx = 3
        }
        if (y >= 6 && y < 9) {
            box_idx = 6
        }
    }
    if (x >= 3 && x < 6) {
        if (y >= 0 && y < 3) {
            box_idx = 1
        }
        if (y >= 3 && y < 6) {
            box_idx = 4
        }
        if (y >= 6 && y < 9) { 
            box_idx = 7
        }
    }
    if (x >= 6 && x < 9) {
        if (y >= 0 && y < 3) {
            box_idx = 2
        }
        if (y >= 3 && y < 6) {
            box_idx = 5
        }
        if (y >= 6 && y < 9) {
            box_idx = 8
        }
    }
    return get_box_cells(sudoku, box_idx)
}
