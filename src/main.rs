use std::collections::HashSet;

type Sudoku = [[i32; 9]; 9];

fn main() {
    println!("Hello, world!");
    let sud = [
        [2, 9, 0, 0, 0, 0, 0, 8, 7],
        [0, 0, 0, 0, 8, 0, 0, 0, 0],
        [0, 0, 5, 2, 7, 0, 0, 4, 1],
        [0, 0, 0, 9, 0, 0, 1, 0, 6],
        [0, 0, 1, 0, 0, 0, 9, 0, 0],
        [9, 0, 4, 0, 0, 6, 0, 0, 0],
        [7, 6, 0, 0, 3, 8, 4, 0, 0],
        [0, 0, 0, 0, 9, 0, 0, 0, 0],
        [3, 1, 0, 0, 0, 0, 0, 9, 8],
    ];

    if let Some(result) = iteration(&sud) {
        println!("result was a winner {:?}", result);
    } else {
        println!("oliver broke my program again");
    }
}

fn iteration(sud: &Sudoku) -> Option<Sudoku> {
    // find the cell with the lowest variability
    let lowest_variability_option = sud
        .as_flattened()
        .into_iter()
        .enumerate()
        .filter(|(_, cell_value)| **cell_value == 0)
        .map(|(idx, _)| {
            (
                (idx % 9, idx / 9),
                legal_digits_for_cell(&sud, (idx % 9) as i32, (idx / 9) as i32),
            )
        })
        .min_by_key(|(_, legals)| legals.len());

    let Some((cell_to_try, cell_to_try_values)) = lowest_variability_option else {
        return Some(*sud);
    };

    let mut trial_sud = sud.clone();
    for ele in cell_to_try_values.into_iter() {
        set_cell_at(
            &mut trial_sud,
            cell_to_try.0 as i32,
            cell_to_try.1 as i32,
            ele,
        );
        if let Some(result) = iteration(&trial_sud) {
            return Some(result);
        }
    }
    return None;
}

fn legal_digits_for_cell(sudoku: &Sudoku, x: i32, y: i32) -> Vec<i32> {
    let mut relevant = get_col(sudoku, x)
        .into_iter()
        .chain(get_row(sudoku, y))
        .chain(get_box_for_cell(sudoku, x, y))
        .filter(|num| *num != 0)
        .collect::<Vec<_>>();

    relevant.sort();
    relevant.dedup();

    return legal_digits(&relevant);
}

fn legal_digits(slice: &[i32]) -> Vec<i32> {
    [1, 2, 3, 4, 5, 6, 7, 8, 9]
        .into_iter()
        .collect::<HashSet<i32>>()
        .symmetric_difference(&slice.iter().cloned().collect::<HashSet<i32>>())
        .cloned()
        .collect::<Vec<i32>>()
}

fn get_row(sudoku: &Sudoku, row: i32) -> [i32; 9] {
    return sudoku[row as usize];
}

fn get_col(sudoku: &Sudoku, col: i32) -> [i32; 9] {
    let col_idx = col as usize;
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

fn set_cell_at(sudoku: &mut Sudoku, x: i32, y: i32, val: i32) {
    sudoku[y as usize][x as usize] = val;
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
        if y >= 0 && y < 3 {
            box_idx = 0;
        }
        if y >= 3 && y < 6 {
            box_idx = 3;
        }
        if y >= 6 && y < 9 {
            box_idx = 6;
        }
    }
    if x >= 3 && x < 6 {
        if y >= 0 && y < 3 {
            box_idx = 1;
        }
        if y >= 3 && y < 6 {
            box_idx = 4;
        }
        if y >= 6 && y < 9 {
            box_idx = 7;
        }
    }
    if x >= 6 && x < 9 {
        if y >= 0 && y < 3 {
            box_idx = 2;
        }
        if y >= 3 && y < 6 {
            box_idx = 5;
        }
        if y >= 6 && y < 9 {
            box_idx = 8;
        }
    }
    return get_box_cells(sudoku, box_idx);
}
